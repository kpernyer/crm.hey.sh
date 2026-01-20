use anyhow::Result;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use crate::config::Config;

pub struct Database {
    pub client: Surreal<Client>,
}

impl Database {
    pub async fn new(config: &Config) -> Result<Self> {
        let db_config = &config.database.surrealdb;

        let client = Surreal::new::<Http>(&db_config.url).await?;

        client
            .signin(Root {
                username: &db_config.username,
                password: &db_config.password,
            })
            .await?;

        client.use_ns(&db_config.namespace).use_db(&db_config.database).await?;

        Ok(Self { client })
    }

    pub async fn init_schema(&self) -> Result<()> {
        let schema = include_str!("../schema/init.surql");
        self.client.query(schema).await?;
        tracing::info!("Database schema initialized");
        Ok(())
    }
}
