use clap::Parser;

// Include the generated protobuf code
pub mod proto {
    tonic::include_proto!("odin");
}

use proto::solana_tx_log_client::SolanaTxLogClient;
use proto::{GetTxRequest, StreamProgramRequest};
use futures_util::StreamExt;

/// Odin gRPC Client - Test the transaction log parser
#[derive(Parser, Debug)]
#[command(name = "odin-client")]
#[command(about = "Client to test Odin gRPC server for Solana transaction logs", long_about = None)]
struct Args {
    /// Use programmatic mode (hardcoded values in code)
    #[arg(short = 'p', long, default_value = "false")]
    programmatic: bool,

    /// Transaction signature to fetch logs for
    #[arg(short, long, default_value = "")]
    tx_sig: String,

    /// Solana RPC URL (optional, defaults to Mainnet Beta)
    #[arg(short, long, default_value = "https://api.mainnet-beta.solana.com")]
    rpc_url: String,

    /// Log filter (case-insensitive, optional)
    #[arg(short, long, default_value = "")]
    filter: String,

    /// Include compute unit logs
    #[arg(short = 'c', long, default_value = "false")]
    include_cu_logs: bool,

    /// Server address to connect to
    #[arg(short, long, default_value = "http://[::1]:50051")]
    server: String,

    /// Enable streaming mode (subscribe to program logs)
    #[arg(long, default_value = "false")]
    stream: bool,

    /// Program address to stream logs for (required in stream mode)
    #[arg(long, default_value = "")]
    program: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // ========================================
    // PROGRAMMATIC MODE - Hardcode your values here!
    // ========================================
    let (tx_sig, rpc_url, filter, include_cu_logs, show_raw_logs) = if args.programmatic || args.tx_sig.is_empty() {
        println!("🔧 Using PROGRAMMATIC mode (hardcoded values)\n");
        
        // 👇 EDIT THESE VALUES TO TEST DIFFERENT TRANSACTIONS
        let tx_signature = "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY".to_string();
        let rpc = "https://api.mainnet-beta.solana.com".to_string();
        let log_filter = "".to_string(); // Empty string = no filter
        let cu_logs = true; // true = include compute unit logs
        let raw_logs = true; // true = show raw transaction logs
        
        (tx_signature, rpc, log_filter, cu_logs, raw_logs)
    } else {
        println!("🔧 Using CLI mode (command-line arguments)\n");
        (args.tx_sig.clone(), args.rpc_url.clone(), args.filter.clone(), args.include_cu_logs, true)
    };

    println!("🔌 Connecting to Odin server at: {}", args.server);

    // Connect to the gRPC server
    let mut client = SolanaTxLogClient::connect(args.server.clone()).await?;

    println!("✅ Connected successfully!");

    // Check if streaming mode
    if args.stream {
        // Streaming mode
        let program = if args.program.is_empty() {
            // Default to Token Program for testing
            "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string()
        } else {
            args.program.clone()
        };

        return test_streaming(client, program, rpc_url, include_cu_logs).await;
    }

    // Unary mode (existing functionality)
    println!("\n📡 Fetching logs for transaction: {}", tx_sig);
    println!("🌐 Using RPC: {}", rpc_url);
    
    if !filter.is_empty() {
        println!("🔍 Filter: {}", filter);
    }
    
    if include_cu_logs {
        println!("⚡ Including compute unit logs");
    }

    // Create the request
    let request = tonic::Request::new(GetTxRequest {
        rpc_url,
        tx_sig,
        include_cu_logs,
        filter,
    });

    // Make the RPC call
    println!("\n⏳ Requesting transaction logs...\n");
    let response = client.get_tx_logs(request).await?;

    let tx_response = response.into_inner();

    // Display compute unit logs if included
    if !tx_response.compute_units.is_empty() {
        println!("⚡ Compute Unit Logs:");
        println!("{}", "=".repeat(80));
        for cu_log in tx_response.compute_units.iter() {
            println!("Program ID: {}", cu_log.program_id);
            println!("  Consumed: {} compute units", cu_log.consumed);
        }
    }

    // Display the program instruction logs
    println!("\n📋 Program Instruction Logs:");
    println!("{}", "=".repeat(80));
    
    if tx_response.logs.is_empty() {
        println!("No logs found (or all filtered out)");
    } else {
        for (idx, log) in tx_response.logs.iter().enumerate() {
            println!("[{}] {}", idx + 1, log);
        }
    }

    // Display raw transaction logs (optional - controlled by show_raw_logs flag)
    if show_raw_logs && !tx_response.raw_logs.is_empty() {
        println!("\n📜 Raw Transaction Logs:");
        println!("{}", "=".repeat(80));
        for (idx, log) in tx_response.raw_logs.iter().enumerate() {
            println!("[{}] {}", idx + 1, log);
        }
    }

    // Display anchor events (currently empty)
    if !tx_response.anchor_events.is_empty() {
        println!("\n🎯 Anchor Events:");
        println!("{}", "=".repeat(80));
        for event in tx_response.anchor_events.iter() {
            println!("Event: {}", event.name);
            println!("  Data: {}", event.data);
        }
    }

    println!("\n✅ Done!");

    Ok(())
}

/// Test streaming mode
async fn test_streaming(
    mut client: SolanaTxLogClient<tonic::transport::Channel>,
    program_address: String,
    rpc_url: String,
    include_cu_logs: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🌊 STREAMING MODE");
    println!("📡 Program: {}", program_address);
    println!("🌐 RPC: {}", rpc_url);
    println!("\n⏳ Subscribing to real-time logs...\n");

    let request = tonic::Request::new(StreamProgramRequest {
        rpc_url,
        program_address: program_address.clone(),
        include_cu_logs,
    });

    let mut stream = client.stream_program_logs(request).await?.into_inner();

    println!("✅ Subscribed! Waiting for transactions...\n");
    println!("{}", "=".repeat(80));

    let mut count = 0;
    while let Some(log_msg) = stream.message().await? {
        count += 1;
        println!("[{}] {}", count, log_msg.log_line);
        
        if include_cu_logs && log_msg.consumed > 0 {
            println!("    ⚡ Consumed: {} CU", log_msg.consumed);
        }
    }

    println!("\n🛑 Stream ended");
    Ok(())
}
