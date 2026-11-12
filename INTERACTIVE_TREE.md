# INTERACTIVE_TREE.md — cua-enhanced Documentation Tree

This document provides an interactive, expandable tree view of the complete cua-enhanced codebase and documentation. Click to expand/collapse sections in the [live architecture visualization](https://nakisisageorge.github.io/cua-enhanced/architecture/).

---

## 📁 Repository Root
```
cua-enhanced/
├── 📄 README.md                           # Project overview, quickstarts, badges
├── 📄 LICENSE.md                          # MIT License (core)
├── 📄 LICENSING.md                        # License breakdown (MIT + FSL-1.1-MIT)
├── 📄 CONTRIBUTING.md                     # Contribution guidelines
├── 📄 SECURITY.md                         # Security reporting
├── 📄 DESIGN_DOCUMENT.md                  # Architecture, decisions, security, scaling
├── 📄 PORTFOLIO_CASE_STUDY.md             # Problem, research, approach, results
├── 📄 BEFORE_AFTER.md                     # Detailed engineering comparison
├── 📄 INTERACTIVE_TREE.md                 # This file
├── 📄 MCP_GUIDE.md                        # MCP server setup, tools, auth
├── 📄 AFRICAN_SANDBOX_GUIDE.md            # Profile catalog, customization, CI
├── 📄 SKILL_DEVELOPMENT_GUIDE.md          # Creating African skills, testing
├── 📄 AGENTS.md                           # Agent guidance for contributors
├── 📄 IMPROVEMENT_PLAN.md                 # Project improvement plan
├── 📄 Makefile                            # Build automation
├── 📁 .github/
│   ├── 📁 workflows/                      # 200+ GitHub Actions workflows
│   │   ├── 📄 ci-cua-bench-regression.yml # NEW: Benchmark regression CI
│   │   ├── 📄 ci-cua-mcp.yml              # NEW: MCP server CI
│   │   ├── 📄 ci-cargo-audit.yml          # NEW: Vulnerability scanning
│   │   ├── 📄 ci-cargo-deny.yml           # NEW: License/dependency checks
│   │   ├── 📄 ci-trufflehog.yml           # NEW: Secret scanning
│   │   ├── 📄 ci-sbom.yml                 # NEW: SBOM generation
│   │   └── ... (existing workflows)
│   ├── 📄 CONTRIBUTING.md
│   ├── 📄 MAINTAINERS.md
│   └── 📄 SECURITY.md
├── 📁 apps/
│   ├── 📁 cua-spaces-macos/               # macOS Spaces app (SwiftUI)
│   ├── 📁 cua-spaces/                     # Shared Spaces core (Tauri)
│   └── 📁 cua-spaces-desktop/             # Linux/Windows desktop app
├── 📁 bin/                                # Binary entry points
│   ├── 📄 ci                              # Local CI runner
│   └── 📄 cua-mcp                         # NEW: MCP server binary
├── 📁 docs/
│   ├── 📁 architecture/                   # NEW: Interactive architecture viz
│   │   ├── 📄 index.html                  # Entry point
│   │   ├── 📄 package.json                # React Flow viz dependencies
│   │   ├── 📄 vite.config.ts              # Vite config
│   │   ├── 📄 tsconfig.json               # TypeScript config
│   │   ├── 📁 src/
│   │   │   ├── 📄 main.tsx                # React entry
│   │   │   ├── 📄 App.tsx                 # Main app component
│   │   │   ├── 📄 styles.css              # Styling
│   │   │   ├── 📁 components/
│   │   │   │   ├── 📄 CustomNode.tsx      # Custom node component
│   │   │   │   ├── 📄 CustomEdge.tsx      # Custom edge component
│   │   │   │   └── 📄 Sidebar.tsx         # Sidebar with filters
│   │   │   ├── 📁 hooks/
│   │   │   │   └── 📄 useStore.ts         # Zustand state management
│   │   │   └── 📁 data/
│   │   │       └── 📄 architecture.ts     # Architecture data (nodes, edges, views)
│   │   └── 📁 screenshots/                # Architecture viz screenshots
│   ├── 📁 contributing/                   # Contribution guides
│   └── 📁 cua-driver/                     # Driver documentation
├── 📁 libs/
│   ├── 📁 cua/                            # Main SDK workspace (90+ crates)
│   │   ├── 📄 Cargo.toml                  # Workspace manifest
│   │   ├── 📄 VERSION                     # Version (0.24.1)
│   │   ├── 📄 CHANGELOG.md                # Changelog
│   │   ├── 📄 README.md                   # SDK README
│   │   ├── 📁 crates/                     # 90+ crates
│   │   │   ├── 📁 cua-mcp/                # NEW: MCP server crate
│   │   │   │   ├── 📄 Cargo.toml
│   │   │   │   ├── 📁 src/
│   │   │   │   │   ├── 📄 lib.rs          # Public API
│   │   │   │   │   ├── 📄 main.rs         # CLI entry
│   │   │   │   │   ├── 📄 config.rs       # Configuration (Figment)
│   │   │   │   │   ├── 📄 error.rs        # Error types
│   │   │   │   │   ├── 📄 server.rs       # MCP server handler
│   │   │   │   │   ├── 📄 tools.rs        # 12 tool implementations
│   │   │   │   │   ├── 📄 session.rs      # Session management
│   │   │   │   │   ├── 📄 regression.rs   # Regression tracking (SQLite)
│   │   │   │   │   └── 📄 prompts.rs      # MCP prompts
│   │   │   │   ├── 📁 migrations/
│   │   │   │   │   └── 📄 001_initial.sql # Database schema
│   │   │   │   ├── 📄 cua-mcp.toml.example
│   │   │   │   └── 📁 tests/
│   │   │   ├── 📁 cua-bench-regression/   # NEW: Benchmark regression crate
│   │   │   │   ├── 📄 Cargo.toml
│   │   │   │   └── 📁 src/
│   │   │   │       ├── 📄 lib.rs
│   │   │   │       ├── 📄 tracker.rs      # RegressionTracker
│   │   │   │       ├── 📄 criterion.rs    # Criterion benchmarks
│   │   │   │       └── 📄 cli.rs          # CLI for regression check
│   │   │   ├── 📁 cua-sdk/                # Core SDK
│   │   │   ├── 📁 cua-cli/                # CLI
│   │   │   ├── 📁 cua-daemon/             # Background daemon
│   │   │   ├── 📁 cua-sandbox-core/       # Sandbox orchestration
│   │   │   ├── 📁 cua-spacesd/            # In-sandbox daemon
│   │   │   ├── 📁 cua-spacesd-client/     # Spacesd gRPC client
│   │   │   ├── 📁 cua-vmm/                # VMM abstraction
│   │   │   ├── 📁 lume/                   # macOS/Linux VM management
│   │   │   ├── 📁 cua-driver/             # Desktop automation
│   │   │   ├── 📁 cua-bench/              # Benchmark framework (Python)
│   │   │   └── ... (80+ more crates)
│   │   ├── 📁 sandbox-profiles/           # NEW: Sandbox profiles
│   │   │   └── 📁 africa/
│   │   │       ├── 📁 mobile-money/
│   │   │       │   ├── 📄 mpesa-daraja.yaml
│   │   │       │   ├── 📄 mtn-momo.yaml
│   │   │       │   ├── 📄 airtel-money.yaml
│   │   │       │   ├── 📄 orange-money.yaml
│   │   │       │   └── 📄 wave-money.yaml
│   │   │       ├── 📁 fintech/
│   │   │       │   ├── 📄 nibss.yaml
│   │   │       │   ├── 📄 pesalink.yaml
│   │   │       │   └── 📄 bankservafrica.yaml
│   │   │       ├── 📁 healthcare/
│   │   │       │   └── 📄 openmrs.yaml
│   │   │       └── 📁 scripts/
│   │   │           ├── 📄 setup-mpesa-daraja.sh
│   │   │           ├── 📄 setup-mtn-momo.sh
│   │   │           └── 📄 setup-generic-mobile-money.sh
│   │   ├── 📁 python/                     # Python bindings
│   │   ├── 📁 typescript/                 # TypeScript bindings
│   │   ├── 📁 swift/                      # Swift bindings
│   │   └── 📁 proto/                      # Protocol buffers
│   ├── 📁 cua-driver/                     # Driver workspace
│   ├── 📁 cua-bench/                      # Benchmark workspace (Python)
│   │   ├── 📄 pyproject.toml
│   │   └── 📁 cua_bench/                  # Benchmark core
│   ├── 📁 cua-spacesd/                    # Spaces daemon
│   ├── 📁 cua-s1/                         # CUA-S1 models
│   ├── 📁 images/                         # Container/VM images
│   └── 📁 skills/                         # Agent skills
│       ├── 📁 gui-automation/             # GUI automation skill
│       ├── 📁 jev-use/                    # Jev-Use decision loop
│       └── 📁 african-*/                  # NEW: African skills (21 variants)
│           ├── 📁 african-mobile-money/
│           │   ├── 📁 mpesa-sw/           # M-Pesa Swahili
│           │   ├── 📁 mpesa-en/           # M-Pesa English
│           │   ├── 📁 mtn-momo-en/        # MTN MoMo English
│           │   ├── 📁 mtn-momo-tw/        # MTN MoMo Twi
│           │   ├── 📁 airtel-en/          # Airtel English
│           │   ├── 📁 airtel-ha/          # Airtel Hausa
│           │   ├── 📁 airtel-yo/          # Airtel Yoruba
│           │   ├── 📁 orange-fr/          # Orange French
│           │   ├── 📁 orange-wo/          # Orange Wolof
│           │   ├── 📁 wave-fr/            # Wave French
│           │   └── 📁 wave-wo/            # Wave Wolof
│           ├── 📁 african-fintech/
│           │   ├── 📁 nibss-en/           # NIBSS English
│           │   ├── 📁 pesalink-sw/        # PesaLink Swahili
│           │   ├── 📁 pesalink-en/        # PesaLink English
│           │   ├── 📁 bankservafrica-en/  # BankservAfrica English
│           │   └── 📁 bankservafrica-af/  # BankservAfrica Afrikaans
│           └── 📁 african-healthcare/
│               ├── 📁 openmrs-en/         # OpenMRS English
│               └── 📁 openmrs-sw/         # OpenMRS Swahili
├── 📁 scripts/                            # Build/deploy scripts
├── 📁 tests/                              # Integration tests
├── 📁 examples/                           # Usage examples
└── 📁 config/                             # Configuration templates
```

---

## 📚 Documentation Navigation

### Core Documentation
- [README.md](README.md) — Project overview, badges, quickstarts
- [DESIGN_DOCUMENT.md](DESIGN_DOCUMENT.md) — Architecture, decisions, security, scaling
- [PORTFOLIO_CASE_STUDY.md](PORTFOLIO_CASE_STUDY.md) — Problem, research, approach, results
- [BEFORE_AFTER.md](BEFORE_AFTER.md) — Detailed engineering comparison
- [INTERACTIVE_TREE.md](INTERACTIVE_TREE.md) — This file

### Feature Guides
- [MCP_GUIDE.md](MCP_GUIDE.md) — MCP server setup, tools, authentication, examples
- [AFRICAN_SANDBOX_GUIDE.md](AFRICAN_SANDBOX_GUIDE.md) — Profile catalog, customization, CI integration
- [SKILL_DEVELOPMENT_GUIDE.md](SKILL_DEVELOPMENT_GUIDE.md) — Creating African skills, testing, publishing

### Architecture Visualization
- [Live Architecture Explorer](https://nakisisageorge.github.io/cua-enhanced/architecture/) — Interactive React Flow viz
- [Architecture Source](docs/architecture/) — Source code for visualization

---

## 🔧 Key Components Quick Reference

### New Crates (Enhanced)
| Crate | Path | Purpose |
|-------|------|---------|
| `cua-mcp` | `libs/cua/crates/cua-mcp/` | Model Context Protocol server |
| `cua-bench-regression` | `libs/cua/crates/cua-bench-regression/` | Benchmark regression detection |
| `cua-sandbox-profiles` | `libs/cua/sandbox-profiles/` | African sandbox profiles (data) |

### New Skills (African Extensions)
| Domain | Skills | Languages |
|--------|--------|-----------|
| Mobile Money | mpesa, mtn-momo, airtel, orange, wave | Sw, En, Tw, Ha, Yo, Fr, Wo |
| FinTech | nibss, pesalink, bankservafrica | En, Sw, Af |
| Healthcare | openmrs | En, Sw |

### New MCP Tools
| Tool | Description |
|------|-------------|
| `cua.sandbox.create` | Create sandbox from African profile |
| `cua.sandbox.execute` | Execute commands in sandbox |
| `cua.sandbox.screenshot` | Capture sandbox screen |
| `cua.sandbox.input` | Send mouse/keyboard input |
| `cua.skill.list` | List available skills |
| `cua.skill.invoke` | Invoke skill with parameters |
| `cua.benchmark.run` | Run cua-bench task/dataset |
| `cua.benchmark.regression` | Check performance regression |
| `cua.profile.list` | List sandbox profiles |
| `cua.profile.get` | Get profile details |
| `cua.sandbox.destroy` | Destroy sandbox |
| `cua.sandbox.status` | Get sandbox status |

---

## 🏗️ Architecture Views (Live Viz)

| View | Components | URL Fragment |
|------|------------|--------------|
| **Overview** | All 90+ crates | `#overview` |
| **SDK** | Core, Bindings, CLI, Daemon | `#sdk` |
| **Driver** | Driver, Bench, Perception | `#driver` |
| **Spaces** | Spaces, Spacesd, Keyvault, Volume | `#spaces` |
| **Virtualization** | Lume, VMM, Sandbox Core, BYOC | `#virt` |
| **Benchmarks** | cua-bench, CUA-S1, RL | `#bench` |
| **Infrastructure** | Media, Cluster, Security, Observability | `#infra` |
| **African Extensions** | MCP, Sandboxes, Skills, Regression, Viz | `#africa` |

---

## 🚀 Quick Links

- **GitHub Repository**: https://github.com/nakisisageorge/cua-enhanced
- **Architecture Visualization**: https://nakisisageorge.github.io/cua-enhanced/architecture/
- **MCP Server Docs**: [MCP_GUIDE.md](MCP_GUIDE.md)
- **African Sandbox Guide**: [AFRICAN_SANDBOX_GUIDE.md](AFRICAN_SANDBOX_GUIDE.md)
- **Skill Development**: [SKILL_DEVELOPMENT_GUIDE.md](SKILL_DEVELOPMENT_GUIDE.md)
- **Issue Tracker**: https://github.com/nakisisageorge/cua-enhanced/issues
- **Discussions**: https://github.com/nakisisageorge/cua-enhanced/discussions

---

## 📝 Contributing to Documentation

1. **Architecture changes**: Update `docs/architecture/src/data/architecture.ts`
2. **New profiles**: Add YAML to `libs/cua/sandbox-profiles/africa/` + setup script
3. **New skills**: Create skill directory under `libs/skills/african-*/`
4. **Documentation**: Edit relevant `.md` file in repository root
5. **CI/CD**: Add workflow to `.github/workflows/`

---

*Generated as part of cua-enhanced distribution. Last updated: 2026-11-11*