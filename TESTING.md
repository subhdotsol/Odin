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

You can use the client in two ways: **Programmatic Mode** (recommended for quick testing) or **CLI Mode** (for custom parameters).

### Programmatic Mode (Recommended)

Simply run the client without any arguments:
```bash
cargo run --bin odin-client
```

The client will use hardcoded values from `src/client.rs` (lines 51-56):
```rust
let tx_signature = "5mEjz...".to_string();
let rpc = "https://api.mainnet-beta.solana.com".to_string();
let log_filter = "".to_string(); // Empty = no filter
let cu_logs = false; // true = include compute unit logs
let raw_logs = false; // true = show raw transaction logs
```

**To test different transactions:** Edit these values in `src/client.rs` and run again.

---

### CLI Mode

Pass arguments via command line for custom testing.

#### Basic Usage

```bash
cargo run --bin odin-client -- --tx-sig 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY
```

#### With Compute Unit Logs

```bash
cargo run --bin odin-client -- \
  --tx-sig 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY \
  --include-cu-logs
```

#### With Log Filter

```bash
cargo run --bin odin-client -- \
  --tx-sig 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY \
  --filter "Instruction"
```

#### With Custom RPC URL

```bash
cargo run --bin odin-client -- \
  --tx-sig YOUR_TX_SIG \
  --rpc-url https://api.devnet.solana.com
```

#### All Options Together

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
| `--tx-sig` | `-t` | Transaction signature (optional in programmatic mode) | - |
| `--rpc-url` | `-r` | Solana RPC URL | `https://api.mainnet-beta.solana.com` |
| `--filter` | `-f` | Case-insensitive log filter | (empty) |
| `--include-cu-logs` | `-c` | Include compute unit logs | `false` |
| `--no-raw-logs` | - | Hide raw transaction logs | `false` (shows by default) |
| `--server` | `-s` | Server address | `http://[::1]:50051` |
| `--programmatic` | `-p` | Force programmatic mode | `false` |
| `--stream` | - | Enable streaming mode | `false` |
| `--program` | - | Program address to stream (required with --stream) | - |

## Output Sections

The client displays transaction data in up to three sections:

1. **⚡ Compute Unit Logs** - Shows compute units consumed per program (if `--include-cu-logs` is used)
2. **📋 Program Instruction Logs** - Filtered program logs (only "Program log:" entries, filtered by `--filter` if provided)
3. **📜 Raw Transaction Logs** - Complete unfiltered logs (hidden if `--no-raw-logs` is used)

## Example Output

```
� Using PROGRAMMATIC mode (hardcoded values)

�🔌 Connecting to Odin server at: http://[::1]:50051
✅ Connected successfully!

📡 Fetching logs for transaction: 5mEjz...
🌐 Using RPC: https://api.mainnet-beta.solana.com
⚡ Including compute unit logs

⏳ Requesting transaction logs...

⚡ Compute Unit Logs:
================================================================================
Program ID: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  Consumed: 4645 compute units
Program ID: ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
  Consumed: 24988 compute units

📋 Program Instruction Logs:
================================================================================
[1] Create
[2] Instruction: GetAccountDataSize
[3] Initialize the associated token account
[4] Instruction: InitializeImmutableOwner
[5] Instruction: Transfer

📜 Raw Transaction Logs:
================================================================================
[1] Program ComputeBudget111111111111111111111111111111 invoke [1]
[2] Program ComputeBudget111111111111111111111111111111 success
[3] Program 11111111111111111111111111111111 invoke [1]
[4] Program log: Create
[5] Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]
... (complete transaction logs)

✅ Done!
```

## Customizing Output

Use CLI flags to control what's displayed:

```bash
# Hide raw logs
cargo run --bin odin-client -- --tx-sig YOUR_TX_SIG --no-raw-logs

# Show compute units
cargo run --bin odin-client -- --tx-sig YOUR_TX_SIG --include-cu-logs

# Filter program logs
cargo run --bin odin-client -- --tx-sig YOUR_TX_SIG --filter "Instruction"

# Combine options
cargo run --bin odin-client -- --tx-sig YOUR_TX_SIG --include-cu-logs --filter "Transfer" --no-raw-logs
```

