extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use crate::schema::users::dsl::*;
use crate::utils::pg_pool::DbConn;
use diesel::prelude::*;
use rocket::response::status;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize)]
pub struct AccountCreateResponse {
    id: i32,
    email: String,
}

#[post("/account/create")]
pub fn account_create(conn: DbConn) -> status::Created<Json<AccountCreateResponse>> {
    let new_user = (email.eq("foo@bar.baz"), encrypted_password.eq("123"));
    let rows_inserted: Vec<i32> = diesel::insert_into(users)
        .values(&new_user)
        .returning(id)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user::User;
    use crate::rocket_server;
    use crate::utils::pg_pool;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use serde_json;

    #[test]
    fn creates_new_accounts() {
        let conn = pg_pool::establish_connection();
        let client = Client::new(rocket_server()).expect("valid rocket instance");
        let req = client
            .post("/account/create")
            .body(
                r#"
                {
                    "productName": "foo",
                    "email": "foo@bar.baz",
                    "password": "123",
                    "passwordConfirmation": "123",
                }
            "#,
            )
            .header(ContentType::JSON);

        diesel::delete(users.filter(email.eq("foo@bar.baz")))
            .execute(&conn)
            .expect("deletes test user");

        let mut response = req.dispatch();
        assert_eq!(response.status(), Status::Created);
        let result: AccountCreateResponse =
            serde_json::from_str(&response.body_string().expect("body response"))
                .expect("reads the json response");

        let user: User = users
            .filter(id.eq(result.id))
            .first(&conn)
            .expect("finds created user");
        assert_eq!(user.email, "foo@bar.baz");
        assert_eq!(user.encrypted_password, "123");
    }
}
