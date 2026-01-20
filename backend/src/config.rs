use config::{Config as ConfigLoader, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub jwt: JwtConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub surrealdb: SurrealDbConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SurrealDbConfig {
    pub url: String,
    pub namespace: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub timeout: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let environment = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let mut config_loader = ConfigLoader::builder()
            // Start with the base configuration
            .add_source(File::with_name("config/base"))
            // Add environment-specific configuration
            .add_source(File::with_name(&format!("config/{}", environment)).required(false))
            // Override with environment variables
            .add_source(
                Environment::with_prefix("CRM")
                    .separator("__")
                    .try_parsing(true)
                    .list_separator(",")
                    .with_list_parse_key("database.surrealdb.tags"),
            )
            // Add secret overrides if available
            .build()?;

        config_loader.try_deserialize()
    }
}
