//! cua-mcp: Model Context Protocol server for Cua computer-use orchestration
//!
//! This crate provides an MCP server that exposes Cua's computer-use capabilities
//! to AI agents via the Model Context Protocol. It supports both stdio and HTTP/SSE
//! transports for integration with Claude Code, Codex, and other MCP-compatible agents.

pub mod config;
pub mod error;
pub mod server;
pub mod tools;
pub mod resources;
pub mod prompts;
pub mod regression;
pub mod session;

use std::sync::Arc;

pub use config::McpConfig;
pub use error::McpError;
pub use server::McpServer;
pub use tools::ToolRegistry;
pub use resources::ResourceRegistry;
pub use prompts::PromptRegistry;
pub use regression::RegressionTracker;
pub use session::SessionManager;

/// Re-export commonly used types
pub use cua_sandbox_core::{Sandbox, Image, SandboxOptions};
pub use cua_proto::{sandbox::v1 as sandbox_v1, spacesd::v1 as spacesd_v1};
pub use rmcp::{
    model::{CallToolResult, ListToolsResult, Tool, Resource, Prompt},
    service::RunningService,
    transport::{StdioTransport, HttpTransport, SseTransport},
};

/// Main entry point for the MCP server
pub async fn run_server(config: McpConfig) -> Result<(), McpError> {
    let server = McpServer::new(config).await?;
    server.run().await
}

/// Create a stdio-based MCP server for local agent integration
pub async fn create_stdio_server(config: McpConfig) -> Result<RunningService<StdioTransport>, McpError> {
    let server = McpServer::new(config).await?;
    server.serve_stdio().await
}

/// Create an HTTP/SSE-based MCP server for remote agent orchestration
#[cfg(feature = "http")]
pub async fn create_http_server(config: McpConfig) -> Result<RunningService<HttpTransport>, McpError> {
    let server = McpServer::new(config).await?;
    server.serve_http().await
}