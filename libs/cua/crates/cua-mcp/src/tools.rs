//! Tool definitions and implementations for the cua-mcp server

use rmcp::{
    handler::server::tool::ToolRouter,
    model::{CallToolResult, Tool},
    schemars::schema::SchemaObject,
    schemars::JsonSchema,
    tool, tool_handler, tool_router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{
    config::McpConfig,
    error::{McpError, Result},
    regression::RegressionTracker,
    session::SessionManager,
};

/// Tool registry managing all available MCP tools
pub struct ToolRegistry {
    router: ToolRouter<()>,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
    regression_tracker: Arc<RegressionTracker>,
}

impl ToolRegistry {
    pub fn new(
        config: Arc<McpConfig>,
        session_manager: Arc<SessionManager>,
        regression_tracker: Arc<RegressionTracker>,
    ) -> Self {
        let mut registry = Self {
            router: ToolRouter::new(),
            config,
            session_manager,
            regression_tracker,
        };
        registry.register_tools();
        registry
    }
    
    fn register_tools(&mut self) {
        // Register all tools
        self.router.add_tool(CreateSandboxTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(ExecuteCommandTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(ScreenshotTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(InputTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(ListSkillsTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(InvokeSkillTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(RunBenchmarkTool::new(self.config.clone(), self.session_manager.clone(), self.regression_tracker.clone()));
        self.router.add_tool(CheckRegressionTool::new(self.config.clone(), self.regression_tracker.clone()));
        self.router.add_tool(ListProfilesTool::new(self.config.clone()));
        self.router.add_tool(GetProfileTool::new(self.config.clone()));
        self.router.add_tool(DestroySandboxTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(GetSandboxStatusTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(UploadFileTool::new(self.config.clone(), self.session_manager.clone()));
        self.router.add_tool(DownloadFileTool::new(self.config.clone(), self.session_manager.clone()));
    }
    
    pub fn router(&self) -> &ToolRouter<()> {
        &self.router
    }
    
    pub fn into_router(self) -> ToolRouter<()> {
        self.router
    }
}

/// Tool: Create a new sandbox from a profile
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateSandboxParams {
    /// Profile name (e.g., "mpesa-daraja", "mtn-momo", "nibss")
    pub profile: String,
    /// Optional custom sandbox name
    pub name: Option<String>,
    /// Override default image
    pub image: Option<String>,
    /// Override CPU allocation
    pub cpu: Option<String>,
    /// Override memory allocation
    pub memory: Option<String>,
    /// Override disk allocation
    pub disk: Option<String>,
    /// Enable network access
    pub network_enabled: Option<bool>,
    /// Environment variables to set
    pub env: Option<std::collections::HashMap<String, String>>,
}

#[tool(name = "cua.sandbox.create", description = "Create a new sandbox from an African profile or custom configuration")]
async fn create_sandbox(
    params: CreateSandboxParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    let session_id = session_manager.create_session().await?;
    
    // Load profile configuration
    let profile_path = config.profiles.dir.join(format!("{}.yaml", params.profile));
    if !profile_path.exists() {
        return Err(McpError::profile(format!("Profile not found: {}", params.profile)));
    }
    
    // In a real implementation, this would use cua-sdk to create the sandbox
    // For now, we return a mock response
    let sandbox_id = params.name.unwrap_or_else(|| format!("{}-{}", params.profile, uuid::Uuid::new_v4().to_string()[..8].to_string()));
    
    let result = serde_json::json!({
        "sandbox_id": sandbox_id,
        "session_id": session_id,
        "profile": params.profile,
        "status": "creating",
        "message": "Sandbox creation initiated. Use cua.sandbox.status to check progress.",
        "estimated_ready_seconds": 30
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Execute a command in a sandbox
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ExecuteCommandParams {
    /// Sandbox ID or session ID
    pub sandbox_id: String,
    /// Command to execute
    pub command: String,
    /// Working directory
    pub workdir: Option<String>,
    /// Environment variables
    pub env: Option<std::collections::HashMap<String, String>>,
    /// Timeout in seconds
    pub timeout_secs: Option<u64>,
    /// Run as user
    pub user: Option<String>,
}

#[tool(name = "cua.sandbox.execute", description = "Execute a command in a sandbox")]
async fn execute_command(
    params: ExecuteCommandParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    // Validate session
    let session = session_manager.get_session(&params.sandbox_id).await
        .ok_or_else(|| McpError::not_found(format!("Sandbox not found: {}", params.sandbox_id)))?;
    
    // In real implementation, use cua-sdk to execute command
    let result = serde_json::json!({
        "sandbox_id": params.sandbox_id,
        "command": params.command,
        "exit_code": 0,
        "stdout": "Command executed successfully (mock)",
        "stderr": "",
        "duration_ms": 150
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Take a screenshot of the sandbox
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ScreenshotParams {
    /// Sandbox ID or session ID
    pub sandbox_id: String,
    /// Output format (png, jpeg)
    pub format: Option<String>,
    /// Quality (1-100)
    pub quality: Option<u8>,
    /// Region to capture (x, y, width, height)
    pub region: Option<[u32; 4]>,
}

#[tool(name = "cua.sandbox.screenshot", description = "Capture a screenshot from the sandbox")]
async fn screenshot(
    params: ScreenshotParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    let session = session_manager.get_session(&params.sandbox_id).await
        .ok_or_else(|| McpError::not_found(format!("Sandbox not found: {}", params.sandbox_id)))?;
    
    // In real implementation, use cua-spacesd to capture screenshot
    let result = serde_json::json!({
        "sandbox_id": params.sandbox_id,
        "format": params.format.unwrap_or("png".to_string()),
        "width": 1920,
        "height": 1080,
        "data": "base64_encoded_screenshot_data",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Send input (mouse/keyboard) to sandbox
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InputParams {
    /// Sandbox ID or session ID
    pub sandbox_id: String,
    /// Input type: click, type, key, scroll, drag, move
    pub input_type: String,
    /// X coordinate (for click, move, drag)
    pub x: Option<u32>,
    /// Y coordinate (for click, move, drag)
    pub y: Option<u32>,
    /// Text to type (for type)
    pub text: Option<String>,
    /// Key to press (for key)
    pub key: Option<String>,
    /// Key combination (for hotkey)
    pub hotkey: Option<String>,
    /// Direction (for scroll)
    pub direction: Option<String>,
    /// Amount (for scroll)
    pub amount: Option<i32>,
    /// Drag destination (for drag)
    pub x2: Option<u32>,
    pub y2: Option<u32>,
}

#[tool(name = "cua.sandbox.input", description = "Send mouse/keyboard input to the sandbox")]
async fn input(
    params: InputParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    let session = session_manager.get_session(&params.sandbox_id).await
        .ok_or_else(|| McpError::not_found(format!("Sandbox not found: {}", params.sandbox_id)))?;
    
    // In real implementation, use cua-driver/cua-spacesd to send input
    let result = serde_json::json!({
        "sandbox_id": params.sandbox_id,
        "input_type": params.input_type,
        "success": true,
        "message": "Input sent successfully (mock)"
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: List available skills
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListSkillsParams {
    /// Filter by category
    pub category: Option<String>,
    /// Filter by language
    pub language: Option<String>,
}

#[tool(name = "cua.skill.list", description = "List available computer-use skills")]
async fn list_skills(
    params: ListSkillsParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    // In real implementation, scan skills directory
    let skills = vec![
        serde_json::json!({
            "name": "gui-automation",
            "description": "Visual GUI automation: click, type, screenshot, drag, scroll",
            "category": "automation",
            "languages": ["en"],
            "commands": ["click", "type", "screenshot", "drag", "scroll", "key", "hotkey"]
        }),
        serde_json::json!({
            "name": "jev-use",
            "description": "Jev-Use decision loop for bounded computer-use",
            "category": "decision",
            "languages": ["en"],
            "commands": ["observe", "choose", "act", "verify"]
        }),
        serde_json::json!({
            "name": "african-mobile-money/mpesa-sw",
            "description": "M-Pesa Daraja automation in Swahili",
            "category": "mobile-money",
            "languages": ["sw"],
            "commands": ["stk_push", "balance_check", "transaction_history", "b2c_payment"]
        }),
        serde_json::json!({
            "name": "african-mobile-money/mtn-momo-en",
            "description": "MTN MoMo automation in English",
            "category": "mobile-money",
            "languages": ["en"],
            "commands": ["request_to_pay", "transfer", "kyc", "balance"]
        }),
        serde_json::json!({
            "name": "african-fintech/nibss-en",
            "description": "NIBSS payment automation",
            "category": "fintech",
            "languages": ["en"],
            "commands": ["nip_transfer", "bvn_verify", "account_validate"]
        }),
        serde_json::json!({
            "name": "african-healthcare/openmrs-en",
            "description": "OpenMRS healthcare workflow automation",
            "category": "healthcare",
            "languages": ["en"],
            "commands": ["patient_search", "encounter_create", "observation_record"]
        }),
    ];
    
    let filtered: Vec<_> = skills.into_iter()
        .filter(|s| params.category.as_ref().map_or(true, |c| s["category"] == *c))
        .filter(|s| params.language.as_ref().map_or(true, |l| s["languages"].as_array().unwrap().contains(&serde_json::Value::String(l.clone()))))
        .collect();
    
    let result = serde_json::json!({
        "skills": filtered,
        "total": filtered.len()
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Invoke a skill
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct InvokeSkillParams {
    /// Sandbox ID or session ID
    pub sandbox_id: String,
    /// Skill name (e.g., "african-mobile-money/mpesa-sw")
    pub skill: String,
    /// Skill command
    pub command: String,
    /// Command arguments
    pub args: std::collections::HashMap<String, serde_json::Value>,
}

#[tool(name = "cua.skill.invoke", description = "Invoke a computer-use skill with parameters")]
async fn invoke_skill(
    params: InvokeSkillParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    let session = session_manager.get_session(&params.sandbox_id).await
        .ok_or_else(|| McpError::not_found(format!("Sandbox not found: {}", params.sandbox_id)))?;
    
    // In real implementation, load and execute the skill
    let result = serde_json::json!({
        "sandbox_id": params.sandbox_id,
        "skill": params.skill,
        "command": params.command,
        "args": params.args,
        "success": true,
        "result": "Skill executed successfully (mock)",
        "duration_ms": 2500
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Run a benchmark task
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RunBenchmarkParams {
    /// Task or dataset path/name
    pub task: String,
    /// Agent to use (cua-agent, harness, custom)
    pub agent: Option<String>,
    /// Model to use (for AI agents)
    pub model: Option<String>,
    /// Number of parallel runs
    pub parallel: Option<u32>,
    /// Number of attempts per variant
    pub attempts: Option<u32>,
    /// Sandbox image
    pub image: Option<String>,
    /// Sandbox kind
    pub kind: Option<String>,
    /// Sandbox runtime
    pub runtime: Option<String>,
}

#[tool(name = "cua.benchmark.run", description = "Run a cua-bench task or dataset")]
async fn run_benchmark(
    params: RunBenchmarkParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
    regression_tracker: Arc<RegressionTracker>,
) -> Result<CallToolResult> {
    let run_id = uuid::Uuid::new_v4().to_string();
    
    // In real implementation, use cua-bench to run the benchmark
    let result = serde_json::json!({
        "run_id": run_id,
        "task": params.task,
        "agent": params.agent.unwrap_or("cua-agent".to_string()),
        "model": params.model,
        "status": "running",
        "message": "Benchmark started. Use cua.benchmark.status to check progress.",
        "estimated_duration_seconds": 300
    });
    
    // Track for regression detection
    if config.regression.enabled {
        regression_tracker.record_run_start(&run_id, &params.task, &params.agent.unwrap_or_default()).await?;
    }
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Check for benchmark regression
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CheckRegressionParams {
    /// Task name
    pub task: String,
    /// Baseline branch/commit
    pub baseline: Option<String>,
    /// Current branch/commit
    pub current: Option<String>,
    /// Metrics to check
    pub metrics: Option<Vec<String>>,
}

#[tool(name = "cua.benchmark.regression", description = "Check for performance regression in benchmark results")]
async fn check_regression(
    params: CheckRegressionParams,
    config: Arc<McpConfig>,
    regression_tracker: Arc<RegressionTracker>,
) -> Result<CallToolResult> {
    let baseline = params.baseline.unwrap_or_else(|| config.regression.baseline_branch.clone());
    let current = params.current.unwrap_or_else(|| "HEAD".to_string());
    let metrics = params.metrics.unwrap_or_else(|| config.regression.metrics.clone());
    
    // In real implementation, query regression database
    let result = serde_json::json!({
        "task": params.task,
        "baseline": baseline,
        "current": current,
        "metrics_checked": metrics,
        "regressions_detected": false,
        "details": [],
        "threshold_percent": config.regression.threshold_percent,
        "message": "No regressions detected (mock)"
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: List available sandbox profiles
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListProfilesParams {
    /// Filter by category
    pub category: Option<String>,
    /// Filter by region
    pub region: Option<String>,
}

#[tool(name = "cua.profile.list", description = "List available sandbox profiles")]
async fn list_profiles(
    params: ListProfilesParams,
    config: Arc<McpConfig>,
) -> Result<CallToolResult> {
    // In real implementation, scan profiles directory
    let profiles = vec![
        serde_json::json!({
            "name": "mpesa-daraja",
            "display_name": "M-Pesa Daraja Sandbox (Kenya)",
            "category": "mobile-money",
            "region": "KE",
            "language": "sw",
            "currency": "KES",
            "provider": "Safaricom"
        }),
        serde_json::json!({
            "name": "mtn-momo",
            "display_name": "MTN MoMo Sandbox (Ghana/Uganda)",
            "category": "mobile-money",
            "region": "GH",
            "language": "en",
            "currency": "GHS",
            "provider": "MTN Group"
        }),
        serde_json::json!({
            "name": "airtel-money",
            "display_name": "Airtel Money Sandbox (Nigeria/Uganda)",
            "category": "mobile-money",
            "region": "NG",
            "language": "en",
            "currency": "NGN",
            "provider": "Airtel Africa"
        }),
        serde_json::json!({
            "name": "orange-money",
            "display_name": "Orange Money Sandbox (Côte d'Ivoire/Senegal)",
            "category": "mobile-money",
            "region": "CI",
            "language": "fr",
            "currency": "XOF",
            "provider": "Orange Group"
        }),
        serde_json::json!({
            "name": "wave-money",
            "display_name": "Wave Money Sandbox (Senegal/CI)",
            "category": "mobile-money",
            "region": "SN",
            "language": "fr",
            "currency": "XOF",
            "provider": "Wave"
        }),
        serde_json::json!({
            "name": "nibss",
            "display_name": "NIBSS Sandbox (Nigeria)",
            "category": "fintech",
            "region": "NG",
            "language": "en",
            "currency": "NGN",
            "provider": "NIBSS"
        }),
        serde_json::json!({
            "name": "pesalink",
            "display_name": "PesaLink Sandbox (Kenya)",
            "category": "fintech",
            "region": "KE",
            "language": "sw",
            "currency": "KES",
            "provider": "IPSL"
        }),
        serde_json::json!({
            "name": "bankservafrica",
            "display_name": "BankservAfrica Sandbox (South Africa)",
            "category": "fintech",
            "region": "ZA",
            "language": "en",
            "currency": "ZAR",
            "provider": "BankservAfrica"
        }),
        serde_json::json!({
            "name": "openmrs",
            "display_name": "OpenMRS Healthcare Sandbox",
            "category": "healthcare",
            "region": "KE",
            "language": "en",
            "currency": "KES",
            "provider": "OpenMRS Community"
        }),
    ];
    
    let filtered: Vec<_> = profiles.into_iter()
        .filter(|p| params.category.as_ref().map_or(true, |c| p["category"] == *c))
        .filter(|p| params.region.as_ref().map_or(true, |r| p["region"] == *r))
        .collect();
    
    let result = serde_json::json!({
        "profiles": filtered,
        "total": filtered.len()
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Get profile details
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetProfileParams {
    /// Profile name
    pub name: String,
}

#[tool(name = "cua.profile.get", description = "Get detailed sandbox profile configuration")]
async fn get_profile(
    params: GetProfileParams,
    config: Arc<McpConfig>,
) -> Result<CallToolResult> {
    // In real implementation, load and parse YAML profile
    let result = serde_json::json!({
        "name": params.name,
        "display_name": format!("{} Profile", params.name),
        "category": "mobile-money",
        "region": "KE",
        "language": "sw",
        "currency": "KES",
        "provider": "Example Provider",
        "image": "ghcr.io/trycua/linux:latest",
        "ports": [8080, 8443, 3211],
        "resources": {
            "cpu": "2",
            "memory": "4G",
            "disk": "20G"
        },
        "skills": ["african-mobile-money/mpesa-sw", "gui-automation"]
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Destroy a sandbox
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DestroySandboxParams {
    /// Sandbox ID or session ID
    pub sandbox_id: String,
    /// Force destroy even if busy
    pub force: Option<bool>,
}

#[tool(name = "cua.sandbox.destroy", description = "Destroy a sandbox and clean up resources")]
async fn destroy_sandbox(
    params: DestroySandboxParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    session_manager.destroy_session(&params.sandbox_id).await?;
    
    let result = serde_json::json!({
        "sandbox_id": params.sandbox_id,
        "destroyed": true,
        "message": "Sandbox destroyed successfully"
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Get sandbox status
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetSandboxStatusParams {
    /// Sandbox ID or session ID
    pub sandbox_id: String,
}

#[tool(name = "cua.sandbox.status", description = "Get the status of a sandbox")]
async fn get_sandbox_status(
    params: GetSandboxStatusParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    let session = session_manager.get_session(&params.sandbox_id).await
        .ok_or_else(|| McpError::not_found(format!("Sandbox not found: {}", params.sandbox_id)))?;
    
    let result = serde_json::json!({
        "sandbox_id": params.sandbox_id,
        "status": "ready",
        "profile": session.profile,
        "created_at": session.created_at,
        "resources": {
            "cpu": "2",
            "memory": "4G",
            "disk": "20G"
        },
        "ports": {
            "spacesd": 3211,
            "app": 8080
        }
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Upload file to sandbox
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UploadFileParams {
    /// Sandbox ID or session ID
    pub sandbox_id: String,
    /// Local file path
    pub local_path: String,
    /// Remote destination path
    pub remote_path: String,
}

#[tool(name = "cua.sandbox.upload", description = "Upload a file to the sandbox")]
async fn upload_file(
    params: UploadFileParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    let session = session_manager.get_session(&params.sandbox_id).await
        .ok_or_else(|| McpError::not_found(format!("Sandbox not found: {}", params.sandbox_id)))?;
    
    let result = serde_json::json!({
        "sandbox_id": params.sandbox_id,
        "local_path": params.local_path,
        "remote_path": params.remote_path,
        "uploaded": true,
        "size_bytes": 1024
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}

/// Tool: Download file from sandbox
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DownloadFileParams {
    /// Sandbox ID or session ID
    pub sandbox_id: String,
    /// Remote file path
    pub remote_path: String,
    /// Local destination path
    pub local_path: String,
}

#[tool(name = "cua.sandbox.download", description = "Download a file from the sandbox")]
async fn download_file(
    params: DownloadFileParams,
    config: Arc<McpConfig>,
    session_manager: Arc<SessionManager>,
) -> Result<CallToolResult> {
    let session = session_manager.get_session(&params.sandbox_id).await
        .ok_or_else(|| McpError::not_found(format!("Sandbox not found: {}", params.sandbox_id)))?;
    
    let result = serde_json::json!({
        "sandbox_id": params.sandbox_id,
        "remote_path": params.remote_path,
        "local_path": params.local_path,
        "downloaded": true,
        "size_bytes": 1024
    });
    
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(result.to_string())]))
}