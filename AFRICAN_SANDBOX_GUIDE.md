# AFRICAN_SANDBOX_GUIDE.md — African Sandbox Profiles

## Overview

This guide documents the pre-configured sandbox profiles for African mobile money, fintech, and healthcare systems. These profiles enable developers to test integrations with realistic simulators in isolated, reproducible environments.

## Profile Catalog

### Mobile Money Profiles

#### M-Pesa Daraja (Kenya) — `mpesa-daraja`
- **Provider**: Safaricom
- **Region**: Kenya (KE)
- **Language**: Swahili (sw)
- **Currency**: KES
- **Simulator Ports**: 8080 (HTTP), 8443 (HTTPS)
- **Key Endpoints**:
  - OAuth: `/oauth/v1/generate`
  - STK Push: `/mpesa/stkpush/v1/processrequest`
  - B2C: `/mpesa/b2c/v1/paymentrequest`
  - C2B: `/mpesa/c2b/v1/registerurl`
  - Query: `/mpesa/stkpushquery/v1/query`
- **Required Env**: `MPESA_CONSUMER_KEY`, `MPESA_CONSUMER_SECRET`
- **Skills**: `african-mobile-money/mpesa-sw`, `african-mobile-money/mpesa-en`

#### MTN MoMo (Ghana/Uganda) — `mtn-momo`
- **Provider**: MTN Group
- **Region**: Ghana (GH) / Uganda (UG)
- **Language**: English (en), Twi (tw)
- **Currency**: GHS / UGX
- **Simulator Ports**: 8081 (HTTP), 8444 (HTTPS)
- **Key Endpoints**:
  - Collection: `/collection/v1_0/requesttopay`
  - Disbursement: `/disbursement/v1_0/transfer`
  - KYC: `/v1_0/accountholder/msisdn/{msisdn}/basicuserinfo`
- **Required Env**: `MTN_MOMO_SUBSCRIPTION_KEY`, `MTN_MOMO_API_USER`, `MTN_MOMO_API_KEY`
- **Skills**: `african-mobile-money/mtn-momo-en`, `african-mobile-money/mtn-momo-tw`

#### Airtel Money (Nigeria/Uganda) — `airtel-money`
- **Provider**: Airtel Africa
- **Region**: Nigeria (NG) / Uganda (UG)
- **Language**: English (en), Hausa (ha), Yoruba (yo)
- **Currency**: NGN / UGX
- **Simulator Ports**: 8082 (HTTP), 8445 (HTTPS)
- **Key Endpoints**: Generic payment API with USSD simulation
- **Required Env**: `AIRTEL_CLIENT_ID`, `AIRTEL_CLIENT_SECRET`
- **Skills**: `african-mobile-money/airtel-en`, `african-mobile-money/airtel-ha`, `african-mobile-money/airtel-yo`

#### Orange Money (Côte d'Ivoire/Senegal) — `orange-money`
- **Provider**: Orange Group
- **Region**: Côte d'Ivoire (CI) / Senegal (SN)
- **Language**: French (fr), Wolof (wo)
- **Currency**: XOF
- **Simulator Ports**: 8083 (HTTP), 8446 (HTTPS)
- **Key Endpoints**: Orange Money Africa API
- **Required Env**: `ORANGE_CLIENT_ID`, `ORANGE_CLIENT_SECRET`
- **Skills**: `african-mobile-money/orange-fr`, `african-mobile-money/orange-wo`

#### Wave Money (Senegal/CI) — `wave-money`
- **Provider**: Wave
- **Region**: Senegal (SN) / Côte d'Ivoire (CI)
- **Language**: French (fr), Wolof (wo)
- **Currency**: XOF
- **Simulator Ports**: 8084 (HTTP), 8447 (HTTPS)
- **Key Endpoints**: Simple payment API, no-fee model
- **Required Env**: `WAVE_API_KEY`
- **Skills**: `african-mobile-money/wave-fr`, `african-mobile-money/wave-wo`

### FinTech Profiles

#### NIBSS (Nigeria) — `nibss`
- **Provider**: Nigeria Inter-Bank Settlement System
- **Region**: Nigeria (NG)
- **Language**: English (en)
- **Currency**: NGN
- **Simulator Ports**: 8090 (HTTP), 8450 (HTTPS)
- **Key Endpoints**:
  - NIP Transfer: `/nip/transfer`
  - BVN Verification: `/bvn/verify`
  - Account Validation: `/account/validate`
