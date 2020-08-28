use crate::schema::user_accounts;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[primary_key(user_id, account_id)]
#[table_name = "user_accounts"]
pub struct UserAccount {
    pub user_id: i32,
    pub account_id: i32,
    pub role: String,
    pub created_at: DateTime<Utc>,
}
