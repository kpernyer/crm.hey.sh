//! CRM.HEY.SH MCP Server
//!
//! Model Context Protocol server enabling LLM integration with the CRM.
//! Supports stdio transport for Claude Desktop/Code and HTTP+SSE for web clients.

use clap::Parser;
use std::io::{self, BufRead, Write};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod error;
mod handlers;
mod protocol;
mod tools;

use config::Config;
use error::McpError;
use protocol::{JsonRpcRequest, JsonRpcResponse, McpMessage};

#[derive(Parser, Debug)]
#[command(name = "crm-mcp-server")]
#[command(about = "MCP server for CRM.HEY.SH - enables LLM integration")]
struct Args {
    /// Transport mode: stdio or http
    #[arg(long, default_value = "stdio", env = "MCP_TRANSPORT")]
    transport: String,

    /// HTTP port (only used with http transport)
    #[arg(long, default_value = "3001", env = "MCP_HTTP_PORT")]
    port: u16,

    /// Database URL
    #[arg(long, default_value = "ws://localhost:8000", env = "CRM__DATABASE__URL")]
    db_url: String,

    /// Database namespace
    #[arg(long, default_value = "crm", env = "CRM__DATABASE__NAMESPACE")]
    db_namespace: String,

    /// Database name
    #[arg(long, default_value = "main", env = "CRM__DATABASE__DATABASE")]
    db_name: String,

    /// Log level
    #[arg(long, default_value = "info", env = "RUST_LOG")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<(), McpError> {
    let args = Args::parse();

    // Initialize logging to stderr (stdout is for MCP protocol)
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_writer(io::stderr)
        .with_ansi(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting CRM MCP Server");
    info!("Transport: {}", args.transport);
    info!("Database: {}", args.db_url);

    let config = Config {
        db_url: args.db_url,
        db_namespace: args.db_namespace,
        db_name: args.db_name,
    };

    match args.transport.as_str() {
        "stdio" => run_stdio_transport(config).await,
        "http" => run_http_transport(config, args.port).await,
        _ => {
            warn!("Unknown transport: {}, falling back to stdio", args.transport);
            run_stdio_transport(config).await
        }
    }
}

/// Run MCP server over stdio (for Claude Desktop, Claude Code)
async fn run_stdio_transport(config: Config) -> Result<(), McpError> {
    info!("Running in stdio mode");

    // Initialize database connection
    let db = handlers::init_db(&config).await?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.map_err(|e| McpError::Io(e.to_string()))?;

        if line.trim().is_empty() {
            continue;
        }

        // Parse JSON-RPC request
        let request: JsonRpcRequest = match serde_json::from_str(&line) {
            Ok(req) => req,
            Err(e) => {
                let error_response = JsonRpcResponse::error(
                    None,
                    -32700,
                    format!("Parse error: {}", e),
                );
                writeln!(stdout, "{}", serde_json::to_string(&error_response).unwrap())
                    .map_err(|e| McpError::Io(e.to_string()))?;
                stdout.flush().map_err(|e| McpError::Io(e.to_string()))?;
                continue;
            }
        };

        // Handle the request
        let response = handlers::handle_request(&db, request).await;

        // Write response
        writeln!(stdout, "{}", serde_json::to_string(&response).unwrap())
            .map_err(|e| McpError::Io(e.to_string()))?;
        stdout.flush().map_err(|e| McpError::Io(e.to_string()))?;
    }

    Ok(())
}

/// Run MCP server over HTTP+SSE (for web clients)
async fn run_http_transport(_config: Config, port: u16) -> Result<(), McpError> {
    info!("HTTP transport not yet implemented, port: {}", port);
    // TODO: Implement HTTP+SSE transport using axum
    // This would expose endpoints like:
    // - POST /mcp/messages - for tool calls
    // - GET /mcp/sse - for server-sent events
    Err(McpError::NotImplemented("HTTP transport".into()))
}
