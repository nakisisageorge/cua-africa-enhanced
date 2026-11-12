//! Main MCP server implementation

use std::sync::Arc;
use tokio::signal;
use rmcp::{
    service::RunningService,
    transport::{StdioTransport, HttpTransport, SseTransport},
    ServerHandler,
};
use anyhow::Result;

use crate::{
    config::{McpConfig, TransportType},
    error::McpError,
    tools::ToolRegistry,
    session::SessionManager,
    regression::RegressionTracker,
};

/// Main MCP server
pub struct McpServer {
    config: Arc<McpConfig>,
    tool_registry: ToolRegistry,
    session_manager: Arc<SessionManager>,
    regression_tracker: Arc<RegressionTracker>,
}

impl McpServer {
    /// Create a new MCP server
    pub async fn new(config: McpConfig) -> Result<Self, McpError> {
        let config = Arc::new(config);
        
        let session_manager = Arc::new(SessionManager::new(config.clone()));
        let regression_tracker = Arc::new(RegressionTracker::new(config.clone()).await?);
        let tool_registry = ToolRegistry::new(config.clone(), session_manager.clone(), regression_tracker.clone());
        
        Ok(Self {
            config,
            tool_registry,
            session_manager,
            regression_tracker,
        })
    }
    
    /// Run the server with the configured transport(s)
    pub async fn run(self) -> Result<(), McpError> {
        match (self.config.transport.stdio, self.config.transport.http, self.config.transport.sse) {
            (true, false, false) => self.run_stdio().await,
            (false, true, false) => self.run_http().await,
            (false, false, true) => self.run_sse().await,
            (true, true, _) | (true, _, true) => self.run_combined().await,
            _ => self.run_stdio().await, // Default to stdio
        }
    }
    
    /// Run with stdio transport (for local agents)
    pub async fn run_stdio(self) -> Result<(), McpError> {
        let transport = StdioTransport::new();
        let service = self.serve(transport).await?;
        
        // Wait for shutdown signal
        signal::ctrl_c().await.map_err(|e| McpError::internal(format!("Signal error: {}", e)))?;
        
        service.cancel().await.map_err(|e| McpError::internal(format!("Shutdown error: {}", e)))?;
        Ok(())
    }
    
    /// Run with HTTP transport
    #[cfg(feature = "http")]
    pub async fn run_http(self) -> Result<(), McpError> {
        let addr = format!("{}:{}", self.config.transport.http_addr, self.config.transport.http_port).parse()
            .map_err(|e| McpError::internal(format!("Invalid address: {}", e)))?;
        
        let transport = HttpTransport::new(addr);
        let service = self.serve(transport).await?;
        
        signal::ctrl_c().await.map_err(|e| McpError::internal(format!("Signal error: {}", e)))?;
        
        service.cancel().await.map_err(|e| McpError::internal(format!("Shutdown error: {}", e)))?;
        Ok(())
    }
    
    /// Run with SSE transport
    #[cfg(feature = "http")]
    pub async fn run_sse(self) -> Result<(), McpError> {
        let addr = format!("{}:{}", self.config.transport.http_addr, self.config.transport.http_port).parse()
            .map_err(|e| McpError::internal(format!("Invalid address: {}", e)))?;
        
        let transport = SseTransport::new(addr, self.config.transport.sse_path.clone());
        let service = self.serve(transport).await?;
        
        signal::ctrl_c().await.map_err(|e| McpError::internal(format!("Signal error: {}", e)))?;
        
        service.cancel().await.map_err(|e| McpError::internal(format!("Shutdown error: {}", e)))?;
        Ok(())
    }
    
    /// Run with combined transports
    #[cfg(feature = "http")]
    pub async fn run_combined(self) -> Result<(), McpError> {
        // For combined transports, we'd need to run multiple services
        // This is a simplified version - in production you'd use a proper multi-transport setup
        if self.config.transport.stdio {
            self.run_stdio().await
        } else if self.config.transport.http {
            self.run_http().await
        } else {
            self.run_sse().await
        }
    }
    
