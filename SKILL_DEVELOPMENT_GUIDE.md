# SKILL_DEVELOPMENT_GUIDE.md — Creating African Language Skills

## Overview

This guide explains how to create, test, and publish African language skills for the cua-enhanced skill system. Skills are reusable automation modules that agents can invoke to perform domain-specific tasks in African market contexts.

## Skill Architecture

### Skill Structure

Each skill is a directory under `libs/skills/` with this structure:

```
libs/skills/african-mobile-money/mpesa-sw/
├── SKILL.md              # Skill manifest (required)
├── main.py               # Skill entry point (Python)
├── main.ts               # Skill entry point (TypeScript, optional)
├── utils.py              # Helper functions
├── locators.py           # UI element locators for the language
├── flows/                # Pre-built flow definitions
│   ├── stk_push.yaml
│   ├── balance_check.yaml
│   └── transaction_history.yaml
├── tests/                # Skill tests
│   ├── test_stk_push.py
│   └── test_balance.py
└── README.md             # Skill documentation
```

### Skill Manifest (SKILL.md)

```yaml
---
name: african-mobile-money/mpesa-sw
description: M-Pesa Daraja automation in Swahili for Kenyan market
version: 1.0.0
author: George Nakisisa
license: MIT
category: mobile-money
language: sw
region: KE
currency: KES
provider: Safaricom
requires:
  - gui-automation
  - cua-sdk >= 0.24.0
commands:
  - stk_push
  - balance_check
  - transaction_history
  - b2c_payment
  - c2b_register
---

# M-Pesa Swahili Skill

Automate M-Pesa Daraja operations with Swahili UI support.

## Commands

### stk_push
Initiate STK Push payment.

**Parameters:**
- `phone` (string, required): Phone number in format 2547XXXXXXXX
- `amount` (integer, required): Amount in KES cents
- `reference` (string, optional): Transaction reference
- `description` (string, optional): Payment description

**Example:**
```bash
cua skill invoke african-mobile-money/mpesa-sw stk_push \
  --phone 254712345678 --amount 100000 --reference "ORDER-123"
```

### balance_check
Check M-Pesa account balance.

**Parameters:**
- `account` (string, optional): Account identifier

### transaction_history
Retrieve transaction history.

**Parameters:**
- `start_date` (string, optional): ISO date
- `end_date` (string, optional): ISO date
- `limit` (integer, optional): Max transactions (default: 50)
```

## Language-Specific Implementation

### Swahili (Kiswahili) — `sw`

#### UI Patterns
- **Button labels**: "Bofya hapa" (Click here), "Maliza" (Finish), "Ghana" (Confirm)
- **Form fields**: "Nambari ya simu" (Phone number), "Kiasi" (Amount), "Sababu" (Reason)
- **Messages**: "Hongera!" (Congratulations!), "Jaribu tena" (Try again)

#### Locators Example
```python
# locators.py
SWAHILI_LOCATORS = {
    "stk_push": {
        "phone_field": "//input[@placeholder='Nambari ya simu']",
        "amount_field": "//input[@placeholder='Kiasi']",
        "submit_button": "//button[contains(text(), 'Ghana')]",
        "confirm_button": "//button[contains(text(), 'Maliza')]",
        "success_message": "//div[contains(text(), 'Hongera')]",
    },
    "ussd_menu": {
        "main_menu": "//div[contains(@class, 'ussd-menu')]",
        "option_1": "//span[text()='1. Lipa']",
        "option_2": "//span[text()='2. Salio']",
        "input_prompt": "//input[@type='tel']",
    }
}
```

### Luganda — `lg`

#### UI Patterns
- **Button labels**: "Tebaawo" (Confirm), "Leka" (Cancel), "Kiyita" (Call)
- **Form fields**: "Nsimbe" (Phone), "Kimu" (Amount), "Ebiro" (Reference)
- **Messages**: "Mukisa!" (Success!), "Obuyinza!" (Error!)

#### Cultural Notes
- Uganda uses MTN MoMo and Airtel Money primarily
- USSD codes: *165# (MTN), *185# (Airtel)
- Currency: UGX (no cents, whole shillings)

