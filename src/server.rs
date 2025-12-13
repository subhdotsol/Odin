use tonic::{Request, Response, Status, transport::Server};
use tokio_stream::wrappers::ReceiverStream;

// Include the generated protobuf code from proto/odin.proto
pub mod proto {
    tonic::include_proto!("odin");
}

// Import the generated types and server trait
use proto::solana_tx_log_server::{SolanaTxLog, SolanaTxLogServer};
use proto::{GetTxRequest, GetTxResponse, StreamProgramRequest, ComputeUnitLog};

// Import the parser module from the odin crate
use odin::parser::TxLogParser;

// Default RPC URL for Solana Mainnet Beta
const DEFAULT_RPC_URL: &str = "https://api.mainnet-beta.solana.com";

// Default server address
const DEFAULT_SERVER_ADDR: &str = "[::1]:50051";

/// OdinService implements the SolanaTxLog gRPC service
#[derive(Debug, Default)]
pub struct OdinService;

#[tonic::async_trait]
impl SolanaTxLog for OdinService {
    type StreamProgramLogsStream = ReceiverStream<Result<proto::StreamTransactionResponse, Status>>;

    /// Fetch transaction logs for a given transaction signature
    async fn get_tx_logs(
        &self,
        request: Request<GetTxRequest>,
    ) -> Result<Response<GetTxResponse>, Status> {
        let req = request.into_inner();

        // Use provided RPC URL or default to Mainnet Beta
        let rpc_url = if req.rpc_url.is_empty() {
            DEFAULT_RPC_URL.to_string()
        } else {
            req.rpc_url
        };

        // Validate transaction signature
        if req.tx_sig.is_empty() {
            return Err(Status::invalid_argument("Transaction signature is required"));
        }

        // Prepare filter (None if empty)
        let filter = if req.filter.is_empty() {
            None
        } else {
            Some(req.filter.as_str())
        };

        // Create parser instance
        let mut parser = TxLogParser::new(
            rpc_url,
            req.tx_sig.clone(),
            filter,
            req.include_cu_logs,
        );

        // Parse the transaction logs
        parser.parse().await.map_err(|e| {
            Status::internal(format!("Failed to parse transaction logs: {}", e))
        })?;

        // Get the parsed logs
        let logs = parser.get_tx_logs();
        let raw_logs = parser.get_raw_logs();

        // Build compute unit logs if requested
        let mut compute_units = Vec::new();
        if req.include_cu_logs {
            let cu_logs = parser.get_cu_logs();
            for (program_id, consumed) in cu_logs.iter() {
                compute_units.push(ComputeUnitLog {
                    program_id: program_id.to_string(),
                    consumed: *consumed,
                });
            }
        }

        // Build the response
        let response = GetTxResponse {
            logs,
            compute_units,
            anchor_events: Vec::new(), // TODO: Implement anchor event parsing later
            raw_logs,
        };

        Ok(Response::new(response))
    }