    /// Serve with a specific transport
    async fn serve<T>(self, transport: T) -> Result<RunningService<T>, McpError>
    where
        T: rmcp::transport::Transport + Send + 'static,
    {
        let handler = McpServerHandler {
            tool_registry: self.tool_registry.into_router(),
            session_manager: self.session_manager,
            regression_tracker: self.regression_tracker,
            config: self.config,
        };
        
        let service = rmcp::serve(handler, transport)
            .await
            .map_err(|e| McpError::transport(format!("Failed to start server: {}", e)))?;
        
        Ok(service)
    }
    
    /// Create stdio server (for backward compatibility)
    pub async fn serve_stdio(self) -> Result<RunningService<StdioTransport>, McpError> {
        let transport = StdioTransport::new();
        self.serve(transport).await
    }
    
    /// Create HTTP server (for backward compatibility)
    #[cfg(feature = "http")]
    pub async fn serve_http(self) -> Result<RunningService<HttpTransport>, McpError> {
        let addr = format!("{}:{}", self.config.transport.http_addr, self.config.transport.http_port).parse()
            .map_err(|e| McpError::internal(format!("Invalid address: {}", e)))?;
        
        let transport = HttpTransport::new(addr);
        self.serve(transport).await
    }
}

/// MCP server handler
struct McpServerHandler {
    tool_registry: rmcp::handler::server::tool::ToolRouter<()>,
    session_manager: Arc<SessionManager>,
    regression_tracker: Arc<RegressionTracker>,
    config: Arc<McpConfig>,
}