### Amharic — `am`

#### UI Patterns
- **Button labels**: "አረጋግጥ" (Confirm), "ይቅር" (Cancel), "ያስገባ" (Submit)
- **Form fields**: "ስልክ" (Phone), "ገንዘብ" (Amount), "ስም" (Name)
- **Direction**: RTL support needed for some contexts
- **Calendar**: Ethiopian calendar (different year/month)

### Hausa — `ha`

#### UI Patterns
- **Button labels**: "Tabbatar" (Confirm), "Soke" (Cancel), "Bi Bayani" (Submit)
- **Form fields**: "Lamba" (Number), "Kudin" (Amount), "Dalili" (Reference)
- **Messages**: "Barka da aiki!" (Well done!), "Kuskure!" (Error!)

#### Cultural Notes
- Nigeria: Airtel, MTN, 9mobile, Glo
- USSD heavy, low data usage preferred
- Prayer time awareness for scheduling

### Yoruba — `yo`

#### UI Patterns
- **Button labels**: "Bẹ́ẹ̀ni" (Yes/Confirm), "Bẹ́ẹ̀kọ́" (Cancel), "Fifí sílẹ̀" (Submit)
- **Form fields**: "Nọ́mbà" (Number), "Ọ́wọ́" (Amount), "Àkọ́nì" (Reference)
- **Messages**: "Ẹ ṣé!" (Thanks!), "Àlàáfíà" (Error/Warning)

### Zulu — `zu`

#### UI Patterns
- **Button labels**: "Qinisekisa" (Confirm), "Khansela" (Cancel), "Ngenisa" (Submit)
- **Form fields**: "Inombolo" (Number), "Imali" (Amount), "Isibonelo" (Reference)
- **Messages**: "Impumelelo!" (Success!), "Iphutha!" (Error!)

### Xhosa — `xh`

#### UI Patterns
- **Button labels**: "Qinisekisa" (Confirm), "Cancele" (Cancel), "Faka" (Submit)
- **Form fields**: "Inombolo" (Number), "Imali" (Amount), "Umzekelo" (Reference)
- **Messages**: "Impumelelo!" (Success!), "Uluphupha!" (Error!)

## Creating a New Skill

### Step 1: Create Directory Structure
```bash
mkdir -p libs/skills/african-mobile-money/my-skill-sw
cd libs/skills/african-mobile-money/my-skill-sw
```

### Step 2: Create SKILL.md
```bash
cat > SKILL.md << 'EOF'
---
name: african-mobile-money/my-skill-sw
description: My custom skill for Swahili market
version: 1.0.0
author: Your Name
license: MIT
category: mobile-money
language: sw
region: KE
currency: KES
provider: Your Provider
requires:
  - gui-automation
commands:
  - my_command
---

# My Skill

Description of what this skill does.

## Commands

### my_command
Description of the command.

**Parameters:**
- `param1` (type, required): Description
- `param2` (type, optional): Description
EOF
```

### Step 3: Create Python Entry Point
```bash
cat > main.py << 'EOF'
#!/usr/bin/env python3
"""
My Skill - Swahili Market Automation
"""

import asyncio
import json
import sys
from typing import Dict, Any
from cua_sandbox import Sandbox, Image

class MySkill:
    def __init__(self, sandbox: Sandbox):
        self.sandbox = sandbox
    
    async def my_command(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """Execute my custom command."""
        # Use gui-automation skill for UI interaction
        # self.sandbox.skill.invoke("gui-automation", "click", ...)
        
        return {
            "success": True,
            "message": "Command executed successfully",
            "data": {}
        }

async def main():
    if len(sys.argv) < 2:
        print(json.dumps({"error": "Usage: skill.py <command> [params_json]"}))
        sys.exit(1)
    
    command = sys.argv[1]
    params = json.loads(sys.argv[2]) if len(sys.argv) > 2 else {}
    
    # Connect to sandbox (assumes cua.sandbox.create was called first)
    # In real implementation, sandbox connection is passed via environment
    # or the skill is invoked through the MCP server which provides the sandbox
    
    skill = MySkill(None)  # Would be initialized with actual sandbox
    
    if command == "my_command":
        result = await skill.my_command(params)
    else:
        result = {"error": f"Unknown command: {command}"}
    
    print(json.dumps(result))

if __name__ == "__main__":
    asyncio.run(main())
EOF
chmod +x main.py
```

