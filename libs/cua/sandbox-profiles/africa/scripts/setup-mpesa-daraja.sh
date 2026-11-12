#!/usr/bin/env bash
# M-Pesa Daraja Sandbox Setup Script
# Installs and configures the Daraja API simulator

set -euo pipefail

echo "Setting up M-Pesa Daraja Sandbox..."

# Install Node.js for the simulator
curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
apt-get update && apt-get install -y nodejs python3 python3-pip

# Create simulator directory
mkdir -p /opt/mpesa-daraja-simulator
cd /opt/mpesa-daraja-simulator

# Create package.json for the simulator
cat > package.json << 'EOF'
{
  "name": "mpesa-daraja-simulator",
  "version": "1.0.0",
  "description": "M-Pesa Daraja API Simulator for Cua Sandbox",
  "main": "server.js",
  "scripts": {
    "start": "node server.js",
    "dev": "nodemon server.js"
  },
  "dependencies": {
    "express": "^4.18.2",
    "cors": "^2.8.5",
    "body-parser": "^1.20.2",
    "axios": "^1.6.0",
    "uuid": "^9.0.0"
  }
}
EOF

# Create the simulator server
cat > server.js << 'EOF'
const express = require('express');
const cors = require('cors');
const bodyParser = require('body-parser');
const { v4: uuidv4 } = require('uuid');

const app = express();
const PORT = process.env.PORT || 8080;

app.use(cors());
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

// In-memory storage for transactions
const transactions = new Map();

// Health check
app.get('/health', (req, res) => {
  res.json({ status: 'ok', service: 'mpesa-daraja-simulator', version: '1.0.0' });
});

// OAuth token endpoint
app.post('/oauth/v1/generate', (req, res) => {
  const { consumer_key, consumer_secret } = req.body;
  
  // In sandbox, accept any credentials
  if (consumer_key && consumer_secret) {
    const access_token = 'sandbox_' + uuidv4().replace(/-/g, '');
    res.json({
      access_token,
      expires_in: '3599'
    });
  } else {
    res.status(400).json({ error: 'invalid_client', message: 'Invalid credentials' });
  }
});

// STK Push endpoint
app.post('/mpesa/stkpush/v1/processrequest', (req, res) => {
  const { BusinessShortCode, Password, Timestamp, TransactionType, Amount, PartyA, PartyB, PhoneNumber, CallBackURL, AccountReference, TransactionDesc } = req.body;
  
  const checkoutRequestID = 'ws_CO_' + Date.now() + '_' + uuidv4().slice(0, 8);
  const merchantRequestID = 'mr_' + uuidv4().slice(0, 8);
  
  // Store transaction for callback simulation
  transactions.set(checkoutRequestID, {
    ...req.body,
    checkoutRequestID,
    merchantRequestID,
    status: 'pending',
    createdAt: new Date().toISOString()
  });
  
  // Simulate async callback after 2 seconds
  setTimeout(() => {
    const txn = transactions.get(checkoutRequestID);
    if (txn) {
      txn.status = 'completed';
      txn.ResultCode = '0';
      txn.ResultDesc = 'The service request is processed successfully.';
      txn.MpesaReceiptNumber = 'OEO' + Date.now().toString().slice(-10);
      txn.CallbackMetadata = {
        Item: [
          { Name: 'Amount', Value: Amount },
          { Name: 'MpesaReceiptNumber', Value: txn.MpesaReceiptNumber },
          { Name: 'TransactionDate', Value: new Date().toISOString().replace(/[-:]/g, '').split('.')[0] },
          { Name: 'PhoneNumber', Value: PhoneNumber }
        ]
      };
      
      // In real implementation, this would POST to CallBackURL
      console.log('STK Push callback for:', checkoutRequestID, txn);
    }
  }, 2000);
  
  res.json({
    MerchantRequestID: merchantRequestID,
    CheckoutRequestID: checkoutRequestID,
    ResponseCode: '0',
    ResponseDescription: 'Success. Request accepted for processing',
    CustomerMessage: 'Success. Request accepted for processing'
  });
});

// B2C Payment endpoint
app.post('/mpesa/b2c/v1/paymentrequest', (req, res) => {
  const { InitiatorName, SecurityCredential, CommandID, Amount, PartyA, PartyB, Remarks, QueueTimeOutURL, ResultURL, Occasion } = req.body;
  
  const conversationID = 'AG_' + Date.now() + '_' + uuidv4().slice(0, 8);
  const originatorConversationID = uuidv4();
  
  res.json({
    ConversationID: conversationID,
    OriginatorConversationID: originatorConversationID,
    ResponseCode: '0',
    ResponseDescription: 'Accept the service request successfully.'
  });
});

// C2B Register URL
app.post('/mpesa/c2b/v1/registerurl', (req, res) => {
  const { ShortCode, ResponseType, ConfirmationURL, ValidationURL } = req.body;
  
  res.json({
    OriginatorConversationID: uuidv4(),
    ConversationID: uuidv4(),
    ResponseCode: '0',
    ResponseDescription: 'Success. Request accepted for processing'
  });
});

// C2B Simulation (for testing)
app.post('/mpesa/c2b/v1/simulate', (req, res) => {
  const { ShortCode, CommandID, Amount, Msisdn, BillRefNumber } = req.body;
  
  res.json({
    OriginatorConversationID: uuidv4(),
    ConversationID: uuidv4(),
    ResponseCode: '0',
    ResponseDescription: 'Success. Request accepted for processing'
  });
});

// Transaction status query
app.post('/mpesa/stkpushquery/v1/query', (req, res) => {
  const { BusinessShortCode, Password, Timestamp, CheckoutRequestID } = req.body;
  
  const txn = transactions.get(CheckoutRequestID);
  if (txn && txn.status === 'completed') {
    res.json({
      ResponseCode: '0',
      ResponseDescription: 'The service request is processed successfully.',
      MerchantRequestID: txn.merchantRequestID,
      CheckoutRequestID: CheckoutRequestID,
      ResultCode: '0',
      ResultDesc: 'The service request is processed successfully.'
    });
  } else {
    res.json({
      ResponseCode: '0',
      ResponseDescription: 'The service request is still being processed.',
      MerchantRequestID: txn?.merchantRequestID || '',
      CheckoutRequestID: CheckoutRequestID,
      ResultCode: '1032',
      ResultDesc: 'Request cancelled by user'
    });
  }
});

// Reversal
app.post('/mpesa/reversal/v1/request', (req, res) => {
  res.json({
    OriginatorConversationID: uuidv4(),
    ConversationID: uuidv4(),
    ResponseCode: '0',
    ResponseDescription: 'Success. Request accepted for processing'
  });
});

app.listen(PORT, '0.0.0.0', () => {
  console.log(`M-Pesa Daraja Simulator running on port ${PORT}`);
});
EOF

# Install dependencies and start
npm install
npm start &

# Wait for server to start
sleep 3

# Verify
curl -f http://localhost:8080/health

echo "M-Pesa Daraja Sandbox setup complete!"