CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  encrypted_password VARCHAR NOT NULL,
  reset_password_token VARCHAR,
  reset_password_token_sent_at TIMESTAMPTZ,
  updated_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
)