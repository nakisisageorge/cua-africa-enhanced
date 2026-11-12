-- Create benchmark_runs table for regression tracking
CREATE TABLE IF NOT EXISTS benchmark_runs (
    id TEXT PRIMARY KEY,
    task TEXT NOT NULL,
    agent TEXT NOT NULL,
    model TEXT,
    baseline_branch TEXT NOT NULL,
    current_branch TEXT NOT NULL,
    commit_sha TEXT NOT NULL,
    status TEXT NOT NULL,
    started_at TEXT NOT NULL,
    completed_at TEXT,
    metrics TEXT NOT NULL DEFAULT '{}',
    regression_detected BOOLEAN NOT NULL DEFAULT 0,
    regression_details TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Index for querying by task and agent
CREATE INDEX IF NOT EXISTS idx_benchmark_runs_task_agent 
ON benchmark_runs(task, agent);

-- Index for querying by branch
CREATE INDEX IF NOT EXISTS idx_benchmark_runs_branch 
ON benchmark_runs(current_branch);

-- Index for time-based queries
CREATE INDEX IF NOT EXISTS idx_benchmark_runs_started_at 
ON benchmark_runs(started_at);

-- Create sessions table for session persistence
CREATE TABLE IF NOT EXISTS mcp_sessions (
    id TEXT PRIMARY KEY,
    profile TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT NOT NULL,
    sandbox_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    ports TEXT NOT NULL DEFAULT '{}',
    resources TEXT NOT NULL DEFAULT '{}',
    metadata TEXT NOT NULL DEFAULT '{}'
);

-- Index for session expiry
CREATE INDEX IF NOT EXISTS idx_mcp_sessions_expires_at 
ON mcp_sessions(expires_at);

-- Create API keys table for authentication
CREATE TABLE IF NOT EXISTS api_keys (
    key_hash TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    permissions TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    last_used_at TEXT,
    expires_at TEXT,
    rate_limit_rpm INTEGER NOT NULL DEFAULT 60,
    active BOOLEAN NOT NULL DEFAULT 1
);

-- Create rate limit tracking table
CREATE TABLE IF NOT EXISTS rate_limits (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    api_key_hash TEXT NOT NULL,
    endpoint TEXT NOT NULL,
    request_count INTEGER NOT NULL DEFAULT 1,
    window_start TEXT NOT NULL,
    FOREIGN KEY (api_key_hash) REFERENCES api_keys(key_hash)
);

CREATE INDEX IF NOT EXISTS idx_rate_limits_key_window 
ON rate_limits(api_key_hash, window_start);