    /// Stream logs for all transactions invoking a program address
    async fn stream_program_logs(
        &self,
        request: Request<StreamProgramRequest>,
    ) -> Result<Response<Self::StreamProgramLogsStream>, Status> {
        use futures_util::{SinkExt, StreamExt};
        use tokio_tungstenite::{connect_async, tungstenite::Message};
        use serde_json::json;

        let req = request.into_inner();

        // Validate program address
        if req.program_address.is_empty() {
            return Err(Status::invalid_argument("Program address is required"));
        }

        // Determine WebSocket URL from RPC URL
        let ws_url = if req.rpc_url.is_empty() {
            "wss://api.mainnet-beta.solana.com".to_string()
        } else {
            // Convert HTTP(S) URL to WS(S)
            req.rpc_url
                .replace("https://", "wss://")
                .replace("http://", "ws://")
        };

        // Use HTTP RPC URL for fetching transaction details
        let rpc_url = if req.rpc_url.is_empty() {
            DEFAULT_RPC_URL.to_string()
        } else {
            req.rpc_url.clone()
        };

        println!("🔌 Connecting to WebSocket: {}", ws_url);
        println!("📡 Subscribing to program: {}", req.program_address);

        // Prepare filter (None if empty)
        let filter = if req.filter.is_empty() {
            None
        } else {
            Some(req.filter.clone())
        };

        // Create channel for streaming
        let (tx, rx) = tokio::sync::mpsc::channel(128);

        // Spawn WebSocket task
        tokio::spawn(async move {
            // Connect to WebSocket
            let ws_stream = match connect_async(&ws_url).await {
                Ok((stream, _)) => stream,
                Err(e) => {
                    eprintln!("❌ WebSocket connection failed: {}", e);
                    let _ = tx.send(Err(Status::internal(format!("WebSocket connection failed: {}", e)))).await;
                    return;
                }
            };

            let (mut write, mut read) = ws_stream.split();

            // Subscribe to logs for the program
            let subscribe_msg = json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "logsSubscribe",
                "params": [
                    {
                        "mentions": [req.program_address.clone()]
                    },
                    {
                        "commitment": "confirmed"
                    }
                ]
            });

            if let Err(e) = write.send(Message::Text(subscribe_msg.to_string())).await {
                eprintln!("❌ Failed to send subscription: {}", e);
                let _ = tx.send(Err(Status::internal("Failed to subscribe"))).await;
                return;
            }

            println!("✅ Subscribed successfully!");

            // Process incoming messages
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        // Parse WebSocket message
                        if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                            // Check if it's a log notification
                            if value.get("method").and_then(|m| m.as_str()) == Some("logsNotification") {
                                // Extract signature
                                let signature = value
                                    .pointer("/params/result/value/signature")
                                    .and_then(|s| s.as_str())
                                    .unwrap_or("unknown");

                                if signature == "unknown" {
                                    continue;
                                }

                                println!("📨 Processing transaction: {}", signature);

                                // Parse the full transaction using TxLogParser
                                let mut parser = TxLogParser::new(
                                    rpc_url.clone(),
                                    signature.to_string(),
                                    filter.as_deref(),
                                    req.include_cu_logs,
                                );

                                match parser.parse().await {
                                    Ok(_) => {
                                        // Get the parsed logs
                                        let logs = parser.get_tx_logs();
                                        let raw_logs = parser.get_raw_logs();

                                        // Build compute unit logs if requested
                                        let mut compute_units = Vec::new();
                                        if req.include_cu_logs {
                                            let cu_logs = parser.get_cu_logs();
                                            for (program_id, consumed) in cu_logs.iter() {
                                                compute_units.push(ComputeUnitLog {
                                                    program_id: program_id.to_string(),
                                                    consumed: *consumed,
                                                });
                                            }
                                        }

                                        // Build the response
                                        let response = proto::StreamTransactionResponse {
                                            signature: signature.to_string(),
                                            logs,
                                            compute_units,
                                            raw_logs,
                                            timestamp: chrono::Utc::now().to_rfc3339(),
                                        };

                                        if tx.send(Ok(response)).await.is_err() {
                                            // Client disconnected
                                            println!("🔌 Client disconnected");
                                            return;
                                        }

                                        println!("✅ Streamed parsed transaction: {}", signature);
                                    }
                                    Err(e) => {
                                        eprintln!("❌ Failed to parse transaction {}: {}", signature, e);
                                        // Continue streaming even if one transaction fails
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("🔌 WebSocket closed");
                        break;
                    }
                    Err(e) => {
                        eprintln!("❌ WebSocket error: {}", e);
                        let _ = tx.send(Err(Status::internal(format!("WebSocket error: {}", e)))).await;
                        break;
                    }
                    _ => {}
                }
            }

            println!("🛑 Stream ended");
        });

        // Return the stream
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = DEFAULT_SERVER_ADDR.parse()?;
    let service = OdinService::default();

    println!("🚀 Odin gRPC Server starting on {}", addr);
    println!("📡 Ready to serve transaction logs...");

    Server::builder()
        .add_service(SolanaTxLogServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
