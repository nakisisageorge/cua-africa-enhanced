// Architecture data for Cua Enhanced visualization

export interface NodeData {
  id: string;
  name: string;
  category: string;
  description: string;
  crates: string[];
  language: string;
  license: string;
  tags: string[];
  color: string;
  position: { x: number; y: number };
}

export interface EdgeData {
  id: string;
  source: string;
  target: string;
  type: 'dependency' | 'data-flow' | 'control-flow' | 'extension';
  label?: string;
}

export const architectureNodes: NodeData[] = [
  // SDK Core
  {
    id: 'cua-sdk',
    name: 'cua-sdk',
    category: 'SDK Core',
    description: 'Core Rust SDK for sandbox management, local runtimes, images, and Spaces integration. UniFFI exports for language bindings.',
    crates: ['cua-sdk', 'cua-sdk-native', 'cua-bindgen'],
    language: 'Rust',
    license: 'MIT',
    tags: ['core', 'bindings', 'uniffi'],
    color: '#58a6ff',
    position: { x: 100, y: 100 },
  },
  {
    id: 'cua-cli',
    name: 'cua-cli',
    category: 'SDK Core',
    description: 'Command-line interface for sandbox operations, Spaces management, authentication, and agent setup.',
    crates: ['cua-cli'],
    language: 'Rust',
    license: 'MIT',
    tags: ['cli', 'user-facing'],
    color: '#58a6ff',
    position: { x: 100, y: 250 },
  },
  {
    id: 'cua-daemon',
    name: 'cua-daemon',
    category: 'SDK Core',
    description: 'Background daemon providing shared cua-sdk instance via gRPC/gRPC-Web. Handles routing, WebSocket bridge, and auth.',
    crates: ['cua-daemon-core', 'cua-daemon'],
    language: 'Rust',
    license: 'MIT',
    tags: ['daemon', 'grpc', 'shared'],
    color: '#58a6ff',
    position: { x: 100, y: 400 },
  },
  {
    id: 'cua-python',
    name: 'cua-python',
    category: 'SDK Bindings',
    description: 'Python bindings for cua-sdk via UniFFI. Provides Sandbox, Image, Pool APIs for Python agents.',
    crates: ['cua/python'],
    language: 'Python',
    license: 'MIT',
    tags: ['bindings', 'python', 'pypi'],
    color: '#58a6ff',
    position: { x: -300, y: 100 },
  },
  {
    id: 'cua-typescript',
    name: '@trycua/cua',
    category: 'SDK Bindings',
    description: 'TypeScript/JavaScript bindings for Node.js and browser. Includes @trycua/cua/spaces for Spaces integration.',
    crates: ['cua/typescript'],
    language: 'TypeScript',
    license: 'MIT',
    tags: ['bindings', 'typescript', 'npm'],
    color: '#58a6ff',
    position: { x: -300, y: 250 },
  },
  {
    id: 'cua-swift',
    name: 'Cua (Swift)',
    category: 'SDK Bindings',
    description: 'Swift bindings via SwiftPM and XCFramework. Not yet released.',
    crates: ['cua/swift'],
    language: 'Swift',
    license: 'MIT',
    tags: ['bindings', 'swift', 'spm'],
    color: '#58a6ff',
    position: { x: -300, y: 400 },
  },

  // Cua Driver
  {
    id: 'cua-driver',
    name: 'cua-driver',
    category: 'Cua Driver',
    description: 'Background computer-use agent for macOS, Windows, Linux. Provides screen, input, windows via native accessibility APIs.',
    crates: ['cua-driver (libs/cua-driver/rust)', 'cua-driver (libs/cua-driver/python)'],
    language: 'Rust/Python',
    license: 'FSL-1.1-MIT',
    tags: ['desktop', 'automation', 'accessibility'],
    color: '#a371f7',
    position: { x: 500, y: 100 },
  },
  {
    id: 'cua-driver-bench',
    name: 'cua-driver-bench',
    category: 'Cua Driver',
    description: 'Benchmark suite for Cua Driver. Measures latency, accuracy, and performance across platforms.',
    crates: ['cua-driver-bench'],
    language: 'Rust',
    license: 'FSL-1.1-MIT',
    tags: ['benchmark', 'performance'],
    color: '#a371f7',
    position: { x: 500, y: 250 },
  },
  {
    id: 'cua-perception',
    name: 'cua-perception',
    category: 'Cua Driver',
    description: 'Visual perception extension using OmniParser for element detection. AGPL-3.0 licensed, separate install.',
    crates: ['cua-perception (extension)'],
    language: 'Rust/ONNX',
    license: 'AGPL-3.0',
    tags: ['vision', 'omniparser', 'extension'],
    color: '#a371f7',
    position: { x: 500, y: 400 },
  },

  // Cua Spaces
  {
    id: 'cua-spaces',
    name: 'cua-spaces',
    category: 'Cua Spaces',
    description: 'Desktop app core (Tauri for Linux/Windows, SwiftUI for macOS). Menu bar, notch, live streams, teleport, agent threads.',
    crates: ['cua-spaces', 'cua-spaces-app-core', 'cua-spaces-ffi', 'cua-spaces-cli', 'cua-spaces-ext'],
    language: 'Rust/Swift/TypeScript',
    license: 'FSL-1.1-MIT',
    tags: ['desktop-app', 'tauri', 'swiftui', 'multiplayer'],
    color: '#3fb950',
    position: { x: 900, y: 100 },
  },
  {
    id: 'cua-spacesd',
    name: 'cua-spacesd',
    category: 'Cua Spaces',
    description: 'In-sandbox daemon on port 3211. Provides processes, files, desktop, input, streaming via gRPC. Runs inside every Space.',
    crates: ['cua-spacesd'],
    language: 'Rust',
    license: 'FSL-1.1-MIT',
    tags: ['daemon', 'in-sandbox', 'grpc', 'streaming'],
    color: '#3fb950',
    position: { x: 900, y: 250 },
  },
  {
    id: 'cua-keyvault',
    name: 'cua-keyvault',
    category: 'Cua Spaces',
    description: 'Local vault and broker owning teleport. Encrypted session storage, signed app teleport approval flow.',
    crates: ['cua-keyvault', 'cua-teleport', 'cua-teleport-bundle'],
    language: 'Rust',
    license: 'FSL-1.1-MIT',
    tags: ['security', 'encryption', 'teleport'],
    color: '#3fb950',
    position: { x: 900, y: 400 },
  },
  {
    id: 'cua-volume',
    name: 'cua-volume',
    category: 'Cua Spaces',
    description: 'Versioned drive shared by every Space and agent. Read grants, resumable LAN transfer, S3 fallback.',
    crates: ['cua-volume', 'cua-snapshot-sync'],
    language: 'Rust',
    license: 'FSL-1.1-MIT',
    tags: ['storage', 'versioning', 'sync'],
    color: '#3fb950',
    position: { x: 900, y: 550 },
  },

  // Virtualization
  {
    id: 'lume',
    name: 'Lume',
    category: 'Virtualization',
    description: 'macOS/Linux VM management on Apple Silicon using Virtualization.framework. Unattended setup, SSH, port forwarding.',
    crates: ['lume', 'lumier'],
    language: 'Rust/Swift',
    license: 'MIT',
    tags: ['vm', 'apple-silicon', 'virtualization'],
    color: '#f85149',
    position: { x: 1300, y: 100 },
  },
  {
    id: 'cua-vmm',
    name: 'cua-vmm',
    category: 'Virtualization',
    description: 'VMM abstraction layer: Firecracker, Virtualization.framework, QEMU. MicroVM init, overlay rootfs, vsock agent.',
    crates: ['cua-vmm', 'cua-microvm-init', 'cua-image'],
    language: 'Rust',
    license: 'MIT',
    tags: ['vmm', 'firecracker', 'microvm'],
    color: '#f85149',
    position: { x: 1300, y: 250 },
  },
  {
    id: 'cua-sandbox-core',
    name: 'cua-sandbox-core',
    category: 'Virtualization',
    description: 'Core sandbox orchestration: lifecycle, readiness, resource management. gVisor, runc, QEMU, Lume runtimes.',
    crates: ['cua-sandbox-core', 'cua-disk', 'cua-repo-cache'],
    language: 'Rust',
    license: 'MIT',
    tags: ['sandbox', 'orchestration', 'runtimes'],
    color: '#f85149',
    position: { x: 1300, y: 400 },
  },
  {
    id: 'cua-byoc',
    name: 'cua-byoc',
    category: 'Virtualization',
    description: 'Bring Your Own Cloud: AWS, GCP, Modal providers. Sandbox creation in user cloud accounts, sweeper for cleanup.',
    crates: ['cua-byoc'],
    language: 'Rust',
    license: 'MIT',
    tags: ['cloud', 'aws', 'gcp', 'modal'],
    color: '#f85149',
    position: { x: 1300, y: 550 },
  },

  // Benchmarks & RL
  {
    id: 'cua-bench',
    name: 'cua-bench',
    category: 'Benchmarks',
    description: 'Computer-use benchmark framework. Task creation, evaluation, trajectory export (ATIF), RL environments. cua-bench-rl for training.',
    crates: ['cua-bench (Python)', 'cua-bench-rl', 'cua-bench-s1'],
    language: 'Python/Rust',
    license: 'MIT',
    tags: ['benchmark', 'rl', 'evaluation', 'trajectories'],
    color: '#d29922',
    position: { x: 500, y: 550 },
  },
  {
    id: 'cua-s1',
    name: 'CUA-S1',
    category: 'Benchmarks',
    description: 'System 1 models for computer-use decisions. Forms-focused scoring from structured elements. Python training, ONNX export.',
    crates: ['cua-s1 (Python)'],
    language: 'Python/PyTorch',
    license: 'MIT',
    tags: ['ml', 'system1', 'forms', 'onnx'],
    color: '#d29922',
    position: { x: 700, y: 550 },
  },

  // Media & Cluster
  {
    id: 'cua-media',
    name: 'cua-media',
    category: 'Media Plane',
    description: 'QUIC-based media plane: wire protocol, transport, codecs. Low-latency video/audio streaming for Spaces viewer.',
    crates: ['cua-media-protocol', 'cua-media-transport', 'cua-media-codec', 'cua-media-client', 'cua-viewer', 'cua-spacesd-html5'],
    language: 'Rust',
    license: 'MIT',
    tags: ['media', 'quic', 'streaming', 'viewer'],
    color: '#39c5cf',
    position: { x: 1300, y: 700 },
  },
  {
    id: 'cua-cluster',
    name: 'cua-cluster',
    category: 'Cluster',
    description: 'Multi-machine Spaces cluster: host job runner, placement, node links, join codes. Coordinator for job queue and scheduling.',
    crates: ['cua-cluster', 'cua-spaces-cluster', 'cua-host', 'cua-mesh', 'cua-relay'],
    language: 'Rust',
    license: 'MIT',
    tags: ['cluster', 'mesh', 'wireguard', 'relay'],
    color: '#a5d6ff',
    position: { x: 1100, y: 700 },
  },

  // Security & Observability
  {
    id: 'cua-auth',
    name: 'cua-auth',
    category: 'Security',
    description: 'Sign-in and shared credential store. OIDC, device authorization, token management.',
    crates: ['cua-auth'],
    language: 'Rust',
    license: 'MIT',
    tags: ['auth', 'oidc', 'credentials'],
    color: '#ff7b72',
    position: { x: -100, y: 550 },
  },
  {
    id: 'cua-telemetry',
    name: 'cua-telemetry',
    category: 'Observability',
    description: 'Anonymous product telemetry. Opt-in crash reporting via Sentry with consent and scrubbing.',
    crates: ['cua-telemetry', 'cua-crash', 'cua-create-failure'],
    language: 'Rust',
    license: 'MIT',
    tags: ['telemetry', 'crash-reporting', 'sentry'],
    color: '#ff7b72',
    position: { x: 100, y: 550 },
  },
  {
    id: 'cua-logging',
    name: 'cua-logging',
    category: 'Observability',
    description: 'Structured logging infrastructure shared across crates.',
    crates: ['cua-logging'],
    language: 'Rust',
    license: 'MIT',
    tags: ['logging', 'infrastructure'],
    color: '#ff7b72',
    position: { x: 300, y: 550 },
  },

  // African Extensions (NEW)
  {
    id: 'cua-mcp',
    name: 'cua-mcp',
    category: 'African Extensions',
    description: 'Model Context Protocol server for Cua. stdio + HTTP/SSE transports. 12 tools for sandbox, skill, benchmark orchestration.',
    crates: ['cua-mcp (NEW)'],
    language: 'Rust',
    license: 'MIT',
    tags: ['mcp', 'ai-agents', 'new', 'africa'],
    color: '#ffa657',
    position: { x: -100, y: 700 },
  },
  {
    id: 'african-sandboxes',
    name: 'African Sandboxes',
    category: 'African Extensions',
    description: 'Pre-configured sandbox profiles: M-Pesa Daraja, MTN MoMo, Airtel Money, Orange Money, Wave, NIBSS, PesaLink, BankservAfrica, OpenMRS.',
    crates: ['sandbox-profiles/africa (NEW)'],
    language: 'YAML/Shell',
    license: 'MIT',
    tags: ['africa', 'mobile-money', 'fintech', 'healthcare', 'new'],
    color: '#ffa657',
    position: { x: 100, y: 700 },
  },
  {
    id: 'african-skills',
    name: 'African Skills',
    category: 'African Extensions',
    description: 'Skill packs for 7 African languages: Swahili, Luganda, Amharic, Hausa, Yoruba, Zulu, Xhosa. Mobile money, fintech, healthcare domains.',
    crates: ['skills/african-* (NEW)'],
    language: 'Python/TypeScript',
    license: 'MIT',
    tags: ['skills', 'i18n', 'africa', 'new'],
    color: '#ffa657',
    position: { x: 300, y: 700 },
  },
  {
    id: 'bench-regression',
    name: 'Benchmark Regression',
    category: 'African Extensions',
    description: 'Criterion-style regression detection for cua-bench. SQLite/PostgreSQL storage, GitHub Actions integration, 5% threshold.',
    crates: ['cua-bench-regression (NEW)'],
    language: 'Rust',
    license: 'MIT',
    tags: ['benchmark', 'regression', 'criterion', 'new'],
    color: '#ffa657',
    position: { x: 500, y: 700 },
  },
  {
    id: 'arch-viz',
    name: 'Architecture Viz',
    category: 'African Extensions',
    description: 'Interactive React Flow architecture explorer. 90+ nodes, 5 views, expandable groups, GitHub Pages deployment.',
    crates: ['docs/architecture (NEW)'],
    language: 'TypeScript/React',
    license: 'MIT',
    tags: ['visualization', 'react-flow', 'documentation', 'new'],
    color: '#ffa657',
    position: { x: 700, y: 700 },
  },
];

