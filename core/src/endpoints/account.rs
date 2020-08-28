extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use crate::models::user::User;
use crate::schema::accounts::dsl as account;
use crate::schema::user_accounts::dsl as user_account;
use crate::schema::users::dsl as user;
use crate::utils::pg_pool::DbConn;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use rocket::response::status;
use rocket_contrib::json::Json;

#[derive(Deserialize)]
pub struct AccountCreateParams {
    product_name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountCreateResponse {
    id: i32,
    email: String,
}

#[post("/account/create", data = "<params>")]
pub fn account_create(
    conn: DbConn,
    params: Json<AccountCreateParams>,
) -> status::Created<Json<AccountCreateResponse>> {
    let encrypted_password = hash(&params.password, DEFAULT_COST).expect("could not hash password");
    let new_user = (
        user::email.eq(&params.email),
        user::encrypted_password.eq(encrypted_password),
    );
    let user_id: i32 = diesel::insert_into(user::users)
        .values(&new_user)
        .returning(user::id)
        .get_result(&*conn)
        .expect("inserts new user");

    let new_account = (account::name.eq(&params.product_name),);
    let account_id: i32 = diesel::insert_into(account::accounts)
        .values(&new_account)
        .returning(account::id)
        .get_result(&*conn)
        .expect("inserts new account");

    let new_user_account = (
        user_account::user_id.eq(user_id),
        user_account::account_id.eq(account_id),
        user_account::role.eq("admin"),
    );
    diesel::insert_into(user_account::user_accounts)
        .values(&new_user_account)
        .execute(&*conn)
        .expect("inserts new user account");

    status::Created(
        String::from(""),
        Some(Json(AccountCreateResponse {
            id: user_id,
            email: String::from("foo"),
        })),
    )
}

#[derive(Serialize, Deserialize)]
pub struct EmailCheckResponse {
    exists: bool,
}

#[get("/account/email_check?<email>")]
pub fn account_email_check(conn: DbConn, email: String) -> Json<EmailCheckResponse> {
    let user: Result<User, _> = user::users.filter(user::email.eq(email)).first(&*conn);
    Json(EmailCheckResponse {
        exists: user.is_ok(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rocket_server;
    use crate::utils::pg_pool;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use serde_json;

    #[test]
    fn check_email_does_no_exist() {
        let client = Client::new(rocket_server()).expect("valid rocket instance");
        let req = client.get("/account/email_check?email=fulano@da.silva");

        let mut response = req.dispatch();
        let result: EmailCheckResponse =
            serde_json::from_str(&response.body_string().expect("body response"))
                .expect("reads the json response");

        assert_eq!(result.exists, false);
    }

    fn cleanup(conn: &PgConnection) {
        let users = user::users
            .filter(user::email.eq("foo@bar.baz"))
            .load::<User>(conn)
            .expect("deletes test user");
        for user in users.iter() {
            user.full_delete(conn).expect("deletes existing user");
        }
    }

    #[test]
    fn creates_new_accounts() {
        let conn = pg_pool::establish_connection();
        cleanup(&conn);

        let client = Client::new(rocket_server()).expect("valid rocket instance");
        let req = client
            .post("/account/create")
            .body(
                r#"
                {
                    "product_name": "foo",
                    "email": "foo@bar.baz",
                    "password": "hunter2"
                }
            "#,
            )
            .header(ContentType::JSON);

        let mut response = req.dispatch();
        assert_eq!(response.status(), Status::Created);
        let result: AccountCreateResponse =
            serde_json::from_str(&response.body_string().expect("body response"))
                .expect("reads the json response");

        let user: User = user::users
            .find(result.id)
            .get_result(&conn)
            .expect("finds created user");
        assert_eq!(user.email, "foo@bar.baz");
        assert_ne!(user.encrypted_password, "hunter2");

        let user_accounts = user.user_accounts(&conn).expect("find accounts");
        let user_account = user_accounts.get(0).expect("has an account");
        assert_eq!(user_account.account.name, "foo");
        assert_eq!(user_account.role, "admin");

        let req = client.get("/account/email_check?email=foo@bar.baz");
        let mut response = req.dispatch();
        let result: EmailCheckResponse =
            serde_json::from_str(&response.body_string().expect("body response"))
                .expect("reads the json response");

        assert_eq!(result.exists, true);
    }
}
