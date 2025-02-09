CREATE TABLE IF NOT EXISTS users(
    user_login VARCHAR(32) PRIMARY KEY,
    user_name VARCHAR(64) NOT NULL,
    user_email VARCHAR,
    tg_login VARCHAR,
);