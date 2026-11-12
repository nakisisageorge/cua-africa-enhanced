//! Session management for cua-mcp

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

use crate::config::McpConfig;
use crate::error::{McpError, Result};

/// Sandbox session information
#[derive(Debug, Clone)]
pub struct SandboxSession {
    pub id: String,
    pub profile: String,
    pub name: String,
    pub status: SessionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub sandbox_id: Option<String>,
    pub ports: HashMap<String, u16>,
    pub resources: SessionResources,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SessionStatus {
    Creating,
    Ready,
    Busy,
    Error(String),
    Destroying,
    Destroyed,
}

#[derive(Debug, Clone)]
pub struct SessionResources {
    pub cpu: String,
    pub memory: String,
    pub disk: String,
    pub network_enabled: bool,
}

/// Session manager for tracking active sandbox sessions
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, SandboxSession>>>,
    config: Arc<McpConfig>,
    cleanup_interval: Duration,
}

impl SessionManager {
    pub fn new(config: Arc<McpConfig>) -> Self {
        let ttl = Duration::from_secs(config.session.ttl_minutes * 60);
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
            cleanup_interval: Duration::from_secs(60), // Check every minute
        }
    }
    
    /// Create a new session
    pub async fn create_session(&self) -> Result<String> {
        let session_id = format!("sess-{}", Uuid::new_v4().to_string()[..12].to_string());
        let now = Utc::now();
        let expires_at = now + chrono::Duration::minutes(self.config.session.ttl_minutes as i64);
        
        let session = SandboxSession {
            id: session_id.clone(),
            profile: "".to_string(),
            name: "".to_string(),
            status: SessionStatus::Creating,
            created_at: now,
            updated_at: now,
            expires_at,
            sandbox_id: None,
            ports: HashMap::new(),
            resources: SessionResources {
                cpu: self.config.cua.default_cpu.clone(),
                memory: self.config.cua.default_memory.clone(),
                disk: "20G".to_string(),
                network_enabled: self.config.security.default_limits.network_enabled,
            },
            metadata: HashMap::new(),
        };
        
        self.sessions.write().await.insert(session_id.clone(), session);
        
        // Start cleanup task if not already running
        self.start_cleanup_task().await;
        
        Ok(session_id)
    }
    
    /// Create a session with a specific profile
    pub async fn create_session_with_profile(&self, profile: &str, name: Option<String>) -> Result<String> {
        let session_id = self.create_session().await?;
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(&session_id) {
            session.profile = profile.to_string();
            session.name = name.unwrap_or_else(|| format!("{}-{}", profile, &session_id[5..13]));
        }
        
        Ok(session_id)
    }
    
    /// Get a session by ID
    pub async fn get_session(&self, id: &str) -> Option<SandboxSession> {
        self.sessions.read().await.get(id).cloned()
    }
    
    /// Update session status
    pub async fn update_status(&self, id: &str, status: SessionStatus) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(id) {
            session.status = status;
            session.updated_at = Utc::now();
            Ok(())
        } else {
            Err(McpError::not_found(format!("Session not found: {}", id)))
        }
    }
    
    /// Update session with sandbox ID and ports
    pub async fn update_sandbox_info(&self, id: &str, sandbox_id: String, ports: HashMap<String, u16>) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(id) {
            session.sandbox_id = Some(sandbox_id);
            session.ports = ports;
            session.status = SessionStatus::Ready;
            session.updated_at = Utc::now();
            Ok(())
        } else {
            Err(McpError::not_found(format!("Session not found: {}", id)))
        }
    }
    
    /// Extend session TTL
    pub async fn extend_session(&self, id: &str, additional_minutes: u64) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(id) {
            session.expires_at = Utc::now() + chrono::Duration::minutes(additional_minutes as i64);
            session.updated_at = Utc::now();
            Ok(())
        } else {
            Err(McpError::not_found(format!("Session not found: {}", id)))
        }
    }
    
    /// Destroy a session
    pub async fn destroy_session(&self, id: &str) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = sessions.get_mut(id) {
            session.status = SessionStatus::Destroying;
            session.updated_at = Utc::now();
            
            // In real implementation, destroy the actual sandbox here
            // For now, just mark as destroyed
            session.status = SessionStatus::Destroyed;
            
            sessions.remove(id);
            Ok(())
        } else {
            Err(McpError::not_found(format!("Session not found: {}", id)))
        }
    }
    
    /// List all active sessions
    pub async fn list_sessions(&self) -> Vec<SandboxSession> {
        self.sessions.read().await.values().cloned().collect()
    }
    
    /// Get session count
    pub async fn session_count(&self) -> usize {
        self.sessions.read().await.len()
    }
    
    /// Clean up expired sessions
    async fn cleanup_expired(&self) -> Result<()> {
        let now = Utc::now();
        let mut sessions = self.sessions.write().await;
        
        let expired_ids: Vec<String> = sessions
            .iter()
            .filter(|(_, s)| s.expires_at < now && s.status != SessionStatus::Destroyed)
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in expired_ids {
            if let Some(session) = sessions.get_mut(&id) {
                session.status = SessionStatus::Destroying;
                // In real implementation, destroy sandbox
                session.status = SessionStatus::Destroyed;
            }
            sessions.remove(&id);
        }
        
        Ok(())
    }
    
    /// Start background cleanup task
    async fn start_cleanup_task(&self) {
        let sessions = self.sessions.clone();
        let interval = self.cleanup_interval;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            loop {
                interval_timer.tick().await;
                let now = Utc::now();
                let mut sessions_write = sessions.write().await;
                
                let expired_ids: Vec<String> = sessions_write
                    .iter()
                    .filter(|(_, s)| s.expires_at < now && s.status != SessionStatus::Destroyed)
                    .map(|(id, _)| id.clone())
                    .collect();
                
                for id in expired_ids {
                    sessions_write.remove(&id);
                }
            }
        });
    }
    
    /// Check rate limit for API key
    pub async fn check_rate_limit(&self, api_key: &str) -> Result<()> {
        // In real implementation, track requests per API key
        // For now, always allow
        Ok(())
    }
}