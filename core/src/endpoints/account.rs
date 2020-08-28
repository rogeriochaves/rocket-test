extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use crate::models::user::User;
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
    let rows_inserted: Vec<i32> = diesel::insert_into(user::users)
        .values(&new_user)
        .returning(user::id)
        .get_results(&*conn)
        .expect("inserts new user");

    let id_inserted = rows_inserted.get(0).expect("get the id of inserted user");

    status::Created(
        String::from(""),
        Some(Json(AccountCreateResponse {
            id: *id_inserted,
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

    #[test]
    fn creates_new_accounts() {
        let conn = pg_pool::establish_connection();
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

        diesel::delete(user::users.filter(user::email.eq("foo@bar.baz")))
            .execute(&conn)
            .expect("deletes test user");

        let mut response = req.dispatch();
        assert_eq!(response.status(), Status::Created);
        let result: AccountCreateResponse =
            serde_json::from_str(&response.body_string().expect("body response"))
                .expect("reads the json response");

        let user: User = user::users
            .filter(user::id.eq(result.id))
            .first(&conn)
            .expect("finds created user");
        assert_eq!(user.email, "foo@bar.baz");
        assert_ne!(user.encrypted_password, "hunter2");

        let req = client.get("/account/email_check?email=foo@bar.baz");
        let mut response = req.dispatch();
        let result: EmailCheckResponse =
            serde_json::from_str(&response.body_string().expect("body response"))
                .expect("reads the json response");

        assert_eq!(result.exists, true);
    }
}
