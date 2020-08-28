use crate::models::account::Account;
use crate::models::user_account::UserAccount;
use crate::schema::accounts::dsl as account;
use crate::schema::user_accounts::dsl as user_account;
use crate::schema::users;
use chrono::{DateTime, Utc};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::collections::HashMap;

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

pub struct AccountWithRole {
    pub account: Account,
    pub user_account: UserAccount,
    pub role: String,
}

impl User {
    pub fn user_accounts(
        &self,
        conn: &PgConnection,
    ) -> Result<Vec<AccountWithRole>, diesel::result::Error> {
        let user_accounts = user_account::user_accounts
            .filter(user_account::user_id.eq(self.id))
            .load::<UserAccount>(conn)?;

        let mut account_ids_roles: HashMap<i32, UserAccount> = user_accounts
            .into_iter()
            .map(|x| (x.account_id, x))
            .collect();
        let account_ids = account_ids_roles.keys();

        let accounts = account::accounts
            .filter(account::id.eq_any(account_ids))
            .load::<Account>(conn)?;

        let accounts_with_roles = accounts
            .into_iter()
            .map(|account| {
                let user_account: UserAccount = account_ids_roles
                    .remove(&account.id)
                    .expect("role for account");
                let role = user_account.role.clone();

                AccountWithRole {
                    role: role,
                    user_account: user_account,
                    account: account,
                }
            })
            .collect();
        Ok(accounts_with_roles)
    }

    pub fn full_delete(&self, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        let user_accounts = self.user_accounts(conn)?;

        for user_account in user_accounts.iter() {
            let users = user_account.account.users(conn)?;
            diesel::delete(&user_account.user_account).execute(conn)?;
            if users.len() == 1 {
                diesel::delete(&user_account.account).execute(conn)?;
            }
        }

        diesel::delete(self).execute(conn)?;

        Ok(())
    }
}
