use anyhow::Result;
use clap::Parser;
use common::{DeviceConfig, DeviceManager};
use device_meter::MeterDevice;
use driver_core::driver::Driver;
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

    #[arg(long, help = "Load multiple devices from group config", default_value_t = false)]
    group: bool,
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

    if args.group {
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
    } else {
        tracing::info!("Loading configuration from {}", args.configfile);
        let config = DeviceConfig::from_file(&args.configfile)
            .unwrap_or_else(|e| {
                tracing::error!("Failed to load config file: {}", e);
                tracing::warn!("Trying environment variables");
                DeviceConfig::from_env().expect("Failed to load configuration")
            });

        let mut device = MeterDevice::new();
        device.initialize_with_device_config(config).await?;
        
        tracing::info!("Starting {} driver v{}", device.metadata().name, device.metadata().version);
        tracing::info!("Device: {}", device.device_name().unwrap_or("unknown"));

        if let Some(runtime) = device.get_runtime() {
            let model = runtime.get_thing_model();
            tracing::info!("Thing model: {} v{} with {} properties, {} services, {} events", 
                model.model_id, model.model_version, 
                model.properties.len(), 
                model.services.len(), 
                model.events.len()
            );
        }

        device.connect().await?;

        let poll_interval = device.poll_interval_ms();
        device.start_processing(poll_interval).await?;

        let mut ticker = tokio::time::interval(std::time::Duration::from_millis(poll_interval));

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    tracing::info!("Shutdown signal received");
                    device.disconnect().await?;
                    tracing::info!("Device stopped");
                    break;
                }
                _ = ticker.tick() => {
                    match device.poll_and_process().await {
                        Ok(()) => {
                            tracing::debug!("Poll and processing completed");
                        }
                        Err(e) => {
                            tracing::error!("Failed to poll and process: {}", e);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
}
