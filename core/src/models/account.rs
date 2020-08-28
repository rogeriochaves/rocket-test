use crate::models::user::User;
use crate::models::user_account::UserAccount;
use crate::schema::accounts;
use crate::schema::user_accounts::dsl as user_account;
use crate::schema::users::dsl as user;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "accounts"]
pub struct Account {
  pub id: i32,
  pub name: String,
  pub created_at: DateTime<Utc>,
}

impl Account {
  pub fn users(&self, conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
    let user_accounts = user_account::user_accounts
      .filter(user_account::account_id.eq(self.id))
      .load::<UserAccount>(conn)?;

    let user_ids: Vec<i32> = user_accounts.iter().map(|x| x.user_id).collect();

    user::users
      .filter(user::id.eq_any(user_ids))
      .load::<User>(conn)
  }
}