## Quick Test Command

Use this ready-to-go command with a known transaction:
```bash
cargo run --bin odin-client -- -t 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY -c
```

Or just run without arguments for programmatic mode:
```bash
cargo run --bin odin-client
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

**"Address already in use"**
- Another server instance is running on port 50051
- Stop the existing server with Ctrl+C or kill the process
- Use `lsof -i :50051` to find the process ID

---

## Streaming Mode

The server supports **real-time streaming** of transaction logs for a specific program address.

### CLI Mode - Streaming

**Default (Token Program):**
```bash
cargo run --bin odin-client -- --stream
```

**Custom Program:**
```bash
cargo run --bin odin-client -- --stream --program YOUR_PROGRAM_ADDRESS
```

**With Compute Units and Filter:**
```bash
cargo run --bin odin-client -- --stream --program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA --include-cu-logs --filter "Transfer"
```

**Without Raw Logs:**
```bash
cargo run --bin odin-client -- --stream --program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA --no-raw-logs
```

### Popular Programs to Monitor

**Memo Program (Moderate Activity):**
```bash
cargo run --bin odin-client -- --stream --program MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr --include-cu-logs
```

**Token Program (Very Active):**
```bash
cargo run --bin odin-client -- --stream --program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA --include-cu-logs --no-raw-logs
```

**System Program:**
```bash
cargo run --bin odin-client -- --stream --program 11111111111111111111111111111111 --no-raw-logs
```

**Raydium AMM:**
```bash
cargo run --bin odin-client -- --stream --program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 --include-cu-logs
```

### Expected Streaming Output

```
🌊 STREAMING MODE
📡 Program: MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr
🌐 RPC: https://api.mainnet-beta.solana.com
⚡ Including compute unit logs

⏳ Subscribing to real-time logs...

✅ Subscribed! Waiting for transactions...

================================================================================
📨 Transaction #1: 5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY
🕐 Timestamp: 2025-12-13T09:41:12.345Z
================================================================================

⚡ Compute Unit Logs:
================================================================================
Program ID: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
  Consumed: 36246 compute units
Program ID: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
  Consumed: 4645 compute units

📋 Program Instruction Logs:
================================================================================
[1] Create
[2] Instruction: GetAccountDataSize
[3] Initialize the associated token account
[4] Instruction: InitializeImmutableOwner
[5] Instruction: Transfer

📜 Raw Transaction Logs:
================================================================================
[1] Program ComputeBudget111111111111111111111111111111 invoke [1]
[2] Program ComputeBudget111111111111111111111111111111 success
[3] Program 11111111111111111111111111111111 invoke [1]
...

================================================================================
📨 Transaction #2: ...
```

Press `Ctrl+C` to stop streaming.

### Postman - Streaming

**Server:** `localhost:50051`  
**Method:** `odin.SolanaTxLog/StreamProgramLogs`  
**Disable TLS:** Yes (use plaintext)

**Message:**
```json
{
  "rpc_url": "",
  "program_address": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
  "include_cu_logs": true,
  "filter": ""
}
```

### grpcurl - Streaming

```bash
grpcurl -plaintext -d '{
  "rpc_url": "",
  "program_address": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr",
  "include_cu_logs": true,
  "filter": ""
}' localhost:50051 odin.SolanaTxLog/StreamProgramLogs
```

---

## Testing Summary

| Feature | Programmatic | CLI | Postman | grpcurl |
|---------|-------------|-----|---------|---------|
| **Unary GetTxLogs** | ✅ | ✅ | ✅ | ✅ |
| **Streaming** | ⚠️ (CLI only) | ✅ | ✅ | ✅ |

**Note:** For programmatic streaming, use CLI mode with `--stream` flag.
