//! cua-mcp: Model Context Protocol server for Cua computer-use orchestration
//!
//! This binary provides an MCP server that exposes Cua's computer-use capabilities
//! to AI agents via the Model Context Protocol.

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use cua_mcp::{config::{Cli, Commands, TransportType}, error::McpError, run_server, McpConfig};

#[tokio::main]
async fn main() -> Result<(), McpError> {
    let cli = Cli::parse();
    
    // Initialize tracing
    let filter = if cli.debug {
        "cua_mcp=debug,rmcp=debug,tower_http=debug"
    } else {
        "cua_mcp=info,rmcp=info"
    };
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(filter))
        .with(tracing_subscriber::fmt::layer()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
        )
        .init();
    
    // Load configuration
    let config = if let Some(config_path) = &cli.config {
        McpConfig::load_from(config_path)?
    } else {
        McpConfig::load().unwrap_or_default()
    };
    
    // Override transport from CLI
    let mut config = config;
    match cli.transport {
        TransportType::Stdio => {
            config.transport.stdio = true;
            config.transport.http = false;
            config.transport.sse = false;
        }
        TransportType::Http => {
            config.transport.stdio = false;
            config.transport.http = true;
            config.transport.sse = false;
        }
        TransportType::Sse => {
            config.transport.stdio = false;
            config.transport.http = false;
            config.transport.sse = true;
        }
        TransportType::Both => {
            config.transport.stdio = true;
            config.transport.http = true;
            config.transport.sse = true;
        }
    }
    
    // Execute command
    match cli.command {
        Commands::Serve { stdio, http, sse } => {
            if stdio { config.transport.stdio = true; }
            if http { config.transport.http = true; }
            if sse { config.transport.sse = true; }
            
            tracing::info!("Starting cua-mcp server");
            tracing::info!("Transport: stdio={}, http={}, sse={}", 
                config.transport.stdio, config.transport.http, config.transport.sse);
            
            run_server(config).await?;
        }
        Commands::Profiles { detail } => {
            list_profiles(&config, detail).await?;
        }
        Commands::Regression { baseline, current, format } => {
            check_regression(&config, &baseline, &current, format).await?;
        }
        Commands::Validate => {
            validate_config(&config).await?;
        }
        Commands::GenConfig { output } => {
            generate_config(&output).await?;
        }
    }
    
    Ok(())
}

async fn list_profiles(config: &McpConfig, detail: bool) -> Result<(), McpError> {
    use std::fs;
    
    let profiles_dir = &config.profiles.dir;
    if !profiles_dir.exists() {
        println!("Profiles directory not found: {}", profiles_dir.display());
        return Ok(());
    }
    
    let mut entries = fs::read_dir(profiles_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "yaml"))
        .collect::<Vec<_>>();
    
    entries.sort_by_key(|e| e.file_name());
    
    println!("Available sandbox profiles ({}):", entries.len());
    println!();
    
    for entry in entries {
        let name = entry.file_name().to_string_lossy().replace(".yaml", "");
        
        if detail {
            // Try to load and display profile details
            println!("📦 {}", name);
            println!("   Path: {}", entry.path().display());
            
            if let Ok(content) = fs::read_to_string(entry.path()) {
                // Parse YAML for key fields
                if let Ok(profile) = serde_yaml::from_str::<serde_json::Value>(&content) {
                    if let Some(display_name) = profile.get("display_name").and_then(|v| v.as_str()) {
                        println!("   Display: {}", display_name);
                    }
                    if let Some(category) = profile.get("category").and_then(|v| v.as_str()) {
                        println!("   Category: {}", category);
                    }
                    if let Some(region) = profile.get("region").and_then(|v| v.as_str()) {
                        println!("   Region: {}", region);
                    }
                    if let Some(language) = profile.get("language").and_then(|v| v.as_str()) {
                        println!("   Language: {}", language);
                    }
                    if let Some(currency) = profile.get("currency").and_then(|v| v.as_str()) {
                        println!("   Currency: {}", currency);
                    }
                    if let Some(provider) = profile.get("provider").and_then(|v| v.as_str()) {
                        println!("   Provider: {}", provider);
                    }
                }
            }
            println!();
        } else {
            println!("  {}", name);
        }
    }
    
    Ok(())
}

async fn check_regression(config: &McpConfig, baseline: &str, current: &str, format: crate::config::OutputFormat) -> Result<(), McpError> {
    use cua_mcp::regression::RegressionTracker;
    use std::sync::Arc;
    
    let tracker = RegressionTracker::new(Arc::new(config.clone())).await?;
    
    // Get all tasks that have runs
    let summary = tracker.get_regression_summary("cua-bench-basic", "cua-agent").await?;
    
    match format {
        crate::config::OutputFormat::Table => {
            println!("Regression Check: {} vs {}", baseline, current);
            println!("========================================");
            println!("Task: {}", summary.task);
            println!("Agent: {}", summary.agent);
            println!("Total Runs: {}", summary.total_runs);
            println!("Regressions Detected: {}", summary.regressions_detected);
            
            if let Some(latest) = summary.latest_run {
                println!("\nLatest Run ({}):", latest.id);
                println!("  Status: {:?}", latest.status);
                println!("  Metrics:");
                for (metric, value) in &latest.metrics {
                    println!("    {}: {:.4}", metric, value);
                }
                if latest.regression_detected {
                    println!("  ⚠️  REGRESSION DETECTED!");
                    for detail in &latest.regression_details {
                        println!("    {}: {:.2}% change (threshold: {:.1}%) - {:?}", 
                            detail.metric, detail.change_percent, detail.threshold_percent, detail.severity);
                    }
                } else {
                    println!("  ✅ No regressions detected");
                }
            }
        }
        crate::config::OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        crate::config::OutputFormat::Csv => {
            println!("task,agent,total_runs,regressions_detected");
            println!("{},{},{},{}", summary.task, summary.agent, summary.total_runs, summary.regressions_detected);
        }
    }
    
    Ok(())
}

async fn validate_config(config: &McpConfig) -> Result<(), McpError> {
    println!("Validating configuration...");
    
    // Check profiles directory
    if !config.profiles.dir.exists() {
        println!("⚠️  Profiles directory not found: {}", config.profiles.dir.display());
    } else {
        println!("✅ Profiles directory exists: {}", config.profiles.dir.display());
    }
    
    // Check database path
    if let Some(parent) = config.regression.db_path.parent() {
        if !parent.exists() {
            println!("⚠️  Regression database directory doesn't exist: {}", parent.display());
        } else {
            println!("✅ Regression database directory exists: {}", parent.display());
        }
    }
    
    // Check TLS config if enabled
    if config.security.enable_mtls {
        if config.security.cert_file.is_none() || config.security.key_file.is_none() {
            println!("⚠️  mTLS enabled but cert/key files not configured");
        } else {
            println!("✅ mTLS certificates configured");
        }
    }
    
    println!("✅ Configuration validation complete");
    Ok(())
}

async fn generate_config(output: &std::path::Path) -> Result<(), McpError> {
    let config = McpConfig::default();
    let toml = toml::to_string_pretty(&config)?;
    tokio::fs::write(output, toml).await?;
    println!("Generated example configuration: {}", output.display());
    Ok(())
}