CREATE TABLE user_accounts (
  user_id integer,
  account_id integer,
  role VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(user_id) REFERENCES users(id),
  FOREIGN KEY(account_id) REFERENCES accounts(id),
  PRIMARY KEY(user_id, account_id)
)