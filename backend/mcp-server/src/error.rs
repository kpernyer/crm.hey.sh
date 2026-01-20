//! Error types for MCP server

use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("IO error: {0}")]
    Io(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl McpError {
    /// Convert to JSON-RPC error code
    pub fn error_code(&self) -> i32 {
        match self {
            McpError::InvalidRequest(_) => -32600,
            McpError::ToolNotFound(_) => -32601,
            McpError::InvalidParams(_) => -32602,
            McpError::Internal(_) => -32603,
            McpError::Json(_) => -32700,
            _ => -32000, // Server error
        }
    }
}
