use anyhow::Result;
use clap::Parser;
use common::DeviceManager;
use device_meter::MeterDevice;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser, Debug)]
#[command(name = "meter-device")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Electricity meter device with thing model via Modbus", long_about = None)]
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
        .add_directive(format!("device_meter={}", args.loglevel).parse().unwrap())
        .add_directive(format!("common={}", args.loglevel).parse().unwrap());
    
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    tracing::info!("Loading multiple devices from group config: {}", args.configfile);
    
    let mut manager = DeviceManager::new();
    manager.register_service("test_write_property", MeterDevice::test_write_property);
    manager.register_service("set_threshold", MeterDevice::set_threshold);
    manager.load_from_file(&args.configfile).await?;
    manager.initialize_all().await?;
    
    tracing::info!("Initialized {} devices total", manager.len());
    
    manager.send_driver_config().await?;
    tracing::info!("Sent all driver configurations to drivers");
    
    let default_report_interval = 5000;
    manager.start_reporting_loop(default_report_interval).await;
    
    Ok(())
}
