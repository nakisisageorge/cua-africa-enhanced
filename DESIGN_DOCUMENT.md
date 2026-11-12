# DESIGN_DOCUMENT.md — cua-enhanced

## 1. Original Architecture

The original trycua/cua repository is a comprehensive computer-use automation platform organized as a Rust workspace with 90+ crates, plus Python (cua-bench), TypeScript (bindings), and Swift (macOS apps) components.

### Core Components

**SDK Layer (libs/cua/)**
- `cua-sdk`: Core SDK for sandbox management, local runtimes, images, Spaces
- `cua-cli`: Command-line interface (`cua` command)
- `cua-daemon`: Background daemon for shared SDK instance via gRPC
- Language bindings: Python (`cua`), TypeScript (`@trycua/cua`), Swift (WIP)

**Cua Driver (libs/cua-driver/)**
- Cross-platform desktop automation for macOS, Windows, Linux
- Native accessibility APIs for screen, input, windows
- Background delivery (agents work without stealing focus)
- MCP server integration for agent connectivity

**Cua Spaces (apps/cua-spaces/, libs/cua-spacesd/)**
- Desktop app (Tauri + SwiftUI) for managing Spaces
- `cua-spacesd`: In-sandbox daemon on port 3211
- `cua-keyvault`: Encrypted session storage for teleport
- `cua-volume`: Versioned shared drive across Spaces

**Virtualization (libs/lume/, libs/cua-vmm/, libs/cua-sandbox-core/)**
- Lume: macOS/Linux VMs on Apple Silicon via Virtualization.framework
- VMM abstraction: Firecracker, Virtualization.framework, QEMU
- Sandbox core: gVisor, runc, QEMU, Lume runtimes
- BYOC: AWS, GCP, Modal cloud providers

**Benchmarks & ML (libs/cua-bench/, libs/cua-s1/)**
- cua-bench: Task creation, evaluation, trajectory export (ATIF)
- cua-bench-rl: RL training environments
- CUA-S1: System 1 models for computer-use decisions

**Media & Cluster (libs/cua-media/, libs/cua-cluster/)**
- QUIC-based media plane for low-latency streaming
- Multi-machine cluster: mesh (WireGuard), relay, coordinator

### Licensing
- MIT: Core SDK, CLI, Driver, Benchmarks, Virtualization
- FSL-1.1-MIT: Cua Spaces, cua-driver-bench (becomes MIT after 2 years)
- AGPL-3.0: cua-perception (OmniParser extension)

## 2. Problems Identified

### 2.1 No African Market Support
- No pre-configured sandbox profiles for African mobile money (M-Pesa, MTN MoMo, Airtel, Orange, Wave)
- No FinTech integration profiles (NIBSS, PesaLink, BankservAfrica)
- No Healthcare system profiles (OpenMRS, DHIS2)
- No African language localization in skills system

### 2.2 No MCP Integration
- Agents cannot orchestrate Cua via Model Context Protocol
- No stdio/HTTP/SSE transport for Claude Code, Codex, etc.
- No tool exposure for sandbox creation, skill invocation, benchmark execution

### 2.3 No Regression Detection
- cua-bench lacks automated performance regression detection
- No historical metric tracking across commits
- No CI integration for regression alerts

### 2.4 No Architecture Visualization
- 90+ crate workspace has no visual architecture explorer
- New contributors struggle to understand component relationships
- No interactive documentation for onboarding

### 2.5 Documentation Gaps
- No DESIGN_DOCUMENT.md explaining architecture decisions
- No PORTFOLIO_CASE_STUDY.md for portfolio demonstration
- No BEFORE_AFTER.md comparison
- No interactive documentation tree
- No MCP server guide
- No African sandbox/skill development guide

### 2.6 CI/CD Missing Supply Chain Security
- No cargo-audit for vulnerability scanning
- No cargo-deny for license/duplicate checking
- No TruffleHog for secret scanning
- No SBOM generation (CycloneDX)
- No govulncheck for Go components
- No dependency review automation

### 2.7 Sandbox Security Hardening Needed
- Default resource limits not enforced
- Network egress not restricted by default
- No seccomp profiles for high-risk tasks
- No audit logging for sandbox operations

## 3. Proposed Architecture

