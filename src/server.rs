use tonic::{Request, Response, Status, transport::Server};
use tokio_stream::wrappers::ReceiverStream;

// Include the generated protobuf code from proto/odin.proto
pub mod proto {
    tonic::include_proto!("odin");
}

// Import the generated types and server trait
use proto::solana_tx_log_server::{SolanaTxLog, SolanaTxLogServer};
use proto::{GetTxRequest, GetTxResponse, StreamProgramRequest, LogMessage, ComputeUnitLog};

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
    type StreamProgramLogsStream = ReceiverStream<Result<LogMessage, Status>>;

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
        };

        Ok(Response::new(response))
    }

    /// Stream logs for all transactions invoking a program address
    /// This will be implemented later as per user's request
    async fn stream_program_logs(
        &self,
        _request: Request<StreamProgramRequest>,
    ) -> Result<Response<Self::StreamProgramLogsStream>, Status> {
        Err(Status::unimplemented("StreamProgramLogs will be implemented later"))
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