- **Required Env**: `NIBSS_CLIENT_ID`, `NIBSS_CLIENT_SECRET`
- **Skills**: `african-fintech/nibss-en`, `african-fintech/bvn-verification-en`

#### PesaLink (Kenya) — `pesalink`
- **Provider**: Integrated Payment Services Limited (IPSL)
- **Region**: Kenya (KE)
- **Language**: Swahili (sw), English (en)
- **Currency**: KES
- **Simulator Ports**: 8091 (HTTP), 8451 (HTTPS)
- **Key Endpoints**: Real-time interbank transfer, account verification
- **Required Env**: `PESALINK_CLIENT_ID`, `PESALINK_CLIENT_SECRET`
- **Skills**: `african-fintech/pesalink-sw`, `african-fintech/pesalink-en`

#### BankservAfrica (South Africa) — `bankservafrica`
- **Provider**: BankservAfrica
- **Region**: South Africa (ZA)
- **Language**: English (en), Afrikaans (af)
- **Currency**: ZAR
- **Simulator Ports**: 8092 (HTTP), 8452 (HTTPS)
- **Key Endpoints**: RTC (Real-Time Clearing), EFT, Debit Orders
- **Required Env**: `BANKSERV_CLIENT_ID`, `BANKSERV_CLIENT_SECRET`
- **Skills**: `african-fintech/bankservafrica-en`, `african-fintech/bankservafrica-af`

### Healthcare Profiles

#### OpenMRS (Multi-country) — `openmrs`
- **Provider**: OpenMRS Community
- **Region**: Kenya (KE) / Uganda (UG) / Tanzania (TZ) / Rwanda (RW)
- **Language**: English (en), Swahili (sw)
- **Currency**: KES / UGX / TZS / RWF
- **Simulator Ports**: 8080 (HTTP), 8443 (HTTPS)
- **Key Endpoints**:
  - REST API: `/openmrs/ws/rest/v1/`
  - FHIR R4: `/openmrs/ws/fhir2/R4/`
- **Default Credentials**: admin / Admin123
- **Skills**: `african-healthcare/openmrs-en`, `african-healthcare/openmrs-sw`

## Using Profiles

### Via MCP Server (Recommended)

```bash
# Start MCP server
cua-mcp serve --stdio

# In your MCP client (Claude Code, Codex, etc.)
# Tool: cua.sandbox.create
{
  "profile": "mpesa-daraja",
  "name": "mpesa-test-001",
  "env": {
    "MPESA_CONSUMER_KEY": "your-key",
    "MPESA_CONSUMER_SECRET": "your-secret"
  }
}
```

### Via CLI (Direct)

```bash
# Create sandbox from profile
cua sb create --profile mpesa-daraja --name mpesa-test

# Execute commands
cua sb exec mpesa-test "curl http://localhost:8080/health"

# Screenshot
cua sb screenshot mpesa-test

# Destroy when done
cua sb rm mpesa-test
```

### Via Python SDK

```python
from cua_sandbox import Image, Sandbox

async with Sandbox.ephemeral(
    Image.linux(),
    local=True,
    profile="mpesa-daraja",  # NEW: profile parameter
    env={
        "MPESA_CONSUMER_KEY": "your-key",
        "MPESA_CONSUMER_SECRET": "your-secret"
    }
) as sb:
    # Run test
    result = await sb.shell.run("curl http://localhost:8080/health")
    print(result.stdout)
```

## Profile Structure

Each profile is a YAML file with this structure:

```yaml
name: "profile-id"
display_name: "Human Readable Name"
description: "Description of the profile"
region: "KE"
category: "mobile-money"
language: "sw"
currency: "KES"
provider: "Provider Name"
documentation_url: "https://..."
sandbox_url: "https://..."

image:
  base: "ghcr.io/trycua/linux:latest"
  setup_script: "scripts/setup-profile.sh"
  ports:
    - 8080   # Simulator HTTP
    - 8443   # Simulator HTTPS
    - 3211   # cua-spacesd
  env:
    ENV_VAR: "${ENV_VAR}"

resources:
  cpu: "2"
  memory: "4G"
  disk: "20G"
  network: "isolated"
  allowlist:
    - "api.provider.com"
    - "github.com"

skills:
  - "skill-name-lang"

tags:
  - "kenya"
  - "mobile-money"

verification:
  health_check: "curl -f http://localhost:8080/health"
  test_command: "python3 -m pytest tests/test_integration.py -v"
```

