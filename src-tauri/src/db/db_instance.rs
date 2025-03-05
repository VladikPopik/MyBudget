use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub struct DBInstance {
    _url: String,
    _active: bool,
    pub connection: SqliteConnection,
}

impl Default for DBInstance {
    fn default() -> Self {
        Self::new()
    }
}

impl DBInstance {
    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Self {
            _url: database_url,
            _active: true,
            connection: connection,
        }
    }
}
