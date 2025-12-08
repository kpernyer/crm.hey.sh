use std::env;

pub struct Config {
    pub database_url: String,
    pub database_namespace: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "ws://localhost:8000".into()),
            database_namespace: env::var("DATABASE_NAMESPACE").unwrap_or_else(|_| "crm".into()),
            database_name: env::var("DATABASE_NAME").unwrap_or_else(|_| "main".into()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "development-secret-change-in-production".into()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .unwrap_or(8080),
        }
    }
}
