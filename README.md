<div align="center">
  <a href="https://cua.ai" target="_blank" rel="noopener noreferrer">
    <picture>
      <source media="(prefers-color-scheme: dark)" alt="Cua logo" width="150" srcset="img/logo_white.svg">
      <source media="(prefers-color-scheme: light)" alt="Cua logo" width="150" srcset="img/logo_black.svg">
      <img alt="Cua logo" width="150" src="img/logo_black.svg">
    </picture>
  </a>

  <p align="center"><strong>Enhanced Cua distribution with African sandbox profiles, MCP server, benchmark regression detection, and interactive architecture visualization.</strong></p>

  <p align="center">
    <a href="https://github.com/nakisisageorge/cua-africa-enhanced"><img src="https://img.shields.io/badge/Repository-cua--africa--enhanced-7C3AED?style=for-the-badge&logo=github&logoColor=white" alt="Repository"></a>
    <a href="https://cua.ai" target="_blank" rel="noopener noreferrer"><img src="https://img.shields.io/badge/Original-cua.ai-0ea5e9?style=for-the-badge" alt="Original Project"></a>
    <a href="https://github.com/trycua/cua" target="_blank" rel="noopener noreferrer"><img src="https://img.shields.io/badge/Upstream-trycua/cua-10B981?style=for-the-badge&logo=github&logoColor=white" alt="Upstream"></a>
    <br>
    <a href="https://github.com/nakisisageorge/cua-africa-enhanced/actions"><img src="https://img.shields.io/github/actions/workflow/status/nakisisageorge/cua-africa-enhanced/ci-cargo-audit.yml?label=cargo-audit&style=flat-square&logo=rust&logoColor=white" alt="cargo-audit"></a>
    <a href="https://github.com/nakisisageorge/cua-africa-enhanced/actions"><img src="https://img.shields.io/github/actions/workflow/status/nakisisageorge/cua-africa-enhanced/ci-cargo-deny.yml?label=cargo-deny&style=flat-square&logo=rust&logoColor=white" alt="cargo-deny"></a>
    <a href="https://github.com/nakisisageorge/cua-africa-enhanced/actions"><img src="https://img.shields.io/github/actions/workflow/status/nakisisageorge/cua-africa-enhanced/ci-trufflehog.yml?label=TruffleHog&style=flat-square&logo=github&logoColor=white" alt="TruffleHog"></a>
    <a href="https://github.com/nakisisageorge/cua-africa-enhanced/actions"><img src="https://img.shields.io/github/actions/workflow/status/nakisisageorge/cua-africa-enhanced/ci-sbom.yml?label=SBOM&style=flat-square&logo=github&logoColor=white" alt="SBOM"></a>
    <a href="https://github.com/nakisisageorge/cua-africa-enhanced/actions"><img src="https://img.shields.io/github/actions/workflow/status/nakisisageorge/cua-africa-enhanced/ci-govulncheck.yml?label=govulncheck&style=flat-square&logo=go&logoColor=white" alt="govulncheck"></a>
    <a href="https://github.com/nakisisageorge/cua-africa-enhanced/actions"><img src="https://img.shields.io/github/actions/workflow/status/nakisisageorge/cua-africa-enhanced/ci-dependency-review.yml?label=Dependency%20Review&style=flat-square&logo=github&logoColor=white" alt="Dependency Review"></a>
    <br>
    <img src="https://img.shields.io/badge/License-MIT-0A66C2?style=flat-square" alt="MIT License">
    <img src="https://img.shields.io/badge/Mode-B%20Enhancement-F59E0B?style=flat-square" alt="Mode B Enhancement">
    <img src="https://img.shields.io/badge/African%20Tech-🇺🇬-10B981?style=flat-square" alt="African Technology">
    <img src="https://img.shields.io/badge/Author-George%20Nakisisa-7C3AED?style=flat-square" alt="Author">
    <img src="https://img.shields.io/badge/Company-GICT%20Africa-F59E0B?style=flat-square" alt="Company">
  </p>

  <p align="center">
    <a href="https://nakisisageorge.github.io/cua-africa-enhanced/architecture/"><strong>Interactive Architecture Visualization</strong></a> ·
    <a href="DESIGN_DOCUMENT.md"><strong>Design Document</strong></a> ·
    <a href="BEFORE_AFTER.md"><strong>Before vs After</strong></a> ·
    <a href="PORTFOLIO_CASE_STUDY.md"><strong>Portfolio Case Study</strong></a> ·
    <a href="MCP_GUIDE.md"><strong>MCP Server Guide</strong></a> ·
    <a href="AFRICAN_SANDBOX_GUIDE.md"><strong>African Sandbox Guide</strong></a>
  </p>
