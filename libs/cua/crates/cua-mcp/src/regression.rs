//! Benchmark regression tracking for cua-mcp

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, Row};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;

use crate::config::McpConfig;
use crate::error::{McpError, Result};

/// Benchmark run record
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BenchmarkRun {
    pub id: String,
    pub task: String,
    pub agent: String,
    pub model: Option<String>,
    pub baseline_branch: String,
    pub current_branch: String,
    pub commit_sha: String,
    pub status: RunStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub metrics: HashMap<String, f64>,
    pub regression_detected: bool,
    pub regression_details: Vec<RegressionDetail>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RunStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegressionDetail {
    pub metric: String,
    pub baseline_value: f64,
    pub current_value: f64,
    pub change_percent: f64,
    pub threshold_percent: f64,
    pub is_regression: bool,
    pub severity: RegressionSeverity,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RegressionSeverity {
    Info,
    Warning,
    Critical,
}

/// Regression tracker using SQLite database
pub struct RegressionTracker {
    pool: Pool<Sqlite>,
    config: Arc<McpConfig>,
    cache: Arc<RwLock<HashMap<String, BenchmarkRun>>>,
}

impl RegressionTracker {
    pub async fn new(config: Arc<McpConfig>) -> Result<Self> {
        // Ensure database directory exists
        if let Some(parent) = config.regression.db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&format!("sqlite:{}", config.regression.db_path.display())).await?;
        
        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;
        
        let tracker = Self {
            pool,
            config,
            cache: Arc::new(RwLock::new(HashMap::new())),
        };
        
        Ok(tracker)
    }
    
    /// Record benchmark run start
    pub async fn record_run_start(
        &self,
        run_id: &str,
        task: &str,
        agent: &str,
    ) -> Result<()> {
        let run = BenchmarkRun {
            id: run_id.to_string(),
            task: task.to_string(),
            agent: agent.to_string(),
            model: None,
            baseline_branch: self.config.regression.baseline_branch.clone(),
            current_branch: "HEAD".to_string(),
            commit_sha: git_commit_sha().await.unwrap_or_default(),
            status: RunStatus::Running,
            started_at: Utc::now(),
            completed_at: None,
            metrics: HashMap::new(),
            regression_detected: false,
            regression_details: Vec::new(),
        };
        
        self.cache.write().await.insert(run_id.to_string(), run.clone());
        
        sqlx::query!(
            r#"
            INSERT INTO benchmark_runs (id, task, agent, model, baseline_branch, current_branch, commit_sha, status, started_at, metrics, regression_detected, regression_details)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            run.id,
            run.task,
            run.agent,
            run.model,
            run.baseline_branch,
            run.current_branch,
            run.commit_sha,
            serde_json::to_string(&run.status)?,
            run.started_at,
            serde_json::to_string(&run.metrics)?,
            run.regression_detected,
            serde_json::to_string(&run.regression_details)?
        ).execute(&self.pool).await?;
        
        Ok(())
    }
    
    /// Record benchmark run completion with metrics
    pub async fn record_run_completion(
        &self,
        run_id: &str,
        metrics: HashMap<String, f64>,
        status: RunStatus,
    ) -> Result<()> {
        let mut cache = self.cache.write().await;
        if let Some(run) = cache.get_mut(run_id) {
            run.metrics = metrics.clone();
            run.status = status.clone();
            run.completed_at = Some(Utc::now());
            
            // Check for regression if enabled
            if self.config.regression.enabled {
                let (regression_detected, details) = self.check_regression(run).await?;
                run.regression_detected = regression_detected;
                run.regression_details = details;
            }
        }
        
        sqlx::query!(
            r#"
            UPDATE benchmark_runs
            SET status = ?, completed_at = ?, metrics = ?, regression_detected = ?, regression_details = ?
            WHERE id = ?
            "#,
            serde_json::to_string(&status)?,
            Utc::now(),
            serde_json::to_string(&metrics)?,
            cache.get(run_id).map(|r| r.regression_detected).unwrap_or(false),
            cache.get(run_id).map(|r| serde_json::to_string(&r.regression_details).unwrap()).unwrap_or_default(),
            run_id
        ).execute(&self.pool).await?;
        
        Ok(())
    }
    
    /// Check for regression against baseline
    async fn check_regression(&self, run: &BenchmarkRun) -> Result<(bool, Vec<RegressionDetail>)> {
        // Get baseline runs for the same task and agent
        let baseline_runs = sqlx::query!(
            r#"
            SELECT metrics FROM benchmark_runs
            WHERE task = ? AND agent = ? AND current_branch = ? AND status = 'Completed'
            ORDER BY started_at DESC
            LIMIT 10
            "#,
            run.task,
            run.agent,
            run.baseline_branch
        ).fetch_all(&self.pool).await?;
        
        if baseline_runs.is_empty() {
            return Ok((false, Vec::new()));
        }
        
        // Calculate median baseline metrics
        let mut baseline_metrics: HashMap<String, Vec<f64>> = HashMap::new();
        for row in baseline_runs {
            if let Ok(metrics) = serde_json::from_str::<HashMap<String, f64>>(&row.metrics) {
                for (key, value) in metrics {
                    baseline_metrics.entry(key).or_default().push(value);
                }
            }
        }
        
        let mut details = Vec::new();
        let mut any_regression = false;
        
        for (metric, current_value) in &run.metrics {
            if let Some(baseline_values) = baseline_metrics.get(metric) {
                if !baseline_values.is_empty() {
                    // Calculate median
                    let mut sorted = baseline_values.clone();
                    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    let median = if sorted.len() % 2 == 0 {
                        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
                    } else {
                        sorted[sorted.len() / 2]
                    };
                    
                    // Calculate percent change
                    let change_percent = if median != 0.0 {
                        ((current_value - median) / median) * 100.0
                    } else {
                        0.0
                    };
                    
                    let threshold = self.config.regression.threshold_percent;
                    let is_regression = change_percent.abs() > threshold;
                    
                    if is_regression {
                        any_regression = true;
                    }
                    
                    let severity = if change_percent.abs() > threshold * 2.0 {
                        RegressionSeverity::Critical
                    } else if change_percent.abs() > threshold {
                        RegressionSeverity::Warning
                    } else {
                        RegressionSeverity::Info
                    };
                    
                    details.push(RegressionDetail {
                        metric: metric.clone(),
                        baseline_value: median,
                        current_value: *current_value,
                        change_percent,
                        threshold_percent: threshold,
                        is_regression,
                        severity,
                    });
                }
            }
        }
        
        Ok((any_regression, details))
    }
    
    /// Get recent runs for a task
    pub async fn get_recent_runs(&self, task: &str, agent: &str, limit: usize) -> Result<Vec<BenchmarkRun>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, task, agent, model, baseline_branch, current_branch, commit_sha, status, started_at, completed_at, metrics, regression_detected, regression_details
            FROM benchmark_runs
            WHERE task = ? AND agent = ?
            ORDER BY started_at DESC
            LIMIT ?
            "#,
            task,
            agent,
            limit as i64
        ).fetch_all(&self.pool).await?;
        
        let mut runs = Vec::new();
        for row in rows {
            runs.push(BenchmarkRun {
                id: row.id,
                task: row.task,
                agent: row.agent,
                model: row.model,
                baseline_branch: row.baseline_branch,
                current_branch: row.current_branch,
                commit_sha: row.commit_sha,
                status: serde_json::from_str(&row.status).unwrap_or(RunStatus::Pending),
                started_at: row.started_at,
                completed_at: row.completed_at,
                metrics: serde_json::from_str(&row.metrics).unwrap_or_default(),
                regression_detected: row.regression_detected,
                regression_details: serde_json::from_str(&row.regression_details).unwrap_or_default(),
            });
        }
        
        Ok(runs)
    }
    
    /// Get regression summary for a task
    pub async fn get_regression_summary(&self, task: &str, agent: &str) -> Result<RegressionSummary> {
        let runs = self.get_recent_runs(task, agent, 20).await?;
        let completed: Vec<_> = runs.into_iter().filter(|r| r.status == RunStatus::Completed).collect();
        
        if completed.is_empty() {
            return Ok(RegressionSummary {
                task: task.to_string(),
                agent: agent.to_string(),
                total_runs: 0,
                regressions_detected: 0,
                latest_run: None,
                metrics_trend: HashMap::new(),
            });
        }
        
        let latest = completed.first().cloned();
        let regressions = completed.iter().filter(|r| r.regression_detected).count();
        
        // Calculate trend for each metric
        let mut metrics_trend = HashMap::new();
        for run in &completed {
            for (metric, value) in &run.metrics {
                metrics_trend.entry(metric.clone()).or_insert_with(Vec::new).push(*value);
            }
        }
        
        Ok(RegressionSummary {
            task: task.to_string(),
            agent: agent.to_string(),
            total_runs: completed.len(),
            regressions_detected: regressions,
            latest_run: latest,
            metrics_trend,
        })
    }
}

/// Regression summary for reporting
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegressionSummary {
    pub task: String,
    pub agent: String,
    pub total_runs: usize,
    pub regressions_detected: usize,
    pub latest_run: Option<BenchmarkRun>,
    pub metrics_trend: HashMap<String, Vec<f64>>,
}

/// Get current git commit SHA
async fn git_commit_sha() -> Result<String> {
    let output = tokio::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .await?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Ok("unknown".to_string())
    }
}