#[rmcp::tool_handler]
impl ServerHandler for McpServerHandler {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        rmcp::model::ServerInfo {
            name: self.config.server.name.clone(),
            version: self.config.server.version.clone(),
            protocol_version: rmcp::model::ProtocolVersion::V_2024_11_05,
            capabilities: rmcp::model::ServerCapabilities {
                tools: Some(rmcp::model::ToolsCapability { list_changed: Some(true) }),
                resources: Some(rmcp::model::ResourcesCapability { subscribe: Some(true), list_changed: Some(true) }),
                prompts: Some(rmcp::model::PromptsCapability { list_changed: Some(true) }),
                logging: Some(rmcp::model::LoggingCapability {}),
                ..Default::default()
            },
            instructions: Some("Cua MCP Server - Computer-use orchestration for AI agents. Provides tools for sandbox management, skill invocation, benchmark execution, and regression detection.".to_string()),
        }
    }
    
    async fn list_tools(&self, _request: rmcp::model::PaginatedRequestParam) -> Result<rmcp::model::ListToolsResult, rmcp::Error> {
        let tools = self.tool_registry.list_tools(None).await?;
        Ok(rmcp::model::ListToolsResult { tools, next_cursor: None })
    }
    
    async fn call_tool(&self, request: rmcp::model::CallToolRequestParam) -> Result<rmcp::model::CallToolResult, rmcp::Error> {
        self.tool_registry.call_tool(request.name, request.arguments).await
            .map_err(|e| rmcp::Error::internal_error(e.to_string(), None))
    }
    
    async fn list_resources(&self, _request: rmcp::model::PaginatedRequestParam) -> Result<rmcp::model::ListResourcesResult, rmcp::Error> {
        // Return sandbox profiles as resources
        let resources = vec![
            rmcp::model::Resource {
                uri: "cua://profiles".to_string(),
                name: "Sandbox Profiles".to_string(),
                description: Some("Available African sandbox profiles".to_string()),
                mime_type: Some("application/json".to_string()),
                annotations: None,
            }
        ];
        Ok(rmcp::model::ListResourcesResult { resources, next_cursor: None })
    }
    
    async fn read_resource(&self, request: rmcp::model::ReadResourceRequestParam) -> Result<rmcp::model::ReadResourceResult, rmcp::Error> {
        if request.uri == "cua://profiles" {
            // Return profile list
            let content = serde_json::json!({
                "profiles": [
                    "mpesa-daraja", "mtn-momo", "airtel-money", "orange-money", "wave-money",
                    "nibss", "pesalink", "bankservafrica", "openmrs"
                ]
            }).to_string();
            
            Ok(rmcp::model::ReadResourceResult {
                contents: vec![rmcp::model::ResourceContents::Text {
                    uri: request.uri,
                    text: content,
                    mime_type: Some("application/json".to_string()),
                }]
            })
        } else {
            Err(rmcp::Error::resource_not_found(request.uri))
        }
    }
    
    async fn list_prompts(&self, _request: rmcp::model::PaginatedRequestParam) -> Result<rmcp::model::ListPromptsResult, rmcp::Error> {
        let prompts = vec![
            rmcp::model::Prompt {
                name: "cua.mobile-money.test".to_string(),
                description: Some("Test a mobile money integration in an African sandbox".to_string()),
                arguments: Some(vec![
                    rmcp::model::PromptArgument {
                        name: "provider".to_string(),
                        description: Some("Mobile money provider (mpesa, mtn, airtel, orange, wave)".to_string()),
                        required: true,
                    },
                    rmcp::model::model::PromptArgument {
                        name: "test_case".to_string(),
                        description: Some("Test case to run (stk_push, balance, transfer, kyc)".to_string()),
                        required: false,
                    },
                ]),
            },
            rmcp::model::Prompt {
                name: "cua.fintech.payment".to_string(),
                description: Some("Test a fintech payment integration".to_string()),
                arguments: Some(vec![
                    rmcp::model::PromptArgument {
                        name: "system".to_string(),
                        description: Some("Payment system (nibss, pesalink, bankservafrica)".to_string()),
                        required: true,
                    },
                ]),
            },
            rmcp::model::Prompt {
                name: "cua.benchmark.regression".to_string(),
                description: Some("Run benchmark and check for regression".to_string()),
                arguments: Some(vec![
                    rmcp::model::PromptArgument {
                        name: "task".to_string(),
                        description: Some("Benchmark task name".to_string()),
                        required: true,
                    },
                    rmcp::model::PromptArgument {
                        name: "agent".to_string(),
                        description: Some("Agent to use (cua-agent, harness)".to_string()),
                        required: false,
                    },
                ]),
            },
        ];
        
        Ok(rmcp::model::ListPromptsResult { prompts, next_cursor: None })
    }
    
    async fn get_prompt(&self, request: rmcp::model::GetPromptRequestParam) -> Result<rmcp::model::GetPromptResult, rmcp::Error> {
        let prompt = match request.name.as_str() {
            "cua.mobile-money.test" => {
                let provider = request.arguments.as_ref()
                    .and_then(|a| a.get("provider"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("mpesa");
                
                let test_case = request.arguments.as_ref()
                    .and_then(|a| a.get("test_case"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("stk_push");
                
                rmcp::model::Prompt {
                    name: request.name,
                    description: Some(format!("Test {} {} integration", provider, test_case)),
                    arguments: request.arguments,
                }
            }
            "cua.fintech.payment" => {
                let system = request.arguments.as_ref()
                    .and_then(|a| a.get("system"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("nibss");
                
                rmcp::model::Prompt {
                    name: request.name,
                    description: Some(format!("Test {} payment integration", system)),
                    arguments: request.arguments,
                }
            }
            "cua.benchmark.regression" => {
                let task = request.arguments.as_ref()
                    .and_then(|a| a.get("task"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("cua-bench-basic");
                
                let agent = request.arguments.as_ref()
                    .and_then(|a| a.get("agent"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("cua-agent");
                
                rmcp::model::Prompt {
                    name: request.name,
                    description: Some(format!("Run {} with {} and check for regression", task, agent)),
                    arguments: request.arguments,
                }
            }
            _ => return Err(rmcp::Error::prompt_not_found(request.name)),
        };
        
        Ok(rmcp::model::GetPromptResult {
            description: prompt.description,
            messages: vec![
                rmcp::model::PromptMessage {
                    role: rmcp::model::PromptMessageRole::User,
                    content: rmcp::model::PromptMessageContent::Text {
                        text: format!("Execute the following prompt: {}", prompt.description.unwrap_or_default()),
                    },
                }
            ],
        })
    }
}