export const architectureEdges: EdgeData[] = [
  // SDK Core dependencies
  { id: 'e1', source: 'cua-sdk', target: 'cua-cli', type: 'dependency' },
  { id: 'e2', source: 'cua-sdk', target: 'cua-daemon', type: 'dependency' },
  { id: 'e3', source: 'cua-sdk', target: 'cua-python', type: 'dependency' },
  { id: 'e4', source: 'cua-sdk', target: 'cua-typescript', type: 'dependency' },
  { id: 'e5', source: 'cua-sdk', target: 'cua-swift', type: 'dependency' },
  
  // Driver uses SDK
  { id: 'e6', source: 'cua-sdk', target: 'cua-driver', type: 'dependency' },
  { id: 'e7', source: 'cua-sdk', target: 'cua-driver-bench', type: 'dependency' },
  { id: 'e8', source: 'cua-sdk', target: 'cua-perception', type: 'extension' },
  
  // Spaces uses SDK + Driver
  { id: 'e9', source: 'cua-sdk', target: 'cua-spaces', type: 'dependency' },
  { id: 'e10', source: 'cua-driver', target: 'cua-spaces', type: 'dependency' },
  { id: 'e11', source: 'cua-sdk', target: 'cua-spacesd', type: 'dependency' },
  { id: 'e12', source: 'cua-driver', target: 'cua-spacesd', type: 'control-flow', label: 'input/screen' },
  
  // Spaces internal
  { id: 'e13', source: 'cua-spaces', target: 'cua-keyvault', type: 'dependency' },
  { id: 'e14', source: 'cua-spaces', target: 'cua-volume', type: 'dependency' },
  { id: 'e15', source: 'cua-keyvault', target: 'cua-teleport', type: 'dependency' },
  { id: 'e16', source: 'cua-teleport', target: 'cua-teleport-bundle', type: 'dependency' },
  
  // Virtualization
  { id: 'e17', source: 'cua-sdk', target: 'lume', type: 'dependency' },
  { id: 'e18', source: 'cua-sdk', target: 'cua-vmm', type: 'dependency' },
  { id: 'e19', source: 'cua-vmm', target: 'cua-microvm-init', type: 'dependency' },
  { id: 'e19b', source: 'cua-vmm', target: 'cua-image', type: 'dependency' },
  { id: 'e20', source: 'cua-sdk', target: 'cua-sandbox-core', type: 'dependency' },
  { id: 'e21', source: 'cua-sandbox-core', target: 'cua-disk', type: 'dependency' },
  { id: 'e22', source: 'cua-sandbox-core', target: 'cua-repo-cache', type: 'dependency' },
  { id: 'e23', source: 'cua-sdk', target: 'cua-byoc', type: 'dependency' },
  
  // Benchmarks
  { id: 'e24', source: 'cua-sdk', target: 'cua-bench', type: 'dependency' },
  { id: 'e25', source: 'cua-spacesd', target: 'cua-bench', type: 'data-flow', label: 'sandbox API' },
  { id: 'e26', source: 'cua-s1', target: 'cua-bench', type: 'dependency', label: 'agent' },
  { id: 'e27', source: 'cua-bench', target: 'cua-bench-rl', type: 'control-flow', label: 'RL training' },
  { id: 'e28', source: 'cua-bench', target: 'cua-bench-s1', type: 'dependency' },
  
  // Media & Cluster
  { id: 'e29', source: 'cua-spacesd', target: 'cua-media', type: 'dependency', label: 'streaming' },
  { id: 'e30', source: 'cua-media', target: 'cua-viewer', type: 'dependency' },
  { id: 'e31', source: 'cua-media', target: 'cua-spacesd-html5', type: 'dependency' },
  { id: 'e32', source: 'cua-sdk', target: 'cua-cluster', type: 'dependency' },
  { id: 'e33', source: 'cua-cluster', target: 'cua-spaces-cluster', type: 'dependency' },
  { id: 'e34', source: 'cua-cluster', target: 'cua-host', type: 'dependency' },
  { id: 'e35', source: 'cua-cluster', target: 'cua-mesh', type: 'dependency' },
  { id: 'e36', source: 'cua-mesh', target: 'cua-relay', type: 'dependency' },
  
  // Security & Observability
  { id: 'e37', source: 'cua-sdk', target: 'cua-auth', type: 'dependency' },
  { id: 'e38', source: 'cua-sdk', target: 'cua-telemetry', type: 'dependency' },
  { id: 'e39', source: 'cua-telemetry', target: 'cua-crash', type: 'dependency' },
  { id: 'e40', source: 'cua-telemetry', target: 'cua-create-failure', type: 'dependency' },
  { id: 'e41', source: 'cua-sdk', target: 'cua-logging', type: 'dependency' },
  
  // African Extensions (NEW)
  { id: 'e42', source: 'cua-sdk', target: 'cua-mcp', type: 'dependency', label: 'NEW' },
  { id: 'e43', source: 'cua-mcp', target: 'african-sandboxes', type: 'control-flow', label: 'orchestrates' },
  { id: 'e44', source: 'cua-mcp', target: 'african-skills', type: 'control-flow', label: 'invokes' },
  { id: 'e45', source: 'cua-mcp', target: 'bench-regression', type: 'data-flow', label: 'tracks' },
  { id: 'e46', source: 'cua-bench', target: 'bench-regression', type: 'data-flow', label: 'feeds' },
  { id: 'e47', source: 'cua-mcp', target: 'arch-viz', type: 'extension', label: 'documents' },
];

