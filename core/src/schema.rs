table! {
    accounts (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    user_accounts (user_id, account_id) {
        user_id -> Int4,
        account_id -> Int4,
        role -> Varchar,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        encrypted_password -> Varchar,
        reset_password_token -> Nullable<Varchar>,
        reset_password_token_sent_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
        created_at -> Timestamptz,
    }
}

joinable!(user_accounts -> accounts (account_id));
joinable!(user_accounts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    accounts,
    user_accounts,
    users,
);
