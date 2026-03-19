use anyhow::Result;
use clap::Parser;
use driver_core::{DriverConfig, MultiDeviceDriver};
use driver_modbus::ModbusDriver;
use tokio::signal;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, reload};

#[derive(Parser, Debug)]
#[command(name = "modbus-driver")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Modbus TCP/RTU driver for industrial devices", long_about = None)]
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
        .add_directive(format!("modbus_driver={}", args.loglevel).parse().unwrap())
        .add_directive(format!("driver_modbus={}", args.loglevel).parse().unwrap())
        .add_directive(format!("driver_core={}", args.loglevel).parse().unwrap());
    
    let (filter, reload_handle) = reload::Layer::new(filter);
    
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    tracing::info!("Loading configuration from {}", args.configfile);
    let config = DriverConfig::from_file(&args.configfile)
        .unwrap_or_else(|_| {
            tracing::warn!("Failed to load config file, trying environment variables");
            DriverConfig::from_env().expect("Failed to load configuration")
        });

    tracing::info!("Starting {} driver v{}", 
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let mut driver = MultiDeviceDriver::<ModbusDriver>::new(config.clone());
    
    tracing::info!("Driver metadata: {:?}", driver.device_manager().get_all_devices());

    driver.initialize(config).await?;

    tracing::info!("Driver initialized, starting polling loop");

    let shutdown = signal::ctrl_c();
    
    tokio::pin!(shutdown);

    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigusr1 = signal(SignalKind::user_defined1()).expect("Failed to setup SIGUSR1 handler");
        let mut sigusr2 = signal(SignalKind::user_defined2()).expect("Failed to setup SIGUSR2 handler");
        
        loop {
            tokio::select! {
                _ = &mut shutdown => {
                    tracing::info!("Shutdown signal received");
                    break;
                }
                _ = sigusr1.recv() => {
                    let new_level = "debug";
                    tracing::info!("Received SIGUSR1, changing log level to {}", new_level);
                    if let Err(e) = reload_handle.modify(|filter| {
                        *filter = EnvFilter::new(format!("modbus_driver={},driver_modbus={},driver_core={}", 
                            new_level, new_level, new_level))
                    }) {
                        tracing::error!("Failed to change log level: {}", e);
                    }
                }
                _ = sigusr2.recv() => {
                    let new_level = "info";
                    tracing::info!("Received SIGUSR2, changing log level to {}", new_level);
                    if let Err(e) = reload_handle.modify(|filter| {
                        *filter = EnvFilter::new(format!("modbus_driver={},driver_modbus={},driver_core={}", 
                            new_level, new_level, new_level))
                    }) {
                        tracing::error!("Failed to change log level: {}", e);
                    }
                }
                result = driver.run_polling_loop() => {
                    if let Err(e) = result {
                        tracing::error!("Polling loop error: {}", e);
                    }
                    break;
                }
            }
        }
    }
    
    #[cfg(not(unix))]
    {
        loop {
            tokio::select! {
                _ = &mut shutdown => {
                    tracing::info!("Shutdown signal received");
                    break;
                }
                result = driver.run_polling_loop() => {
                    if let Err(e) = result {
                        tracing::error!("Polling loop error: {}", e);
                    }
                    break;
                }
            }
        }
    }

    driver.device_manager_mut().stop_all().await?;
    tracing::info!("Driver stopped");

    Ok(())
}
