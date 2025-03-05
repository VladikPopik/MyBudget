CREATE TABLE IF NOT EXISTS users(
    user_login VARCHAR(32) PRIMARY KEY,
    user_name VARCHAR(64) NOT NULL,
    user_password VARCHAR(64) NOT NULL,
    user_email VARCHAR,
    tg_login VARCHAR);

CREATE TABLE IF NOT EXISTS budgets(
    budget_id VARCHAR(36) PRIMARY KEY,
    budget_name TEXT,
    budget_type VARCHAR(16) NOT NULL,
    user_login VARCHAR(32),
    ts_start INTEGER NOT NULL,
    duration INTEGER NOT NULL,
    budget_limit REAL NOT NULL,
    budget_pred REAL,
    FOREIGN KEY (user_login) REFERENCES users(user_login));