### Step 4: Create Locators
```bash
cat > locators.py << 'EOF'
"""
UI Element Locators for Swahili Market
"""

SWAHILI_LOCATORS = {
    "common": {
        "submit_button": "//button[contains(text(), 'Ghana') or contains(text(), 'Maliza')]",
        "cancel_button": "//button[contains(text(), 'Leka') or contains(text(), 'Ghairi')]",
        "confirm_dialog": "//div[@role='dialog']//button[contains(text(), 'Ghana')]",
        "success_toast": "//div[contains(@class, 'toast') and contains(text(), 'Hongera')]",
        "error_toast": "//div[contains(@class, 'toast') and contains(text(), 'Jaribu tena')]",
    },
    "payment": {
        "phone_field": "//input[@type='tel' or @placeholder='Nambari ya simu']",
        "amount_field": "//input[@type='number' or @placeholder='Kiasi']",
        "reference_field": "//input[@placeholder='Rejea' or @placeholder='Sababu']",
        "pay_button": "//button[contains(text(), 'Lipa') or contains(text(), 'Maliza')]",
    },
    "ussd": {
        "menu_container": "//div[contains(@class, 'ussd')]",
        "option_template": "//span[text()='{option}. {label}']",
        "input_field": "//input[@type='text' or @type='tel']",
    }
}

def get_locator(category: str, key: str) -> str:
    """Get a locator by category and key."""
    return SWAHILI_LOCATORS.get(category, {}).get(key, "")
EOF
```

### Step 5: Create Tests
```bash
mkdir -p tests
cat > tests/test_my_command.py << 'EOF'
"""
Tests for my-skill-sw
"""

import pytest
import asyncio
from unittest.mock import AsyncMock, MagicMock, patch
import sys
import os

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from main import MySkill

@pytest.fixture
def mock_sandbox():
    sandbox = AsyncMock()
    sandbox.skill = AsyncMock()
    sandbox.shell = AsyncMock()
    return sandbox

@pytest.mark.asyncio
async def test_my_command_success(mock_sandbox):
    skill = MySkill(mock_sandbox)
    result = await skill.my_command({"param1": "test"})
    
    assert result["success"] is True
    assert "message" in result

@pytest.mark.asyncio
async def test_my_command_with_params(mock_sandbox):
    skill = MySkill(mock_sandbox)
    result = await skill.my_command({"param1": "value1", "param2": "value2"})
    
    assert result["success"] is True
EOF
```

### Step 6: Create README
```bash
cat > README.md << 'EOF'
# my-skill-sw

Custom skill for Swahili-speaking markets.

## Installation

This skill is automatically available when using the `african-mobile-money` sandbox profiles.

## Commands

### my_command
Description of the command.

**Parameters:**
- `param1` (string, required): Description
- `param2` (string, optional): Description

**Returns:**
```json
{
  "success": true,
  "message": "Command executed successfully",
  "data": {}
}
```

## Usage

### Via MCP Server
```bash
cua skill invoke african-mobile-money/my-skill-sw my_command \
  --param1 "value1" --param2 "value2"
```

### Via CLI
```bash
cua skill invoke african-mobile-money/my-skill-sw my_command \
  '{"param1": "value1", "param2": "value2"}'
```

## Testing

```bash
cd libs/skills/african-mobile-money/my-skill-sw
python -m pytest tests/ -v
```

## Development

1. Edit `main.py` for command implementations
2. Edit `locators.py` for UI element locators
3. Add tests in `tests/`
4. Update `SKILL.md` for new commands
EOF
```

## Testing Skills

### Unit Tests
```bash
cd libs/skills/african-mobile-money/my-skill-sw
python -m pytest tests/ -v
```