## Creating Custom Profiles

### 1. Create Profile YAML
```bash
cp libs/cua/sandbox-profiles/africa/mobile-money/mpesa-daraja.yaml \
   libs/cua/sandbox-profiles/africa/mobile-money/my-custom.yaml
```

### 2. Create Setup Script
```bash
cp libs/cua/sandbox-profiles/africa/scripts/setup-mpesa-daraja.sh \
   libs/cua/sandbox-profiles/africa/scripts/setup-my-custom.sh
```

Edit the script to install your simulator.

### 3. Register Profile
The MCP server auto-discovers profiles in `libs/cua/sandbox-profiles/africa/`.

### 4. Create Skills (Optional)
Create skill directories under `libs/skills/african-mobile-money/your-skill/`.

## CI/CD Integration

### GitHub Actions

```yaml
# .github/workflows/test-african-integrations.yml
name: Test African Integrations

on: [push, pull_request]

jobs:
  test-mpesa:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Start MCP Server
        run: |
          cargo build --release -p cua-mcp
          ./target/release/cua-mcp serve --http --sse &
          sleep 5
      - name: Create M-Pesa Sandbox
        run: |
          curl -X POST http://localhost:3212/mcp \
            -H "Content-Type: application/json" \
            -d '{
              "jsonrpc": "2.0",
              "id": 1,
              "method": "tools/call",
              "params": {
                "name": "cua.sandbox.create",
                "arguments": {
                  "profile": "mpesa-daraja",
                  "name": "ci-mpesa-test"
                }
              }
            }'
      - name: Run M-Pesa Tests
        run: |
          # Your test commands here
          curl -X POST http://localhost:3212/mcp \
            -H "Content-Type: application/json" \
            -d '{
              "jsonrpc": "2.0",
              "id": 2,
              "method": "tools/call",
              "params": {
                "name": "cua.skill.invoke",
                "arguments": {
                  "sandbox_id": "ci-mpesa-test",
                  "skill": "african-mobile-money/mpesa-sw",
                  "command": "stk_push",
                  "args": {"phone": "254712345678", "amount": 1000}
                }
              }
            }'
      - name: Cleanup
        run: |
          curl -X POST http://localhost:3212/mcp \
            -H "Content-Type: application/json" \
            -d '{
              "jsonrpc": "2.0",
              "id": 3,
              "method": "tools/call",
              "params": {
                "name": "cua.sandbox.destroy",
                "arguments": {"sandbox_id": "ci-mpesa-test"}
              }
            }'
```

### Local Development

```bash
# Start local MCP server
cargo build --release -p cua-mcp
./target/release/cua-mcp serve --stdio &

# In another terminal, use with your agent
# The server communicates via stdin/stdout
```

## Best Practices

1. **Use environment variables** for credentials, never hardcode
2. **Destroy sandboxes** after tests to free resources
3. **Use network allowlists** to restrict sandbox egress
3. **Set resource limits** appropriate for your test
4. **Monitor sandbox logs** for debugging
5. **Use skills** for repeatable test patterns
6. **Version control profiles** with your test code

## Troubleshooting

### Sandbox Fails to Start
- Check Docker/gVisor is running: `docker ps` or `systemctl status gvisor`
- Verify base image: `docker pull ghcr.io/trycua/linux:latest`
- Check resource availability: `free -h`, `df -h`
- Review setup script logs: `docker logs <sandbox-id>`

### Simulator Not Responding
- Verify setup script completed: `docker exec <sandbox-id> curl localhost:8080/health`
- Check port mappings in profile YAML
- Verify environment variables are set

### Network Issues
- Confirm allowlist includes required domains
- Check firewall rules on host
- Verify DNS resolution in sandbox: `docker exec <sandbox-id> nslookup api.provider.com`

## Support

- **Documentation**: https://cua.ai/docs
- **Issues**: https://github.com/nakisisageorge/cua-enhanced/issues
- **Email**: nakisisageorge@gmail.com