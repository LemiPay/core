use std::env;

#[derive(Clone)]
pub struct DbConfig {
    pub database_url: String,
    pub jwt_secret: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok(); // intenta cargar .env del cwd
        dotenvy::from_filename("../.env").ok(); // fallback

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        Self {
            database_url,
            jwt_secret,
        }
    }
}
