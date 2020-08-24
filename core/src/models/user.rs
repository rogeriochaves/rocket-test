use crate::schema::users;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub encrypted_password: String,
    pub reset_password_token: Option<String>,
    pub reset_password_token_sent_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
