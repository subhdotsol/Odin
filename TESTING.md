# Testing the Odin gRPC Server

This guide shows you how to test your transaction log parser using the client.

## Step 1: Start the Server

In one terminal, run:
```bash
cargo run --bin odin-server
```

You should see:
```
🚀 Odin gRPC Server starting on [::1]:50051
📡 Ready to serve transaction logs...
```

## Step 2: Test with the Client

In another terminal, run the client with a transaction signature.

### Basic Usage

```bash
cargo run --bin odin-client -- --tx-sig 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY
```

### With Compute Unit Logs

```bash
cargo run --bin odin-client -- \
  --tx-sig 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY \
  --include-cu-logs
```

### With Log Filter

```bash
cargo run --bin odin-client -- \
  --tx-sig 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY \
  --filter "Instruction"
```

### With Custom RPC URL

```bash
cargo run --bin odin-client -- \
  --tx-sig YOUR_TX_SIG \
  --rpc-url https://api.devnet.solana.com
```

### All Options Together

```bash
cargo run --bin odin-client -- \
  --tx-sig 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY \
  --rpc-url https://api.mainnet-beta.solana.com \
  --filter "Program" \
  --include-cu-logs \
  --server http://[::1]:50051
```

## Client Options

| Flag | Short | Description | Default |
|------|-------|-------------|---------|
| `--tx-sig` | `-t` | Transaction signature (required) | - |
| `--rpc-url` | `-r` | Solana RPC URL | `https://api.mainnet-beta.solana.com` |
| `--filter` | `-f` | Case-insensitive log filter | (empty) |
| `--include-cu-logs` | `-c` | Include compute unit logs | `false` |
| `--server` | `-s` | Server address | `http://[::1]:50051` |

## Example Output

```
🔌 Connecting to Odin server at: http://[::1]:50051
✅ Connected successfully!

📡 Fetching logs for transaction: 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY
🌐 Using RPC: https://api.mainnet-beta.solana.com
⚡ Including compute unit logs

⏳ Requesting transaction logs...

📋 Transaction Logs:
================================================================================
[1] Instruction: Initialize
[2] Instruction: Transfer
[3] Instruction: Close

⚡ Compute Unit Logs:
================================================================================
Program ID: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  Consumed: 4645 compute units

✅ Done!
```

## Quick Test Command

Use this ready-to-go command with a known transaction:
```bash
cargo run --bin odin-client -- -t 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY -c
```

## Troubleshooting

**"Connection refused"**
- Make sure the server is running first
- Check that you're using the correct server address

**"Invalid transaction signature"**
- Verify the transaction signature is correct
- Make sure it exists on the network you're querying (mainnet/devnet)

**"No logs found"**
- The transaction might not have any program logs
- Your filter might be too restrictive
- Try without a filter first