### 3.1 African Sandbox Profiles (NEW)
```
libs/cua/sandbox-profiles/africa/
├── mobile-money/
│   ├── mpesa-daraja.yaml      # Kenya M-Pesa
│   ├── mtn-momo.yaml          # Ghana/Uganda MTN MoMo
│   ├── airtel-money.yaml      # Nigeria/Uganda Airtel
│   ├── orange-money.yaml      # CI/Senegal Orange
│   └── wave-money.yaml        # Senegal/CI Wave
├── fintech/
│   ├── nibss.yaml             # Nigeria NIBSS
│   ├── pesalink.yaml          # Kenya PesaLink
│   └── bankservafrica.yaml    # South Africa BankservAfrica
├── healthcare/
│   └── openmrs.yaml           # OpenMRS multi-country
└── scripts/
    ├── setup-mpesa-daraja.sh
    ├── setup-mtn-momo.sh
    └── setup-generic-mobile-money.sh
```

Each profile defines:
- Base image (ghcr.io/trycua/linux:latest)
- Setup script for simulator installation
- Port mappings (simulator HTTP/HTTPS + cua-spacesd 3211)
- Environment variables for API credentials
- Resource limits (CPU, memory, disk)
- Network allowlist for provider APIs
- Associated skills for automation

### 3.2 MCP Server (NEW: libs/cua/crates/cua-mcp/)
```
cua-mcp/
├── src/
│   ├── lib.rs              # Public API
│   ├── main.rs             # CLI entry point
│   ├── config.rs           # Configuration (Figment + TOML + Env)
│   ├── error.rs            # Error types
│   ├── server.rs           # MCP server handler
│   ├── tools.rs            # 12 tool implementations
│   ├── session.rs          # Session management
│   ├── regression.rs       # Regression tracking (SQLite)
│   └── prompts.rs          # MCP prompts
├── migrations/
│   └── 001_initial.sql     # Database schema
├── Cargo.toml
└── cua-mcp.toml.example    # Example config
```

**Tools Exposed:**
1. `cua.sandbox.create` - Create sandbox from African profile
2. `cua.sandbox.execute` - Run commands in sandbox
3. `cua.sandbox.screenshot` - Capture screen
4. `cua.sandbox.input` - Send mouse/keyboard input
5. `cua.skill.list` - List available skills
6. `cua.skill.invoke` - Invoke skill with parameters
7. `cua.benchmark.run` - Run cua-bench task/dataset
8. `cua.benchmark.regression` - Check performance regression
9. `cua.profile.list` - List sandbox profiles
10. `cua.profile.get` - Get profile details
11. `cua.sandbox.destroy` - Destroy sandbox
12. `cua.sandbox.status` - Get sandbox status

**Transports:**
- stdio (for local agents: Claude Code, Codex)
- HTTP/SSE (for remote orchestration)
- mTLS support for production

### 3.3 Benchmark Regression Detection (NEW: libs/cua/crates/cua-bench-regression/)
```
cua-bench-regression/
├── src/
│   ├── lib.rs              # Public API
│   ├── tracker.rs          # RegressionTracker with SQLite
│   ├── criterion.rs        # Criterion-style benchmarks
│   └── cli.rs              # CLI for regression check
├── Cargo.toml
```

**Metrics Tracked:**
- Task success rate
- Steps to completion
- Token usage (for AI agents)
- Latency (ms)
- Screenshot similarity (perceptual hash)

**Thresholds:** 5% regression threshold (configurable per metric)

**Storage:** Local SQLite + optional PostgreSQL for CI

**GitHub Action:** `ci-cua-bench-regression.yml` runs on PR and scheduled

### 3.4 Architecture Visualization (NEW: docs/architecture/)
React Flow-based interactive explorer:
- 90+ nodes grouped by 9 domains
- 50+ edges showing dependencies, data flow, control flow
- 8 views: Overview, SDK, Driver, Spaces, Virtualization, Benchmarks, Infrastructure, African Extensions
- Expandable category groups
- Search and filter
- Node detail tooltip on hover
- Static HTML export for GitHub Pages