### Integration Tests (with real sandbox)
```bash
# Start MCP server
cua-mcp serve --stdio &

# Create sandbox
cua sb create --profile mpesa-daraja --name skill-test

# Run skill test
cua skill invoke african-mobile-money/my-skill-sw my_command \
  --param1 "test"

# Cleanup
cua sb rm skill-test
```

### CI/CD Testing
```yaml
# .github/workflows/test-skills.yml
name: Test African Skills

on: [push, pull_request]

jobs:
  test-skills:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Python
        uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Install dependencies
        run: |
          pip install pytest pytest-asyncio
      - name: Test all African skills
        run: |
          for skill in libs/skills/african-*/; do
            if [ -f "$skill/tests/test_*.py" ]; then
              echo "Testing $skill"
              cd "$skill" && python -m pytest tests/ -v && cd -
            fi
          done
```

## Publishing Skills

### 1. Add to Repository
```bash
git add libs/skills/african-mobile-money/my-skill-sw/
git commit -m "feat: add my-skill-sw for Swahili market"
git push
```

### 2. Skill Registry (Future)
Skills will be discoverable via the MCP server's `cua.skill.list` tool.

### 3. Versioning
Follow semantic versioning in `SKILL.md`:
```yaml
version: 1.0.0  # MAJOR.MINOR.PATCH
```

## Best Practices

### 1. Language Authenticity
- **Use native speakers** for UI text review
- **Test with real devices** in target markets
- **Account for regional variations** (e.g., Kenyan vs Tanzanian Swahili)

### 2. UI Resilience
- **Multiple locator strategies**: XPath, CSS, text content, accessibility IDs
- **Fallback locators**: Primary + backup for each element
- **Wait strategies**: Explicit waits with timeouts, not sleeps

### 3. Error Handling
```python
async def robust_click(self, locator: str, retries: int = 3):
    for attempt in range(retries):
        try:
            await self.sandbox.skill.invoke("gui-automation", "click", {"xpath": locator})
            return True
        except Exception as e:
            if attempt == retries - 1:
                raise
            await asyncio.sleep(1 * (attempt + 1))
    return False
```

### 4. Cultural Sensitivity
- **Prayer times**: Avoid scheduling during Islamic prayer times in Muslim-majority regions
- **Holidays**: Respect local holidays (Independence days, religious holidays)
- **Currency formatting**: Use local conventions (UGX: no decimals, KES: 2 decimals)

### 5. Performance
- **Batch operations**: Group multiple UI actions
- **Reuse screenshots**: Don't re-capture if UI hasn't changed
- **Parallel execution**: Run independent flows concurrently

## Debugging Skills

### 1. Enable Debug Logging
```bash
export CUA_LOG_LEVEL=debug
cua skill invoke african-mobile-money/my-skill-sw my_command
```

### 2. Trajectory Replay
```bash
# View recorded trajectory
cua trajectory view

# List sessions
cua trajectory ls
```

### 3. Screenshot on Failure
```python
async def safe_operation(self, operation, *args):
    try:
        return await operation(*args)
    except Exception as e:
        # Capture screenshot for debugging
        await self.sandbox.skill.invoke("gui-automation", "screenshot", {})
        raise
```

## Example: Complete M-Pesa STK Push Skill

See `libs/skills/african-mobile-money/mpesa-sw/` for a production-ready example with:
- STK Push with phone/amount validation
- Balance check with formatted output
- Transaction history with pagination
- B2C payment for disbursements
- C2B URL registration
- USSD flow automation
- Comprehensive tests

## Resources

- **Cua SDK Docs**: https://cua.ai/docs/cua-sdk
- **GUI Automation Skill**: `libs/skills/gui-automation/`
- **Jev-Use Skill**: `libs/skills/jev-use/`
- **MCP Guide**: [MCP_GUIDE.md](MCP_GUIDE.md)
- **African Sandbox Guide**: [AFRICAN_SANDBOX_GUIDE.md](AFRICAN_SANDBOX_GUIDE.md)

## Support

- **Issues**: https://github.com/nakisisageorge/cua-enhanced/issues
- **Discord**: https://discord.gg/mVnXXpdE85
- **Email**: nakisisageorge@gmail.com