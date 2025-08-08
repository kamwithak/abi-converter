# Monaco Protocol Core Contract ABIs

This directory contains the Application Binary Interface (ABI) files for the four core contracts of the Monaco Protocol:

## Core Contracts

### 1. CLOB.abi.json
- **Central Limit Order Book** contract
- Handles order placement, matching, and cancellation
- Includes batch order functionality
- Size: ~47KB, 2,448 lines

### 2. Vault.abi.json  
- **Vault** contract for asset custody and settlement
- Manages token transfers and fee collection
- Handles settlement processing
- Size: ~47KB, 2,371 lines

### 3. State.abi.json
- **State** contract for order and system state management
- Tracks order lifecycle and status
- Manages trader permissions and blacklisting
- Size: ~27KB, 1,352 lines

### 4. Book.abi.json
- **Book** contract for order book management
- Handles price-time priority matching
- Manages bid/ask levels and order storage
- Size: ~19KB, 970 lines

## Usage

These ABI files can be used with:
- **Frontend applications** (React, Vue, etc.) with ethers.js or web3.js
- **Backend services** for contract interaction
- **SDKs and APIs** for protocol integration
- **Testing frameworks** and development tools

## Example Usage with ethers.js

```javascript
import { ethers } from 'ethers';
import ClobAbi from './abis/CLOB.abi.json';

const provider = new ethers.providers.JsonRpcProvider('RPC_URL');
const clobContract = new ethers.Contract(CONTRACT_ADDRESS, ClobAbi, provider);

// Example: Place a batch of orders
const result = await clobContract.placeBatchOrders(
  baseTokens, 
  quoteTokens, 
  isBuys, 
  prices, 
  quantities
);
```

## Generated From
- **Source**: Hardhat compilation artifacts
- **Compiler**: Solidity ^0.8.20
- **EVM Target**: Paris
- **Generated**: Auto-extracted using `jq '.abi'` 