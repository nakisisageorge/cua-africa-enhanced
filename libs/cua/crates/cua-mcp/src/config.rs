//! Configuration for the cua-mcp server

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use config::{Config, File, Environment};
use figment::{Figment, providers::{Toml, Env}};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// Server configuration
    pub server: ServerConfig,
    /// Transport configuration
    pub transport: TransportConfig,
    /// Cua SDK configuration
    pub cua: CuaConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Session configuration
    pub session: SessionConfig,
    /// Regression tracking configuration
    pub regression: RegressionConfig,
    /// Sandbox profiles configuration
    pub profiles: ProfilesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server name for MCP identification
    pub name: String,
    /// Server version
    pub version: String,
    /// Maximum concurrent sessions
    pub max_sessions: usize,
    /// Request timeout in seconds
    pub request_timeout_secs: u64,
    /// Enable debug logging
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportConfig {
    /// Enable stdio transport (for local agents like Claude Code)
    pub stdio: bool,
    /// Enable HTTP transport
    pub http: bool,
    /// HTTP server bind address
    pub http_addr: String,
    /// HTTP server port
    pub http_port: u16,
    /// Enable SSE transport
    pub sse: bool,
    /// SSE endpoint path
    pub sse_path: String,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuaConfig {
    /// Default sandbox image
    pub default_image: String,
    /// Default sandbox kind (container|vm)
    pub default_kind: String,
    /// Default sandbox runtime (gvisor|runc|qemu|lume)
    pub default_runtime: String,
    /// Default CPU allocation
    pub default_cpu: String,
    /// Default memory allocation
    pub default_memory: String,
    /// Sandbox home directory
    pub sandbox_home: Option<PathBuf>,
    /// Enable cua-spacesd in sandboxes
    pub enable_spacesd: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Require API key authentication
    pub require_api_key: bool,
    /// API keys (key -> permissions)
    pub api_keys: std::collections::HashMap<String, Vec<String>>,
    /// Enable mTLS for HTTP transport
    pub enable_mtls: bool,
    /// Certificate file path
    pub cert_file: Option<PathBuf>,
    /// Key file path
    pub key_file: Option<PathBuf>,
    /// CA certificate file path
    pub ca_file: Option<PathBuf>,
    /// Allowed networks for sandbox egress
    pub allowed_networks: Vec<String>,
    /// Default sandbox resource limits
    pub default_limits: ResourceLimits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_cpu: String,
    pub max_memory: String,
    pub max_disk: String,
    pub max_processes: u32,
    pub max_open_files: u32,
    pub network_enabled: bool,
    pub read_only_rootfs: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session TTL in minutes
    pub ttl_minutes: u64,
    /// Maximum sessions per API key
    pub max_per_key: usize,
    /// Enable session persistence
    pub persist: bool,
    /// Session store path
    pub store_path: Option<PathBuf>,
    /// Rate limit: requests per minute
    pub rate_limit_rpm: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionConfig {
    /// Enable regression tracking
    pub enabled: bool,
    /// Database path
    pub db_path: PathBuf,
    /// Regression threshold (percentage)
    pub threshold_percent: f64,
    /// Metrics to track
    pub metrics: Vec<String>,
    /// Baseline branch
    pub baseline_branch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilesConfig {
    /// Profiles directory
    pub dir: PathBuf,
    /// Auto-discover profiles
    pub auto_discover: bool,
    /// Default profile
    pub default: Option<String>,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            transport: TransportConfig::default(),
            cua: CuaConfig::default(),
            security: SecurityConfig::default(),
            session: SessionConfig::default(),
            regression: RegressionConfig::default(),
            profiles: ProfilesConfig::default(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "cua-mcp".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            max_sessions: 100,
            request_timeout_secs: 300,
            debug: false,
        }
    }
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            stdio: true,
            http: false,
            http_addr: "127.0.0.1".to_string(),
            http_port: 3212,
            sse: false,
            sse_path: "/sse".to_string(),
            cors_origins: vec!["*".to_string()],
        }
    }
}