export const categoryGroups = [
  { id: 'sdk-core', label: 'SDK Core', color: '#58a6ff', nodes: ['cua-sdk', 'cua-cli', 'cua-daemon'] },
  { id: 'sdk-bindings', label: 'SDK Bindings', color: '#58a6ff', nodes: ['cua-python', 'cua-typescript', 'cua-swift'] },
  { id: 'cua-driver', label: 'Cua Driver', color: '#a371f7', nodes: ['cua-driver', 'cua-driver-bench', 'cua-perception'] },
  { id: 'cua-spaces', label: 'Cua Spaces', color: '#3fb950', nodes: ['cua-spaces', 'cua-spacesd', 'cua-keyvault', 'cua-volume'] },
  { id: 'virtualization', label: 'Virtualization', color: '#f85149', nodes: ['lume', 'cua-vmm', 'cua-sandbox-core', 'cua-byoc'] },
  { id: 'benchmarks', label: 'Benchmarks & ML', color: '#d29922', nodes: ['cua-bench', 'cua-s1'] },
  { id: 'media-cluster', label: 'Media & Cluster', color: '#39c5cf', nodes: ['cua-media', 'cua-cluster'] },
  { id: 'security-obs', label: 'Security & Observability', color: '#ff7b72', nodes: ['cua-auth', 'cua-telemetry', 'cua-logging'] },
  { id: 'africa-extensions', label: 'African Extensions (NEW)', color: '#ffa657', nodes: ['cua-mcp', 'african-sandboxes', 'african-skills', 'bench-regression', 'arch-viz'] },
];

export const views = [
  { id: 'overview', label: 'Overview', description: 'Full architecture with all components', groups: categoryGroups.map(g => g.id) },
  { id: 'sdk', label: 'SDK', description: 'SDK core, bindings, and CLI', groups: ['sdk-core', 'sdk-bindings'] },
  { id: 'driver', label: 'Driver', description: 'Cua Driver and perception', groups: ['cua-driver'] },
  { id: 'spaces', label: 'Spaces', description: 'Cua Spaces, spacesd, keyvault, volume', groups: ['cua-spaces'] },
  { id: 'virt', label: 'Virtualization', description: 'Lume, VMM, sandbox core, BYOC', groups: ['virtualization'] },
  { id: 'bench', label: 'Benchmarks', description: 'cua-bench, CUA-S1, RL', groups: ['benchmarks'] },
  { id: 'infra', label: 'Infrastructure', description: 'Media, cluster, security, observability', groups: ['media-cluster', 'security-obs'] },
  { id: 'africa', label: 'African Extensions', description: 'MCP, sandboxes, skills, regression, viz', groups: ['africa-extensions'] },
];