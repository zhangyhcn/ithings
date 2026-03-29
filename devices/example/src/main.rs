use anyhow::Result;
use clap::Parser;
use common::DeviceManager;
use device_example::ExampleDevice;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(name = "example-device")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Example device driver demonstrating DeviceBuilder usage", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.json", help = "Configuration file path")]
    configfile: String,
    
    #[arg(short, long, default_value = "info", help = "Log level (debug, info, warn, error)")]
    loglevel: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    let filter = EnvFilter::from_default_env()
        .add_directive(format!("device_example={}", args.loglevel).parse().unwrap())
        .add_directive(format!("common={}", args.loglevel).parse().unwrap());
    
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    tracing::info!("Loading devices from config: {}", args.configfile);
    
    let mut manager = DeviceManager::new();
    manager.register_service("echo", ExampleDevice::echo_service);
    manager.register_service("add", ExampleDevice::add_service);
    manager.register_service("get_status", ExampleDevice::get_status_service);
    manager.load_from_file(&args.configfile).await?;
    manager.initialize_all().await?;
    
    tracing::info!("Initialized {} devices total", manager.len());
    
    manager.send_driver_config().await?;
    tracing::info!("Sent all driver configurations to drivers");
    
    let default_report_interval = 5000;
    manager.start_reporting_loop(default_report_interval).await;
    
    Ok(())
}
