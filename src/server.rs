use tonic::{Request, Response, Status, transport::Server};

// Include the generated protobuf code from proto/odin.proto
pub mod odin {
    tonic::include_proto!("odin");
}

// Import the generated types and server trait
use odin::solana_tx_log_server::{SolanaTxLog, SolanaTxLogServer};
use odin::{GetTxRequest, GetTxResponse, StreamProgramRequest, LogMessage, ComputeUnitLog, AnchorEvent};

// The following types are now available from the generated code:
// - GetTxRequest, GetTxResponse
// - StreamProgramRequest
// - LogMessage, ComputeUnitLog, AnchorEvent
// - solana_tx_log_server::SolanaTxLog (the server trait)
