use std::env;

#[derive(Clone)]
pub struct DbConfig {
    pub database_url: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); // intenta cargar .env del cwd
        dotenvy::from_filename("../.env").ok(); // fallback

        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        Self { database_url }
    }
}