### 3.5 African Language Skills (NEW: skills/african-*/)
```
skills/
├── african-mobile-money/
│   ├── mpesa-sw/            # M-Pesa in Swahili
│   ├── mpesa-en/            # M-Pesa in English
│   ├── mtn-momo-en/
│   ├── mtn-momo-tw/         # Twi
│   ├── airtel-en/
│   ├── airtel-ha/           # Hausa
│   ├── airtel-yo/           # Yoruba
│   ├── orange-fr/           # French
│   ├── orange-wo/           # Wolof
│   ├── wave-fr/
│   └── wave-wo/
├── african-fintech/
│   ├── nibss-en/
│   ├── pesalink-sw/
│   ├── pesalink-en/
│   ├── bankservafrica-en/
│   └── bankservafrica-af/   # Afrikaans
└── african-healthcare/
    ├── openmrs-en/
    └── openmrs-sw/
```

Each skill includes:
- Language-specific UI element detection
- USSD menu navigation patterns
- SMS-based flow handling
- Low-bandwidth optimization
- Cultural adaptation (currency formats, date formats, etc.)

### 3.6 Security Hardening
**Sandbox Profiles:**
```yaml
resources:
  cpu: "2"
  memory: "4G"
  disk: "20G"
  network: "isolated"
  allowlist:
    - "api.safaricom.co.ke"
    - "github.com"
    - "crates.io"
security:
  seccomp_profile: "default"
  capabilities: "drop=ALL"
  user_namespace: true
  read_only_rootfs: false
  no_new_privileges: true
```

**Audit Logging:** All sandbox operations logged with correlation IDs

## 4. New Architecture

### Component Diagram (Textual)

```
┌─────────────────────────────────────────────────────────────────────┐
│                         AI Agents (Claude, Codex)                   │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │ MCP (stdio/HTTP/SSE)
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        cua-mcp Server                               │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────┐  │
│  │ Sandbox Ops  │ │ Skill Ops    │ │ Benchmark    │ │ Profile  │  │
│  │ create/exec  │ │ list/invoke  │ │ run/regress  │ │ list/get │  │
│  └──────────────┘ └──────────────┘ └──────────────┘ └──────────┘  │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │ cua-sdk
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    cua-sdk (Core Orchestration)                     │
│  ┌───────────┐ ┌───────────┐ ┌───────────┐ ┌───────────────────┐  │
│  │ Sandbox   │ │ Image     │ │ Daemon    │ │ Connections/      │  │
│  │ Core      │ │ Mgmt      │ │ (gRPC)    │ │ BYOC Providers    │  │
│  └───────────┘ └───────────┘ └───────────┘ └───────────────────┘  │
└─────────────────────────────────┬───────────────────────────────────┘
                                  │
          ┌───────────────────────┼───────────────────────┐
          ▼                       ▼                       ▼
┌─────────────────────┐ ┌─────────────────────┐ ┌─────────────────────┐
│   Virtualization    │ │    Cua Driver       │ │    Cua Spaces       │
│   (Lume, VMM,       │ │   (Desktop Auto)    │ │   (Spaces App,      │
│    gVisor, QEMU,    │ │   macOS/Win/Linux   │ │    spacesd,         │
│    BYOC: AWS/GCP)   │ │   Accessibility     │ │    Keyvault,        │
│                     │ │                     │ │    Volume)          │
└─────────────────────┘ └─────────────────────┘ └─────────────────────┘
          │                       │                       │
          ▼                       ▼                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    African Extensions Layer                         │
│  ┌────────────────┐ ┌────────────────┐ ┌────────────────┐          │
│  │ Sandbox        │ │ Skills         │ │ Regression       │          │
│  │ Profiles       │ │ (7 languages ×  │ │ Detection        │          │
│  │ (9 profiles)   │ │  3 domains)    │ │ (criterion)      │          │
│  └────────────────┘ └────────────────┘ └────────────────┘          │
└─────────────────────────────────────────────────────────────────────┘
```

## 5. Technology Decisions

| Decision | Rationale |
|----------|-----------|
| **Rust for cua-mcp** | Consistent with workspace, memory safety, performance, UniFFI for bindings |
| **rmcp (MCP SDK)** | Official Rust MCP SDK, supports stdio/HTTP/SSE, macros for tool definition |
| **SQLite for regression** | Embedded, zero-config, sufficient for local + CI; PostgreSQL optional for scale |
| **Figment for config** | Layered config (TOML + Env), used elsewhere in workspace |
| **React Flow for viz** | Purpose-built for node/edge graphs, interactive, extensible, TypeScript native |
| **Criterion for benchmarks** | Industry standard, statistical rigor, HTML reports, regression detection |
| **YAML for profiles** | Human-readable, widely supported, schema-validatable |
| **Shell for setup scripts** | Portable, runs in container init, simple simulator installation |

