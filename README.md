


# Odin - Solana Transaction Log gRPC Service

![Odin](https://wallpapercave.com/wp/wp2621046.jpg)  

**Odin** is a Rust-based gRPC service for fetching, parsing, and streaming Solana transaction logs.  
It supports:
- One-off transaction log fetching with filtering
- Real-time log streaming for a given Solana program (coming soon)
- Compute unit (CU) logging per program
- Raw and filtered transaction logs
- Anchor event detection (coming soon)

---

## Features

- ✅ Fetch transaction logs by **transaction signature**
- ✅ Include **compute unit logs** per program
- ✅ **Raw transaction logs** (complete, unfiltered)
- ✅ **Filtered program logs** (only "Program log:" entries)
- ✅ **Programmatic mode** for easy testing
- ✅ Configurable Solana RPC URL (defaults to Mainnet Beta)
- 🚧 Stream logs for a **program address** in real-time (coming soon)
- 🚧 Detect **Anchor events** emitted during program execution (coming soon)

---

## Installation

**Prerequisites:**

- Rust (latest stable)
- Cargo
- Solana RPC access (Mainnet Beta by default)

```bash
git clone https://github.com/subhdotsol/Odin.git
cd odin
cargo build --release
```

The binaries will be located at:

```text
./target/release/odin-server
./target/release/odin-client
```

---

## Quick Start

### 1. Start the Server

```bash
cargo run --bin odin-server
```

Output:
```
🚀 Odin gRPC Server starting on [::1]:50051
📡 Ready to serve transaction logs...
```

### 2. Test with the Client

**Programmatic Mode** (easiest):
```bash
cargo run --bin odin-client
```

**CLI Mode** (custom parameters):
```bash
cargo run --bin odin-client -- -t YOUR_TX_SIGNATURE -c
```

See [TESTING.md](TESTING.md) for detailed usage examples.

---

## gRPC API

Odin exposes the following gRPC methods defined in [`proto/odin.proto`](proto/odin.proto):

### 1. `GetTxLogs` (Unary) ✅

Fetch logs for a specific transaction signature.

```proto
rpc GetTxLogs(GetTxRequest) returns (GetTxResponse);
```

**GetTxRequest:**

| Field           | Type   | Description                                              |
| --------------- | ------ | -------------------------------------------------------- |
| rpc_url         | string | Optional. Solana RPC endpoint. Defaults to Mainnet Beta. |
| tx_sig          | string | Required. Transaction signature to fetch.                |
| include_cu_logs | bool   | Optional. Include compute unit logs.                     |
| filter          | string | Optional. Filter logs containing this string (case-insensitive). |

**GetTxResponse:**

| Field           | Type                  | Description                                    |
| --------------- | --------------------- | ---------------------------------------------- |
| logs            | repeated string       | Filtered program log lines (only "Program log:") |
| compute_units   | repeated ComputeUnitLog | Compute unit consumption per program          |
| anchor_events   | repeated AnchorEvent  | Anchor events (coming soon)                    |
| raw_logs        | repeated string       | Complete unfiltered transaction logs           |

**ComputeUnitLog:**

| Field      | Type   | Description                    |
| ---------- | ------ | ------------------------------ |
| program_id | string | Program public key             |
| consumed   | uint64 | Compute units consumed         |

---

### 2. `StreamProgramLogs` (Server-Side Streaming) 🚧

Stream logs for every transaction invoking a given program address.

```proto
rpc StreamProgramLogs(StreamProgramRequest) returns (stream LogMessage);
```

**Status:** Coming soon

---

## Usage Example (Rust Client)

```rust
use odin::proto::solana_tx_log_client::SolanaTxLogClient;
use odin::proto::GetTxRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SolanaTxLogClient::connect("http://[::1]:50051").await?;

    // Fetch logs for a transaction
    let response = client
        .get_tx_logs(GetTxRequest {
            rpc_url: "".into(),
            tx_sig: "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY".into(),
            include_cu_logs: true,
            filter: "".into(),
        })
        .await?
        .into_inner();

    println!("Program Logs: {:?}", response.logs);
    println!("Raw Logs: {:?}", response.raw_logs);
    println!("Compute Units: {:?}", response.compute_units);

    Ok(())
}
```

---

## Project Structure

```
odin/
├── proto/
│   └── odin.proto          # gRPC service definitions
├── src/
│   ├── lib.rs              # Library entry point
│   ├── parser.rs           # Transaction log parser
│   ├── server.rs           # gRPC server implementation
│   └── client.rs           # gRPC client for testing
├── build.rs                # Proto compilation script
├── Cargo.toml              # Dependencies
├── TESTING.md              # Testing guide
├── errors.md               # Common errors and solutions
└── README.md               # This file
```

---

## Documentation

- **[TESTING.md](TESTING.md)** - Complete testing guide with examples
- **[errors.md](errors.md)** - Common errors and how to fix them
- **[proto/odin.proto](proto/odin.proto)** - gRPC API definitions

---

## Future Features

* ✅ ~~Raw transaction logs~~
* ✅ ~~Compute unit logging~~
* ✅ ~~Programmatic client mode~~
* 🚧 Server-side streaming for program logs
* 🚧 Anchor event detection and parsing
* 🚧 Advanced log filtering by type or event
* 🚧 JSON output format option
* 🚧 Multi-program subscription support

---

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

*Odin is named after the Norse god of wisdom, knowledge, and magic – guiding you through Solana logs with foresight.*
