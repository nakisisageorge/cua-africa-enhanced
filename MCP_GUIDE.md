# MCP_GUIDE.md — cua-mcp Model Context Protocol Server

## Overview

The `cua-mcp` server exposes Cua's computer-use capabilities to AI agents via the Model Context Protocol (MCP). It enables agents like Claude Code, Codex, Cursor, and custom MCP clients to orchestrate sandboxes, invoke skills, run benchmarks, and manage African market test environments.

## Quick Start

### Installation

```bash
# From source (requires Rust 1.97+)
git clone https://github.com/nakisisageorge/cua-enhanced
cd cua-enhanced/libs/cua/crates/cua-mcp
cargo build --release

# Or install via cargo (when published)
cargo install cua-mcp
```

### Running the Server

#### Stdio Transport (Local Agents)
```bash
# For Claude Code, Codex, Cursor
cua-mcp serve --stdio

# With custom config
cua-mcp --config cua-mcp.toml serve --stdio
```

#### HTTP/SSE Transport (Remote Orchestration)
```bash
# HTTP + SSE on port 3212
cua-mcp serve --http --sse

# With custom address
cua-mcp serve --http --sse --addr 0.0.0.0 --port 8080
```

### Configuration

Create `cua-mcp.toml`:

```toml
[server]
name = "cua-mcp"
version = "0.1.0"
max_sessions = 100
request_timeout_secs = 300
debug = false

[transport]
stdio = true
http = true
sse = true
http_addr = "127.0.0.1"
http_port = 3212
sse_path = "/sse"
cors_origins = ["*"]

[cua]
default_image = "ghcr.io/trycua/linux:latest"
default_kind = "container"
default_runtime = "gvisor"
default_cpu = "2"
default_memory = "4G"
enable_spacesd = true

[security]
require_api_key = false
api_keys = {}
enable_mtls = false
cert_file = ""
key_file = ""
ca_file = ""
allowed_networks = [
  "api.safaricom.co.ke",
  "sandbox.momodeveloper.mtn.com",
  "openapi.airtel.africa",
  "api.orange.com",
  "api.wave.com",
  "github.com",
  "crates.io"
]

[security.default_limits]
max_cpu = "4"
max_memory = "8G"
max_disk = "50G"
max_processes = 100
max_open_files = 1024
network_enabled = true
read_only_rootfs = false

[session]
ttl_minutes = 60
max_per_key = 10
persist = true
store_path = ".cua/mcp/sessions"
rate_limit_rpm = 60

[regression]
enabled = true
db_path = ".cua/mcp/regression.db"
threshold_percent = 5.0
metrics = [
  "success_rate",
  "steps_to_completion",
  "token_usage",
  "latency_ms",
  "screenshot_similarity"
]
baseline_branch = "main"

[profiles]
dir = "libs/cua/sandbox-profiles/africa"
auto_discover = true
default = "mpesa-daraja"
```

## Available Tools

### 1. `cua.sandbox.create`
Create a new sandbox from an African profile or custom configuration.

**Parameters:**
```json
{
  "profile": "mpesa-daraja",
  "name": "my-test-sandbox",
  "image": "ghcr.io/trycua/linux:latest",
  "cpu": "2",
  "memory": "4G",
  "disk": "20G",
  "network_enabled": true,
  "env": {
    "MPESA_CONSUMER_KEY": "your-key",
    "MPESA_CONSUMER_SECRET": "your-secret"
  }
}
```

**Returns:** Sandbox ID, session ID, status, estimated ready time.

### 2. `cua.sandbox.execute`
Execute a command in a sandbox.

**Parameters:**
```json
{
  "sandbox_id": "mpesa-daraja-abc123",
  "command": "curl -X POST http://localhost:8080/oauth/v1/generate ...",
  "workdir": "/home/user",
  "env": { "VAR": "value" },
  "timeout_secs": 60,
  "user": "root"
}
```

### 3. `cua.sandbox.screenshot`
Capture a screenshot from the sandbox.

**Parameters:**
```json
{
  "sandbox_id": "mpesa-daraja-abc123",
  "format": "png",
  "quality": 80,
  "region": [0, 0, 1920, 1080]
}
```

**Returns:** Base64-encoded image data with metadata.

### 4. `cua.sandbox.input`
Send mouse/keyboard input to the sandbox.

**Parameters:**
```json
{
  "sandbox_id": "mpesa-daraja-abc123",
  "input_type": "click",
  "x": 500,
  "y": 300
}
```

**Input Types:** `click`, `type`, `key`, `scroll`, `drag`, `move`, `hotkey`

### 5. `cua.skill.list`
List available computer-use skills.

**Parameters:**
```json
{
  "category": "mobile-money",
  "language": "sw"
}
```

### 6. `cua.skill.invoke`
Invoke a skill with parameters.

**Parameters:**
```json
{
  "sandbox_id": "mpesa-daraja-abc123",
  "skill": "african-mobile-money/mpesa-sw",
  "command": "stk_push",
  "args": {
    "phone": "254712345678",
    "amount": 1000,
    "reference": "TEST-001"
  }
}
```

### 7. `cua.benchmark.run`
Run a cua-bench task or dataset.

**Parameters:**
```json
{
  "task": "cua-bench-basic",
  "agent": "cua-agent",
  "model": "anthropic/claude-sonnet-4-20250514",
  "parallel": 4,
  "attempts": 3,
  "image": "ghcr.io/trycua/linux:latest",
  "kind": "container",
  "runtime": "gvisor"
}
```

### 8. `cua.benchmark.regression`
Check for performance regression in benchmark results.

**Parameters:**
```json
{
  "task": "cua-bench-basic",
  "baseline": "main",
  "current": "HEAD",
  "metrics": ["success_rate", "steps_to_completion", "latency_ms"]
}
```

