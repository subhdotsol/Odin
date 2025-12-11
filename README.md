


# Odin - Solana Transaction Log gRPC Service

![Odin](https://wallpapercave.com/wp/wp2621046.jpg)  

**Odin** is a Rust-based gRPC service for fetching, parsing, and streaming Solana transaction logs.  
It supports:
- One-off transaction log fetching.
- Real-time log streaming for a given Solana program.
- Compute unit (CU) logging.
- Anchor event detection and parsing.

---

## Features

- Fetch transaction logs by **transaction signature**.
- Stream logs for a **program address** in real-time via gRPC.
- Include **compute unit logs** per program.
- Detect **Anchor events** emitted during program execution.
- Configurable Solana RPC URL (defaults to `https://api.mainnet-beta.solana.com`).

---

## Installation

**Prerequisites:**

- Rust (latest stable)
- Cargo
- Solana RPC access (Mainnet Beta by default)

```bash
git clone https://github.com/yourusername/odin.git
cd odin
cargo build --release
````

The binary will be located at:

```text
./target/release/odin
```

---

## gRPC API

Odin exposes the following gRPC methods:

### 1. `GetTxLogs` (Unary)

Fetch logs for a specific transaction signature.

```proto
rpc GetTxLogs(TxRequest) returns (TxResponse);
```

**TxRequest:**

| Field           | Type   | Description                                              |
| --------------- | ------ | -------------------------------------------------------- |
| rpc_url         | string | Optional. Solana RPC endpoint. Defaults to Mainnet Beta. |
| tx_sig          | string | Optional. Transaction signature to fetch.                |
| include_cu_logs | bool   | Optional. Include compute unit logs.                     |
| filter          | string | Optional. Filter logs containing this string.            |

**TxResponse:**

* `logs`: raw log lines
* `compute_units`: optional CU logs
* `anchor_events`: optional Anchor events

---

### 2. `StreamProgramLogs` (Server-Side Streaming)

Stream logs for every transaction invoking a given program address.

```proto
rpc StreamProgramLogs(StreamProgramRequest) returns (stream LogMessage);
```

**StreamProgramRequest:**

| Field           | Type   | Description                                              |
| --------------- | ------ | -------------------------------------------------------- |
| rpc_url         | string | Optional. Solana RPC endpoint. Defaults to Mainnet Beta. |
| program_address | string | Required. Solana program pubkey.                         |
| include_cu_logs | bool   | Optional. Include compute unit logs.                     |

**LogMessage:**

* `log_line`: raw log line
* `program_id`: program pubkey
* `consumed`: optional compute units consumed
* `anchor_event`: optional Anchor event object

---

## Usage Example (Rust Client)

```rust
use tonic::transport::Channel;
use odin::solana_logs::solana_tx_log_client::SolanaTxLogClient;
use odin::solana_logs::{TxRequest, StreamProgramRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SolanaTxLogClient::connect("http://[::1]:50051").await?;

    // Fetch logs for a transaction
    let tx_response = client
        .get_tx_logs(TxRequest {
            rpc_url: "".into(),
            tx_sig: "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY".into(),
            include_cu_logs: true,
            filter: None,
        })
        .await?
        .into_inner();

    println!("Transaction Logs: {:?}", tx_response.logs);

    // Stream logs for a program
    let mut stream = client
        .stream_program_logs(StreamProgramRequest {
            rpc_url: "".into(),
            program_address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA".into(),
            include_cu_logs: true,
        })
        .await?
        .into_inner();

    while let Some(log) = stream.message().await? {
        println!("Streamed Log: {:?}", log);
    }

    Ok(())
}
```

---

### Optional: Future Features

* Advanced log filtering by type or event.
* JSON output for logs.
* Multi-program subscription support.

---

*Odin is named after the Norse god of wisdom, knowledge, and magic – guiding you through Solana logs with foresight.*
