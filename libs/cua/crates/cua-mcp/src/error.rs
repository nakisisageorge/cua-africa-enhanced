//! Error types for cua-mcp

use thiserror::Error;

#[derive(Error, Debug)]
pub enum McpError {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    
    #[error("Cua SDK error: {0}")]
    CuaSdk(#[from] cua_sdk::Error),
    
    #[error("Sandbox error: {0}")]
    Sandbox(String),
    
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Authorization error: {0}")]
    Authz(String),
    
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),
    
    #[error("Session error: {0}")]
    Session(String),
    
    #[error("Regression tracking error: {0}")]
    Regression(String),
    
    #[error("Profile error: {0}")]
    Profile(String),
    
    #[error("Tool execution error: {0}")]
    ToolExecution(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
    
    #[error("TLS error: {0}")]
    Tls(#[from] rustls::Error),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("MCP protocol error: {0}")]
    McpProtocol(String),
    
    #[error("Timeout: {0}")]
    Timeout(String),
}

impl McpError {
    pub fn sandbox(msg: impl Into<String>) -> Self {
        Self::Sandbox(msg.into())
    }
    
    pub fn transport(msg: impl Into<String>) -> Self {
        Self::Transport(msg.into())
    }
    
    pub fn auth(msg: impl Into<String>) -> Self {
        Self::Auth(msg.into())
    }
    
    pub fn authz(msg: impl Into<String>) -> Self {
        Self::Authz(msg.into())
    }
    
    pub fn rate_limit(msg: impl Into<String>) -> Self {
        Self::RateLimit(msg.into())
    }
    
    pub fn session(msg: impl Into<String>) -> Self {
        Self::Session(msg.into())
    }
    
    pub fn regression(msg: impl Into<String>) -> Self {
        Self::Regression(msg.into())
    }
    
    pub fn profile(msg: impl Into<String>) -> Self {
        Self::Profile(msg.into())
    }
    
    pub fn tool(msg: impl Into<String>) -> Self {
        Self::ToolExecution(msg.into())
    }
    
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
    
    pub fn invalid_arg(msg: impl Into<String>) -> Self {
        Self::InvalidArgument(msg.into())
    }
    
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
    
    pub fn mcp(msg: impl Into<String>) -> Self {
        Self::McpProtocol(msg.into())
    }
    
    pub fn timeout(msg: impl Into<String>) -> Self {
        Self::Timeout(msg.into())
    }
}

pub type Result<T> = std::result::Result<T, McpError>;