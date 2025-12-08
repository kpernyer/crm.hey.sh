use anyhow::Result;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use std::env;

pub struct Database {
    pub client: Surreal<Client>,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let url = env::var("SURREALDB_URL").unwrap_or_else(|_| "localhost:8000".into());
        let namespace = env::var("SURREALDB_NAMESPACE").unwrap_or_else(|_| "crm".into());
        let database = env::var("SURREALDB_DATABASE").unwrap_or_else(|_| "main".into());
        let username = env::var("SURREALDB_USER").unwrap_or_else(|_| "root".into());
        let password = env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".into());

        let client = Surreal::new::<Ws>(&url).await?;

        client
            .signin(Root {
                username: &username,
                password: &password,
            })
            .await?;

        client.use_ns(&namespace).use_db(&database).await?;

        Ok(Self { client })
    }

    pub async fn init_schema(&self) -> Result<()> {
        let schema = include_str!("../schema/init.surql");
        self.client.query(schema).await?;
        tracing::info!("Database schema initialized");
        Ok(())
    }
}