</div>

---

## Overview

**cua-africa-enhanced** is a substantial enhancement fork of [trycua/cua](https://github.com/trycua/cua) (Computer-Use Agents 2.0), adding African technology domain support, Model Context Protocol (MCP) server for agent orchestration, benchmark regression detection, and comprehensive documentation — all built to senior engineering standards.

This project demonstrates **Senior Software Engineering**, **AI Engineering**, **Cybersecurity**, and **African Technology** expertise through meaningful, production-ready improvements.

### What is Cua?

Cua provides computer-use capabilities for AI agents: desktop automation (Cua Driver), sandboxed environments (Cua SDK/Spaces), local VMs (Lume), decision models (CUA-S1), and evaluation benchmarks (Cua Bench). See the [original project](https://github.com/trycua/cua) for the core platform.

### Why This Enhancement Exists

The original Cua platform is excellent but lacks:
- **African domain sandbox environments** for testing mobile money, fintech, and healthcare applications
- **MCP server** for standardizing agent-tool communication across sandboxes, skills, and benchmarks
- **Benchmark regression detection** to catch performance regressions in computer-use agents
- **Interactive architecture visualization** for understanding the complex multi-crate workspace
- **Production-grade CI/CD** with supply chain security (cargo-audit, cargo-deny, TruffleHog, SBOM, govulncheck)
- **Security hardening** for sandbox isolation (seccomp, network isolation, audit logging)

This fork adds all of the above while maintaining full compatibility with the upstream platform.

---

## Major Improvements

| Area | Original (trycua/cua) | Enhanced (cua-africa-enhanced) |
|------|----------------------|--------------------------------|
| **African Sandbox Profiles** | None | 9 profiles: M-Pesa Daraja, MTN MoMo, Airtel Money, Orange Money, Wave Money, NIBSS, PesaLink, BankservAfrica, OpenMRS |
| **African Language Skills** | None | 21 skill variants across 7 languages (Swahili, Luganda, Amharic, Hausa, Yoruba, Zulu, Xhosa) × 3 domains |
| **MCP Server** | None | `cua-mcp` crate: stdio + HTTP/SSE transports, 12 tools for sandbox/skill/benchmark orchestration |
| **Benchmark Regression** | Basic benchmarks | Criterion-style regression detection (5% threshold), SQLite/PostgreSQL persistence, 5 benchmark suites |
| **Architecture Visualization** | None | React Flow interactive diagram (90+ nodes, 50+ edges, 8 views) |
| **Documentation** | Standard README | 7 comprehensive docs + interactive tree (Design Doc, Case Study, Before/After, MCP Guide, Sandbox Guide, Skill Guide, Architecture Tree) |
| **CI/CD Security** | Standard Rust CI | cargo-audit, cargo-deny, TruffleHog, SBOM (CycloneDX), govulncheck, dependency review |
| **Sandbox Security** | Basic isolation | seccomp profiles, network policies, resource limits, audit logging |
| **License Compliance** | Mixed MIT/FSL | Clear attribution, all enhancements MIT-licensed |

---

## African Sandbox Profiles

Pre-configured sandbox environments for testing African digital services:

### Mobile Money (5 providers)
| Profile | Country | API | Use Case |
|---------|---------|-----|----------|
| `mpesa-daraja` | Kenya | Safaricom Daraja API | STK Push, B2C, C2B, Reversal |
| `mtn-momo` | Uganda/Ghana/Zambia | MTN MoMo API | Collections, Disbursements, Remittances |
| `airtel-money` | 14 African countries | Airtel Money API | Payments, Transfers, Wallet ops |
| `orange-money` | 17 countries | Orange Money API | Merchant payments, P2P, Bill pay |
| `wave-money` | Senegal, Côte d'Ivoire | Wave API | Mobile money, Savings, Credit |

### FinTech / Interbank (3 systems)
| Profile | Region | Purpose |
|---------|--------|---------|
| `nibss` | Nigeria | NIBSS Instant Payments, BVN validation |
| `pesalink` | Kenya | PesaLink interbank transfers |
| `bankservafrica` | South Africa | BankservAfrica EFT, Real-time clearing |

### Healthcare (1 system)
| Profile | Region | Purpose |
|---------|--------|---------|
| `openmrs` | Pan-African | OpenMRS medical records, FHIR integration |

### Quickstart
```bash
# Create a sandbox with M-Pesa Daraja profile
cua sb create --profile africa/mobile-money/mpesa-daraja --name mpesa-test

# Run a test transaction
cua sb exec mpesa-test python test_stk_push.py

# Clean up
cua sb rm mpesa-test
```

See [AFRICAN_SANDBOX_GUIDE.md](AFRICAN_SANDBOX_GUIDE.md) for complete setup scripts and configuration.

---

## African Language Skills

21 computer-use agent skills across 7 African languages × 3 domains:

| Language | Mobile Money | FinTech | Healthcare |
|----------|-------------|---------|------------|
| **Swahili** | `sw-mobile-money` | `sw-fintech` | `sw-healthcare` |
| **Luganda** | `lg-mobile-money` | `lg-fintech` | `lg-healthcare` |
| **Amharic** | `am-mobile-money` | `am-fintech` | `am-healthcare` |
| **Hausa** | `ha-mobile-money` | `ha-fintech` | `ha-healthcare` |
| **Yoruba** | `yo-mobile-money` | `yo-fintech` | `yo-healthcare` |
| **Zulu** | `zu-mobile-money` | `zu-fintech` | `zu-healthcare` |
| **Xhosa** | `xh-mobile-money` | `xh-fintech` | `xh-healthcare` |

Skills enable agents to navigate African-language UIs, validate local identifiers, and interact with region-specific applications.

See [SKILL_DEVELOPMENT_GUIDE.md](SKILL_DEVELOPMENT_GUIDE.md) for creating custom skills.

---

## MCP Server (`cua-mcp`)

Model Context Protocol server exposing 12 tools for agent orchestration:

| Tool | Transport | Description |
|------|-----------|-------------|
| `sandbox_create` | stdio, HTTP/SSE | Create sandbox with optional African profile |
| `sandbox_exec` | stdio, HTTP/SSE | Execute command in sandbox |
| `sandbox_screenshot` | stdio, HTTP/SSE | Capture sandbox screenshot |
| `sandbox_list` | stdio, HTTP/SSE | List active sandboxes |
| `skill_list` | stdio, HTTP/SSE | List available African language skills |
| `skill_install` | stdio, HTTP/SSE | Install skill into sandbox |
| `benchmark_run` | stdio, HTTP/SSE | Run Cua Bench evaluation |
| `benchmark_regression_check` | stdio, HTTP/SSE | Check for performance regressions |
| `profile_list` | stdio, HTTP/SSE | List African sandbox profiles |
| `profile_get` | stdio, HTTP/SSE | Get profile configuration |
| `architecture_visualize` | HTTP/SSE | Stream architecture visualization data |
| `health_check` | stdio, HTTP/SSE | Server health and readiness |

### Quickstart (stdio)
```bash
cargo run -p cua-mcp -- --transport stdio
```

### Quickstart (HTTP/SSE)
```bash
cargo run -p cua-mcp -- --transport http --port 3212
```

See [MCP_GUIDE.md](MCP_GUIDE.md) for full API reference, authentication, and integration examples.

---

## Benchmark Regression Detection

Criterion-style statistical regression detection for computer-use benchmarks:

- **5 benchmark suites**: `cua-bench`, `cua-bench-rl`, `cua-bench-s1`, `cua-driver-bench`, `cua-spaces-bench`
- **5% threshold** for detecting statistically significant regressions
- **SQLite/PostgreSQL** persistence for historical comparisons
- **CI integration** via `cargo bench --bench regression`
- **GitHub Actions** workflow for automated regression reporting

```bash
# Run regression detection locally
cargo bench -p cua-bench-regression --bench regression_check

# View historical trends
cargo run -p cua-bench-regression -- trend --metric duration --days 30
```

---

## Interactive Architecture Visualization

React Flow-based interactive architecture explorer at `docs/architecture/`:

- **90+ nodes** across 8 architectural views
- **50+ edges** showing data flow and dependencies
- **Views**: Overview, Sandbox Layer, MCP Server, Benchmark System, Skills System, Security, Deployment, Development
- **Interactive**: Expand/collapse, search, filter, node details, relationship tracing
- **Responsive**: Works on desktop and mobile

```bash
cd docs/architecture
npm install
npm run dev  # Local development
npm run build  # Production build to docs/architecture/dist
```

Deploy to GitHub Pages: `https://nakisisageorge.github.io/cua-africa-enhanced/architecture/`

See [INTERACTIVE_TREE.md](INTERACTIVE_TREE.md) for the documentation tree structure.

---

## Documentation Suite

| Document | Purpose |
|----------|---------|
| [DESIGN_DOCUMENT.md](DESIGN_DOCUMENT.md) | Architecture decisions, technology choices, security model, scalability |
| [BEFORE_AFTER.md](BEFORE_AFTER.md) | Detailed engineering comparison (original vs enhanced) |
| [PORTFOLIO_CASE_STUDY.md](PORTFOLIO_CASE_STUDY.md) | Professional case study for portfolio |
| [MCP_GUIDE.md](MCP_GUIDE.md) | MCP server API reference, authentication, examples |
| [AFRICAN_SANDBOX_GUIDE.md](AFRICAN_SANDBOX_GUIDE.md) | Sandbox profiles, setup scripts, configuration |
| [SKILL_DEVELOPMENT_GUIDE.md](SKILL_DEVELOPMENT_GUIDE.md) | Creating custom computer-use skills |
| [INTERACTIVE_TREE.md](INTERACTIVE_TREE.md) | Interactive documentation tree structure |

---

## Installation

### Prerequisites
- Rust 1.75+ (`rustup`)
- Node.js 20+ (for architecture visualization)
- Python 3.12+ (for sandbox SDK)
- Docker (for gVisor containers)
- macOS 26+ / Linux (for Lume VMs)

### Build from Source
```bash
git clone https://github.com/nakisisageorge/cua-africa-enhanced.git
cd cua-africa-enhanced

# Build Rust workspace (includes cua-mcp, cua-bench-regression)
cargo build --release --workspace

# Build architecture visualization
cd docs/architecture && npm install && npm run build
```

### Using Pre-built Binaries
See [GitHub Releases](https://github.com/nakisisageorge/cua-africa-enhanced/releases) for platform-specific binaries.

---

## Configuration

### Environment Variables
```bash
# MCP Server
CUA_MCP_TRANSPORT=http  # or stdio
CUA_MCP_PORT=3212
CUA_MCP_HOST=0.0.0.0
CUA_MCP_AUTH_TOKEN=<optional-bearer-token>

# Sandbox Profiles
CUA_PROFILE_DIR=./libs/cua/sandbox-profiles/africa

# Benchmark Regression
CUA_BENCH_DB_URL=sqlite://benchmark.db  # or postgresql://...
CUA_BENCH_REGRESSION_THRESHOLD=0.05

# Security
CUA_SECCOMP_PROFILE=./security/seccomp/default.json
CUA_NETWORK_POLICY=./security/network/deny-all.yaml
```

See individual component READMEs for complete configuration options.

---

## Usage Examples

### Run African Mobile Money Test in Sandbox
```python
from cua_sandbox import Image, Sandbox

async with Sandbox.ephemeral(Image.linux(), local=True, profile="africa/mobile-money/mpesa-daraja") as sb:
    # Install test dependencies
    await sb.shell.run("pip install mpesa-daraja-sdk")
    
    # Run STK Push test
    result = await sb.shell.run("python test_stk_push.py --amount 100 --phone 2547xxxxxxxx")
    print(result.stdout)
```

### Connect Agent via MCP (stdio)
```python
import asyncio
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client

async def main():
    server = StdioServerParameters(command="cua-mcp", args=["--transport", "stdio"])
    async with stdio_client(server) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()
            
            # Create sandbox with M-Pesa profile
            result = await session.call_tool("sandbox_create", {
                "profile": "africa/mobile-money/mpesa-daraja",
                "name": "agent-test"
            })
            print(result)

asyncio.run(main())
```

### Check Benchmark Regression
```bash
# Run full benchmark suite with regression detection
cargo bench -p cua-bench-regression --bench regression_check -- --threshold 0.05

# Output: JSON report with regression status per benchmark
```

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        cua-africa-enhanced                      │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────┐  │
│  │ Cua Driver   │  │ Cua SDK/CLI  │  │ Cua Spaces (Desktop) │  │
│  │ (Automation) │  │ (Sandboxes)  │  │ (Multiplayer VMs)    │  │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬────────────┘  │
│         │                 │                      │               │
│         └─────────────────┼──────────────────────┘               │
│                           ▼                                      │
│              ┌────────────────────────┐                          │
│              │   cua-mcp (MCP Server) │◄── stdio / HTTP/SSE      │
│              │  12 orchestration tools│                          │
│              └───────────┬────────────┘                          │
│                          │                                        │
│         ┌────────────────┼────────────────┐                      │
│         ▼                ▼                ▼                      │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐                │
│  │ Sandbox     │ │ Skills      │ │ Benchmark   │                │
│  │ Profiles    │ │ (21 African │ │ Regression  │                │
│  │ (9 African) │ │  languages) │ │ (5 suites)  │                │
│  └─────────────┘ └─────────────┘ └─────────────┘                │
│         │                │                │                      │
│         ▼                ▼                ▼                      │
│  ┌─────────────────────────────────────────────┐               │
│  │         Security Hardening Layer            │               │
│  │  seccomp │ network isolation │ audit logging │               │
│  └─────────────────────────────────────────────┘               │
└─────────────────────────────────────────────────────────────────┘
```

See [Interactive Architecture](https://nakisisageorge.github.io/cua-africa-enhanced/architecture/) for explorable diagram.

---

## Security

### Hardening Measures
- **seccomp-bpf** profiles for sandbox syscall filtering
- **Network policies** (default deny, explicit allowlists per profile)
- **Resource limits** (CPU, memory, disk, pids via cgroups)
- **Audit logging** for all sandbox operations
- **No privileged containers** — gVisor/user-mode VMs only

### Supply Chain Security
- `cargo-audit` — vulnerability scanning (RustSec database)
- `cargo-deny` — license, bans, sources, advisories
- `TruffleHog` — secret detection in code and history
- `SBOM` — CycloneDX generation for all crates
- `govulncheck` — Go vulnerability scanning (for tooling)
- `dependency-review` — PR-time dependency analysis

### Responsible Disclosure
See [SECURITY.md](.github/SECURITY.md) for vulnerability reporting.

---

## Testing

```bash
# Unit tests (all crates)
cargo test --workspace

# Integration tests (requires Docker)
cargo test --workspace --test integration

# Benchmark regression tests
cargo bench -p cua-bench-regression

# Architecture visualization tests
cd docs/architecture && npm test

# Security scans
cargo audit
cargo deny check
trufflehog git file://. --json
```

### Test Status
| Suite | Status |
|-------|--------|
| Unit Tests | ✅ Passing |
| Integration Tests | ✅ Passing (CI) |
| Benchmark Regression | ✅ Passing |
| Security Scans | ✅ Clean |
| Architecture Viz | ✅ Build passing |

---

## Deployment

### Docker (Sandbox Runtime)
```dockerfile
# Base sandbox image with cua-spacesd
FROM ghcr.io/nakisisageorge/cua-sandbox:latest

# Add African profiles
COPY libs/cua/sandbox-profiles/africa /profiles/africa
```

### Kubernetes (Operator)
See `deploy/kubernetes/` for operator manifests (planned).

### GitHub Pages (Documentation)
Architecture visualization auto-deploys via `.github/workflows/docs-deploy.yml` on main branch pushes.

---

## Contributing

This is a portfolio project demonstrating engineering capabilities. Issues and PRs welcome for:
- Additional African sandbox profiles
- New language skills
- MCP tool extensions
- Benchmark improvements
- Documentation enhancements

See [CONTRIBUTING.md](.github/CONTRIBUTING.md) for guidelines.

---

## License

This enhancement fork is **MIT-licensed** (see [LICENSE.md](LICENSE.md)).

**Attribution:**
- Original Cua platform: Copyright (c) Cua AI, Inc. — MIT/FSL-1.1-MIT licensed
- Enhancements by George Nakisisa / GICT Africa Technologies — MIT licensed
- All African sandbox profiles, skills, MCP server, regression detection, architecture visualization, and documentation are original work

This fork complies with all upstream license obligations. FSL-1.1-MIT components (Cua Spaces, Cua Driver Bench) are not modified or redistributed.

---

## Author

**George Nakisisa**  
Senior Software Engineer · AI Engineer · Cybersecurity  
GICT Africa Technologies  
Kampala, Uganda 🇺🇬  

- GitHub: [@nakisisageorge](https://github.com/nakisisageorge)
- Website: [gictafrica.com](https://gictafrica.com)
- Email: nakisisageorge@gmail.com
- X: [@gictafrica](https://x.com/gictafrica)

---

## Acknowledgments

- [trycua/cua](https://github.com/trycua/cua) — Original Computer-Use 2.0 platform
- African mobile money providers (Safaricom, MTN, Airtel, Orange, Wave) for API documentation
- OpenMRS community for healthcare interoperability standards
- Criterion.rs for benchmark statistics methodology
- React Flow for architecture visualization framework