### 9. `cua.profile.list`
List available sandbox profiles.

**Parameters:**
```json
{
  "category": "mobile-money",
  "region": "KE"
}
```

### 10. `cua.profile.get`
Get detailed sandbox profile configuration.

**Parameters:**
```json
{
  "name": "mpesa-daraja"
}
```

### 11. `cua.sandbox.destroy`
Destroy a sandbox and clean up resources.

**Parameters:**
```json
{
  "sandbox_id": "mpesa-daraja-abc123",
  "force": true
}
```

### 12. `cua.sandbox.status`
Get the status of a sandbox.

**Parameters:**
```json
{
  "sandbox_id": "mpesa-daraja-abc123"
}
```

## Available Prompts

### `cua.mobile-money.test`
Test a mobile money integration in an African sandbox.

**Arguments:**
- `provider`: Mobile money provider (mpesa, mtn, airtel, orange, wave)
- `test_case`: Test case to run (stk_push, balance, transfer, kyc)

### `cua.fintech.payment`
Test a fintech payment integration.

**Arguments:**
- `system`: Payment system (nibss, pesalink, bankservafrica)

### `cua.benchmark.regression`
Run benchmark and check for regression.

**Arguments:**
- `task`: Benchmark task name
- `agent`: Agent to use (cua-agent, harness)

## Resources

### `cua://profiles`
Returns list of available sandbox profiles as JSON.

## Integration Examples

### Claude Code
Add to `.claude/settings.json`:
```json
{
  "mcpServers": {
    "cua": {
      "command": "cua-mcp",
      "args": ["serve", "--stdio"]
    }
  }
}
```

### Codex
Add to `~/.codex/config.toml`:
```toml
[mcp.servers.cua]
command = "cua-mcp"
args = ["serve", "--stdio"]
```

### Custom HTTP Client
```python
import httpx
import json

async def call_cua_mcp(tool: str, params: dict):
    async with httpx.AsyncClient() as client:
        response = await client.post(
            "http://localhost:3212/mcp",
            json={
                "jsonrpc": "2.0",
                "id": 1,
                "method": "tools/call",
                "params": {"name": tool, "arguments": params}
            }
        )
        return response.json()
```

## Authentication

### Development (No Auth)
Set `require_api_key = false` in config.

### Production (API Key)
```toml
[security]
require_api_key = true
api_keys = {
  "prod-key-1" = ["sandbox:create", "sandbox:execute", "skill:invoke"],
  "prod-key-2" = ["benchmark:run", "benchmark:regression"]
}
```

### mTLS (Enterprise)
```toml
[security]
enable_mtls = true
cert_file = "/etc/cua-mcp/server.crt"
key_file = "/etc/cua-mcp/server.key"
ca_file = "/etc/cua-mcp/ca.crt"
```

## Rate Limiting

Default: 60 requests/minute per API key.

Configure in `cua-mcp.toml`:
```toml
[session]
rate_limit_rpm = 120  # requests per minute
```

## Session Management

Sessions auto-expire after 60 minutes (configurable). Use `cua.sandbox.destroy` to clean up early.

```toml
[session]
ttl_minutes = 120
max_per_key = 20
persist = true
```

## Regression Tracking

The server automatically tracks benchmark runs and detects regressions:

```toml
[regression]
enabled = true
db_path = ".cua/mcp/regression.db"
threshold_percent = 5.0
metrics = ["success_rate", "steps_to_completion", "token_usage", "latency_ms", "screenshot_similarity"]
baseline_branch = "main"
```

### CLI Regression Check
```bash
cua-mcp regression --baseline main --current HEAD --format table
```

Output formats: `table`, `json`, `csv`

## African Sandbox Profiles

Pre-configured profiles in `libs/cua/sandbox-profiles/africa/`:

| Profile | Category | Region | Language | Currency |
|---------|----------|--------|----------|----------|
| `mpesa-daraja` | mobile-money | KE | sw | KES |
| `mtn-momo` | mobile-money | GH | en | GHS |
| `airtel-money` | mobile-money | NG | en | NGN |
| `orange-money` | mobile-money | CI | fr | XOF |
| `wave-money` | mobile-money | SN | fr | XOF |
| `nibss` | fintech | NG | en | NGN |
| `pesalink` | fintech | KE | sw | KES |
| `bankservafrica` | fintech | ZA | en | ZAR |
| `openmrs` | healthcare | KE | en | KES |

## Troubleshooting

### Sandbox Won't Start
1. Check Docker/gVisor is running
2. Verify image exists: `docker pull ghcr.io/trycua/linux:latest`
3. Check resource limits (CPU, memory, disk)
4. Review logs: `cua-mcp` logs to stderr

### MCP Connection Failed
1. Verify server is running: `cua-mcp serve --stdio`
2. Check client config points to correct command
3. For HTTP: verify port 3212 is accessible
4. Check firewall rules

### Regression DB Errors
1. Ensure db directory exists: `mkdir -p .cua/mcp`
2. Check SQLite permissions
3. Run migrations: `sqlx migrate run --source migrations`

## Best Practices

1. **Use profiles** for consistent African market testing
2. **Destroy sandboxes** when done to free resources
3. **Set API keys** in production environments
4. **Monitor regression alerts** in CI/CD
5. **Use mTLS** for production HTTP deployments
6. **Profile sandboxes** for your specific use case

## Support

- **Documentation**: https://cua.ai/docs
- **Issues**: https://github.com/nakisisageorge/cua-enhanced/issues
- **Discord**: https://discord.gg/mVnXXpdE85
- **Email**: nakisisageorge@gmail.com