use clap::Parser;

// Include the generated protobuf code
pub mod proto {
    tonic::include_proto!("odin");
}

use proto::solana_tx_log_client::SolanaTxLogClient;
use proto::GetTxRequest;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // ========================================
    // PROGRAMMATIC MODE - Hardcode your values here!
    // ========================================
    let (tx_sig, rpc_url, filter, include_cu_logs) = if args.programmatic || args.tx_sig.is_empty() {
        println!("🔧 Using PROGRAMMATIC mode (hardcoded values)\n");
        
        // 👇 EDIT THESE VALUES TO TEST DIFFERENT TRANSACTIONS
        let tx_signature = "5mEjzNZjbrFmwyAWUMZemyASaheGj4MFWo2rG8DsD98m2ukKtx8JXkERhJ6GCFPc7s4D2zh36d8XrNBEsquagKkY".to_string();
        let rpc = "https://api.mainnet-beta.solana.com".to_string();
        let log_filter = "".to_string(); // Empty string = no filter
        let cu_logs = true; // true = include compute unit logs
        
        (tx_signature, rpc, log_filter, cu_logs)
    } else {
        println!("🔧 Using CLI mode (command-line arguments)\n");
        (args.tx_sig.clone(), args.rpc_url.clone(), args.filter.clone(), args.include_cu_logs)
    };

    println!("🔌 Connecting to Odin server at: {}", args.server);

    // Connect to the gRPC server
    let mut client = SolanaTxLogClient::connect(args.server).await?;

    println!("✅ Connected successfully!");
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

    // Display the logs
    println!("📋 Transaction Logs:");
    println!("{}", "=".repeat(80));
    
    if tx_response.logs.is_empty() {
        println!("No logs found (or all filtered out)");
    } else {
        for (idx, log) in tx_response.logs.iter().enumerate() {
            println!("[{}] {}", idx + 1, log);
        }
    }

    // Display compute unit logs if included
    if !tx_response.compute_units.is_empty() {
        println!("\n⚡ Compute Unit Logs:");
        println!("{}", "=".repeat(80));
        for cu_log in tx_response.compute_units.iter() {
            println!("Program ID: {}", cu_log.program_id);
            println!("  Consumed: {} compute units", cu_log.consumed);
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