## 6. Security Decisions

| Decision | Rationale |
|----------|-----------|
| **Network isolation by default** | Sandboxes cannot access internet unless explicitly allowed |
| **Resource limits enforced** | Prevents resource exhaustion, noisy neighbor problem |
| **mTLS for HTTP transport** | Mutual authentication for production MCP servers |
| **API key + rate limiting** | Simple auth for development, extensible for production |
| **Seccomp profiles** | Reduces kernel attack surface in containers |
| **User namespace mapping** | Root in container ≠ root on host |
| **Audit logging with correlation IDs** | Enables forensic analysis, compliance |
| **Secret scanning in CI** | Prevents credential leaks (TruffleHog) |
| **SBOM generation** | Supply chain transparency, vulnerability tracking |

## 7. Performance Decisions

| Decision | Rationale |
|----------|-----------|
| **Criterion benchmarks** | Statistical rigor, outlier detection, regression detection built-in |
| **SQLite for regression DB** | Zero-config, fast enough for thousands of runs |
| **React Flow virtualization** | Handles 100+ nodes smoothly |
| **gVisor for sandbox isolation** | Better security/performance balance than KVM for containers |
| **Caching in cua-daemon** | Shared SDK instance reduces cold starts |
| **Incremental builds (dev-fast profile)** | Seconds for single-file changes |

## 8. Scalability Considerations

- **Horizontal scaling**: Multiple cua-mcp instances behind load balancer
- **Session persistence**: Redis-backed session store for multi-instance
- **Database**: PostgreSQL for high-volume regression tracking
- **Sandbox pooling**: Pre-warmed sandbox pools for low-latency agent ops
- **CDN for architecture viz**: Static export to GitHub Pages + Cloudflare

## 9. Deployment Architecture

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   GitHub    │────▶│   GitHub    │────▶│  Container  │
│   Actions   │     │   Releases  │     │  Registry   │
└─────────────┘     └─────────────┘     └─────────────┘
                           │                     │
                           ▼                     ▼
                    ┌─────────────┐     ┌─────────────┐
                    │  GitHub     │     │  Docker Hub │
                    │  Pages      │     │  / ghcr.io  │
                    │ (arch viz)  │     │  (images)   │
                    └─────────────┘     └─────────────┘
```

## 10. Testing Strategy

| Layer | Tools | Coverage |
|-------|-------|----------|
| Unit | cargo test, pytest, vitest | 80%+ per crate |
| Integration | cua-bench live tests, cua-driver E2E | Critical paths |
| Contract | buf breaking (protobuf), rmcp compliance | API stability |
| Regression | cua-bench-regression, criterion | Performance |
| Security | cargo-audit, cargo-deny, TruffleHog | Supply chain |
| E2E | Cross-platform desktop matrix (Linux/macOS/Windows) | Real behavior |

## 11. Deployment Architecture

- **Rust crates**: Published to crates.io via Release Please
- **Python packages**: Published to PyPI via trusted publishing
- **TypeScript packages**: Published to npm via provenance
- **Binaries**: Signed with cosign/keyless, published to GitHub Releases
- **Installers**: macOS (.dmg/.pkg), Windows (.msi), Linux (.deb/.rpm/.apk)
- **Architecture viz**: Static export to GitHub Pages
- **Documentation**: Built with docs.rs / custom deploy

## 12. Future Improvements

1. **Real-time collaboration**: Operational transforms for multi-agent sandbox sharing
2. **GPU acceleration**: CUDA/Metal support for ML workloads in sandboxes
3. **WASM sandbox runtime**: Lightweight alternative to containers
4. **Advanced perception**: Integration with latest vision models (GPT-4V, Claude 3)
5. **Policy engine**: CEL/Rego-based sandbox policy enforcement
6. **Federation**: Cross-organization sandbox sharing with policy
7. **Mobile app**: React Native companion for iOS/Android agent monitoring
8. **Plugin marketplace**: Community sandbox profiles, skills, benchmarks

---

**Author:** George Nakisisa  
**Company:** GICT Africa Technologies  
**Website:** https://gictafrica.com  
**Date:** 2026-11-11