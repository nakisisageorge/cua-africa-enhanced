#!/usr/bin/env bash
# MTN MoMo Sandbox Setup Script

set -euo pipefail

echo "Setting up MTN MoMo Sandbox..."

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
apt-get update && apt-get install -y nodejs python3 python3-pip

mkdir -p /opt/mtn-momo-simulator
cd /opt/mtn-momo-simulator

cat > package.json << 'EOF'
{
  "name": "mtn-momo-simulator",
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
const PORT = process.env.PORT || 8081;
const collections = new Map();
const disbursements = new Map();

app.use(cors());
app.use(bodyParser.json());

app.get('/health', (req, res) => {
  res.json({ status: 'ok', service: 'mtn-momo-simulator', version: '1.0.0' });
});

// OAuth token
app.post('/collection/token', (req, res) => {
  const auth = req.headers.authorization;
  if (auth && auth.startsWith('Basic ')) {
    const access_token = 'momo_' + uuidv4().replace(/-/g, '');
    res.json({ access_token, token_type: 'Bearer', expires_in: 3600 });
  } else {
    res.status(401).json({ error: 'unauthorized' });
  }
});

// Request to pay (collection)
app.post('/collection/v1_0/requesttopay', (req, res) => {
  const { amount, currency, externalId, payer, payerMessage, payeeNote } = req.body;
  const referenceId = uuidv4();
  
  collections.set(referenceId, {
    ...req.body,
    referenceId,
    status: 'PENDING',
    createdAt: new Date().toISOString()
  });
  
  // Simulate completion after 3 seconds
  setTimeout(() => {
    const coll = collections.get(referenceId);
    if (coll) {
      coll.status = 'SUCCESSFUL';
      coll.financialTransactionId = 'FT_' + Date.now();
    }
  }, 3000);
  
  res.status(202).json({ referenceId });
});

// Get collection status
app.get('/collection/v1_0/requesttopay/:referenceId', (req, res) => {
  const coll = collections.get(req.params.referenceId);
  if (coll) {
    res.json({
      amount: coll.amount,
      currency: coll.currency,
      externalId: coll.externalId,
      payer: coll.payer,
      status: coll.status,
      financialTransactionId: coll.financialTransactionId
    });
  } else {
    res.status(404).json({ error: 'not found' });
  }
});

// Disbursement token
app.post('/disbursement/token', (req, res) => {
  const auth = req.headers.authorization;
  if (auth && auth.startsWith('Basic ')) {
    const access_token = 'momo_disb_' + uuidv4().replace(/-/g, '');
    res.json({ access_token, token_type: 'Bearer', expires_in: 3600 });
  } else {
    res.status(401).json({ error: 'unauthorized' });
  }
});

// Transfer (disbursement)
app.post('/disbursement/v1_0/transfer', (req, res) => {
  const { amount, currency, externalId, payee, payerMessage, payeeNote } = req.body;
  const referenceId = uuidv4();
  
  disbursements.set(referenceId, {
    ...req.body,
    referenceId,
    status: 'PENDING',
    createdAt: new Date().toISOString()
  });
  
  setTimeout(() => {
    const disb = disbursements.get(referenceId);
    if (disb) {
      disb.status = 'SUCCESSFUL';
      disb.financialTransactionId = 'FTD_' + Date.now();
    }
  }, 3000);
  
  res.status(202).json({ referenceId });
});

// Get disbursement status
app.get('/disbursement/v1_0/transfer/:referenceId', (req, res) => {
  const disb = disbursements.get(req.params.referenceId);
  if (disb) {
    res.json({
      amount: disb.amount,
      currency: disb.currency,
      externalId: disb.externalId,
      payee: disb.payee,
      status: disb.status,
      financialTransactionId: disb.financialTransactionId
    });
  } else {
    res.status(404).json({ error: 'not found' });
  }
});

// KYC (basic)
app.get('/v1_0/accountholder/msisdn/:msisdn/basicuserinfo', (req, res) => {
  res.json({
    given_name: 'Test',
    family_name: 'User',
    birthdate: '1990-01-01',
    locale: 'en_GH',
    gender: 'M'
  });
});

app.listen(PORT, '0.0.0.0', () => {
  console.log(`MTN MoMo Simulator running on port ${PORT}`);
});
EOF

npm install
npm start &
sleep 3
curl -f http://localhost:8081/health
echo "MTN MoMo Sandbox setup complete!"