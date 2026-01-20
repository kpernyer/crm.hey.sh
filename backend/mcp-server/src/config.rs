//! Configuration for MCP server

/// MCP Server configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// SurrealDB connection URL
    pub db_url: String,
    /// Database namespace
    pub db_namespace: String,
    /// Database name
    pub db_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: "ws://localhost:8000".into(),
            db_namespace: "crm".into(),
            db_name: "main".into(),
        }
    }
}
