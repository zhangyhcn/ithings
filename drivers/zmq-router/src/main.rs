use anyhow::Result;
use clap::Parser;
use std::time::Duration;
use tracing::{info, error, debug};
use zmq::{Context, XPUB, XSUB};

#[derive(Parser, Debug)]
#[command(name = "zmq-router")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "ZeroMQ Router for driver-device communication", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "tcp://*:5550", help = "Frontend address (XSUB) for publishers")]
    frontend: String,
    
    #[arg(short, long, default_value = "tcp://*:5551", help = "Backend address (XPUB) for subscribers")]
    backend: String,
    
    #[arg(short, long, default_value = "info", help = "Log level (debug, info, warn, error)")]
    loglevel: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    tracing_subscriber::fmt()
        .with_env_filter(format!("zmq_router={}", args.loglevel))
        .init();

    info!("Starting ZMQ Router v{}", env!("CARGO_PKG_VERSION"));
    info!("Frontend (XSUB): {}", args.frontend);
    info!("Backend (XPUB): {}", args.backend);

    let context = Context::new();
    
    let frontend = context.socket(XSUB)?;
    frontend.bind(&args.frontend)?;
    info!("XSUB socket bound to {}", args.frontend);
    
    let backend = context.socket(XPUB)?;
    backend.bind(&args.backend)?;
    info!("XPUB socket bound to {}", args.backend);

    info!("ZMQ Router started, proxying messages...");
    info!("Architecture:");
    info!("  - Publishers (drivers) connect to XSUB port 5550");
    info!("  - Subscribers (devices) connect to XPUB port 5551");
    info!("  - Topic format: tenantid/orgid/siteid/namespaceid/device_instanceid");

    zmq::proxy(&frontend, &backend)?;
    
    Ok(())
}
