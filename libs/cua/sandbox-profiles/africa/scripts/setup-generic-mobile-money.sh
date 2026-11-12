#!/usr/bin/env bash
# Generic African Mobile Money Simulator Setup
# Usage: setup-generic-mobile-money.sh <name> <port> <provider>

set -euo pipefail

NAME=${1:-"generic-momo"}
PORT=${2:-8080}
PROVIDER=${3:-"Generic"}

echo "Setting up $PROVIDER Mobile Money Sandbox ($NAME)..."

curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
apt-get update && apt-get install -y nodejs python3 python3-pip

mkdir -p /opt/$NAME-simulator
cd /opt/$NAME-simulator

cat > package.json << EOF
{
  "name": "$NAME-simulator",
  "version": "1.0.0",
  "main": "server.js",
  "scripts": { "start": "node server.js" },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5",
    "body-parser": "^1.20.2",
    "uuid": "^9.0.0"
  }
}
EOF

cat > server.js << 'EOF'
const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const { v4: uuidv4 } = require('uuid');

const app = express();
const PORT = process.env.PORT || 8080;
const transactions = new Map();

app.use(cors());
app.use(bodyParser.json());

app.get('/health', (req, res) => {
  res.json({ status: 'ok', service: process.env.SERVICE_NAME || 'mobile-money-simulator', version: '1.0.0' });
});

// Generic payment initiation
app.post('/api/v1/payments', (req, res) => {
  const { amount, currency, phone, reference, callback_url } = req.body;
  const transactionId = 'txn_' + uuidv4().replace(/-/g, '').slice(0, 24);
  
  transactions.set(transactionId, {
    ...req.body,
    transactionId,
    status: 'PENDING',
    provider: process.env.PROVIDER_NAME || 'Generic',
    createdAt: new Date().toISOString()
  });
  
  // Simulate async completion
  setTimeout(() => {
    const txn = transactions.get(transactionId);
    if (txn) {
      txn.status = Math.random() > 0.1 ? 'COMPLETED' : 'FAILED';
      txn.providerRef = 'PRV_' + Date.now();
      if (txn.callback_url) {
        console.log(`Callback to ${txn.callback_url}:`, { transactionId, status: txn.status });
      }
    }
  }, 2000 + Math.random() * 3000);
  
  res.status(202).json({
    transactionId,
    status: 'PENDING',
    message: 'Payment initiated'
  });
});

// Query transaction
app.get('/api/v1/payments/:transactionId', (req, res) => {
  const txn = transactions.get(req.params.transactionId);
  if (txn) {
    res.json(txn);
  } else {
    res.status(404).json({ error: 'Transaction not found' });
  }
});

// USSD simulation
app.post('/api/v1/ussd', (req, res) => {
  const { sessionId, phoneNumber, text } = req.body;
  
  // Simple USSD menu simulation
  let response = '';
  const inputs = text.split('*').filter(Boolean);
  
  if (inputs.length === 0) {
    response = `CON Welcome to ${process.env.PROVIDER_NAME || 'Mobile Money'}\n1. Send Money\n2. Check Balance\n3. Buy Airtime\n4. Pay Bill\n0. Exit`;
  } else if (inputs[0] === '1' && inputs.length === 1) {
    response = 'CON Enter recipient phone number:';
  } else if (inputs[0] === '1' && inputs.length === 2) {
    response = 'CON Enter amount:';
  } else if (inputs[0] === '1' && inputs.length === 3) {
    response = `CON Confirm sending ${inputs[2]} to ${inputs[1]}?\n1. Yes\n2. No`;
  } else if (inputs[0] === '1' && inputs.length === 4 && inputs[3] === '1') {
    response = 'END Transaction successful! Reference: TXN' + Date.now().toString().slice(-8);
  } else if (inputs[0] === '2') {
    response = 'END Your balance is KES 15,000.00';
  } else {
    response = 'END Invalid option. Try again.';
  }
  
  res.set('Content-Type', 'text/plain');
  res.send(response);
});

app.listen(PORT, '0.0.0.0', () => {
  console.log(`${process.env.PROVIDER_NAME || 'Mobile Money'} Simulator running on port ${PORT}`);
});
EOF

# Set environment variables for the simulator
export SERVICE_NAME=$NAME
export PROVIDER_NAME=$PROVIDER
export PORT=$PORT

npm install
npm start &
sleep 3
curl -f http://localhost:$PORT/health
echo "$PROVIDER Mobile Money Sandbox ($NAME) setup complete!"