impl Default for CuaConfig {
    fn default() -> Self {
        Self {
            default_image: "ghcr.io/trycua/linux:latest".to_string(),
            default_kind: "container".to_string(),
            default_runtime: "gvisor".to_string(),
            default_cpu: "2".to_string(),
            default_memory: "4G".to_string(),
            sandbox_home: None,
            enable_spacesd: true,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            require_api_key: false,
            api_keys: std::collections::HashMap::new(),
            enable_mtls: false,
            cert_file: None,
            key_file: None,
            ca_file: None,
            allowed_networks: vec![
                "github.com".to_string(),
                "crates.io".to_string(),
                "api.safaricom.co.ke".to_string(),
                "sandbox.momodeveloper.mtn.com".to_string(),
                "openapi.airtel.africa".to_string(),
                "api.orange.com".to_string(),
                "api.wave.com".to_string(),
            ],
            default_limits: ResourceLimits::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_cpu: "4".to_string(),
            max_memory: "8G".to_string(),
            max_disk: "50G".to_string(),
            max_processes: 100,
            max_open_files: 1024,
            network_enabled: true,
            read_only_rootfs: false,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            ttl_minutes: 60,
            max_per_key: 10,
            persist: true,
            store_path: None,
            rate_limit_rpm: 60,
        }
    }
}

impl Default for RegressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            db_path: PathBuf::from(".cua/mcp/regression.db"),
            threshold_percent: 5.0,
            metrics: vec![
                "success_rate".to_string(),
                "steps_to_completion".to_string(),
                "token_usage".to_string(),
                "latency_ms".to_string(),
                "screenshot_similarity".to_string(),
            ],
            baseline_branch: "main".to_string(),
        }
    }
}

impl Default for ProfilesConfig {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("sandbox-profiles/africa"),
            auto_discover: true,
            default: Some("mpesa-daraja".to_string()),
        }
    }
}

impl McpConfig {
    /// Load configuration from file and environment
    pub fn load() -> Result<Self> {
        let figment = Figment::new()
            .merge(Toml::file("cua-mcp.toml"))
            .merge(Env::prefixed("CUA_MCP_").global())
            .merge(Config::default());
        
        Ok(figment.extract()?)
    }
    
    /// Load with custom config file
    pub fn load_from(path: &PathBuf) -> Result<Self> {
        let figment = Figment::new()
            .merge(Toml::file(path))
            .merge(Env::prefixed("CUA_MCP_").global());
        
        Ok(figment.extract()?)
    }
}

/// CLI arguments
#[derive(Parser, Debug)]
#[command(name = "cua-mcp", version, about = "Model Context Protocol server for Cua computer-use orchestration")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Config file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
    
    /// Enable debug logging
    #[arg(short, long, global = true)]
    pub debug: bool,
    
    /// Transport type
    #[arg(short, long, global = true, value_enum, default_value = "stdio")]
    pub transport: TransportType,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the MCP server
    Serve {
        /// Override stdio transport
        #[arg(long)]
        stdio: bool,
        /// Override HTTP transport
        #[arg(long)]
        http: bool,
        /// Override SSE transport
        #[arg(long)]
        sse: bool,
    },
    /// List available sandbox profiles
    Profiles {
        /// Show detailed profile info
        #[arg(short, long)]
        detail: bool,
    },
    /// Run benchmark regression check
    Regression {
        /// Baseline branch/commit
        #[arg(long, default_value = "main")]
        baseline: String,
        /// Current branch/commit
        #[arg(long, default_value = "HEAD")]
        current: String,
        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },
    /// Validate configuration
    Validate,
    /// Generate example configuration
    GenConfig {
        /// Output path
        #[arg(short, long, default_value = "cua-mcp.toml")]
        output: PathBuf,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum TransportType {
    Stdio,
    Http,
    Sse,
    Both,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}