extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use crate::models::user::User;
use crate::schema::users::dsl::*;
use crate::utils::pg_pool::DbConn;
use diesel::prelude::*;
use rocket_contrib::json::Json;
// use crate::data::schema;
// use crate::utils::responders::*;
// use rocket::response::content;

#[get("/account")]
pub fn get_users(conn: DbConn) -> QueryResult<Json<Vec<User>>> {
    users.load::<User>(&*conn).map(Json)
    // .map(Cors)
}
