use anyhow::Result;
use clap::Parser;
use driver_core::{DriverConfig, MultiDeviceDriver};
use driver_bacnet::BacnetDriver;
use tokio::signal;
use tracing_subscriber::{fmt, prelude::*, EnvFilter, reload};
use common::config::group::DeviceGroupConfig;
use driver_core::device_manager::DeviceInstanceConfig;

#[derive(Parser, Debug)]
#[command(name = "bacnet-driver")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "BACnet protocol driver for building automation", long_about = None)]
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
        .add_directive(format!("bacnet_driver={}", args.loglevel).parse().unwrap())
        .add_directive(format!("driver_bacnet={}", args.loglevel).parse().unwrap())
        .add_directive(format!("driver_core={}", args.loglevel).parse().unwrap());
    
    let (filter, reload_handle) = reload::Layer::new(filter);
    
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    tracing::info!("Loading configuration from {}", args.configfile);
    
    let group_config = DeviceGroupConfig::from_file(&args.configfile)
        .map_err(|e| {
            tracing::error!("Failed to load group config: {}", e);
            e
        })?;

    tracing::info!("Loaded device group: {} devices, tenant={}", 
        group_config.devices.len(), 
        group_config.tenant_id
    );

    tracing::info!("Starting {} driver v{}", 
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    let base_zmq_config = if let Some(first_device) = group_config.devices.first() {
        let zmq_cfg = &first_device.driver.zmq;
        common::config::driver::ZmqConfig {
            enabled: zmq_cfg.enabled,
            publisher_address: String::new(),
            topic: zmq_cfg.topic.clone(),
            subscriber_enabled: true,
            subscriber_address: String::new(),
            write_topic: "driver/write".to_string(),
            config_update_topic: "driver/config_update".to_string(),
            high_water_mark: None,
            router_address: zmq_cfg.router_address.clone(),
            router_sub_port: zmq_cfg.router_sub_port,
            router_pub_port: zmq_cfg.router_pub_port,
        }
    } else {
        Default::default()
    };

    let init_config = DriverConfig {
        driver_name: "bacnet-driver".to_string(),
        driver_type: "bacnet".to_string(),
        device_instance_id: "bacnet-driver-group".to_string(),
        poll_interval_ms: group_config.devices.first().map(|d| d.poll_interval_ms).unwrap_or(1000),
        zmq: base_zmq_config,
        logging: common::config::driver::LoggingConfig {
            level: group_config.devices.first().map(|d| d.driver.logging.level.clone()).unwrap_or_else(|| "info".to_string()),
            format: group_config.devices.first().map(|d| d.driver.logging.format.clone()).unwrap_or_else(|| "json".to_string()),
        },
        custom: Default::default(),
    };

    let mut driver = MultiDeviceDriver::<BacnetDriver>::new(init_config.clone());
    
    driver.initialize(init_config).await?;

    for device in &group_config.devices {
        tracing::info!("Adding device: {} ({})", device.device_name, device.device_id);
        
        let custom = device.driver.custom.clone();
        if let Some(profile) = custom.get("profile") {
            tracing::debug!("Device has profile with {} device commands", 
                profile.get("deviceCommands").map(|c| c.as_array().map(|a| a.len()).unwrap_or(0)).unwrap_or(0)
            );
        }
        
        let device_config = DeviceInstanceConfig {
            device_instance_id: device.device_id.clone(),
            device_profile: None,
            custom: custom,
            poll_interval_ms: Some(device.poll_interval_ms),
        };
        
        driver.handle_config_update(device_config).await?;
    }

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
                        *filter = EnvFilter::new(format!("bacnet_driver={},driver_bacnet={},driver_core={}", 
                            new_level, new_level, new_level))
                    }) {
                        tracing::error!("Failed to change log level: {}", e);
                    }
                }
                _ = sigusr2.recv() => {
                    let new_level = "info";
                    tracing::info!("Received SIGUSR2, changing log level to {}", new_level);
                    if let Err(e) = reload_handle.modify(|filter| {
                        *filter = EnvFilter::new(format!("bacnet_driver={},driver_bacnet={},driver_core={}", 
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
