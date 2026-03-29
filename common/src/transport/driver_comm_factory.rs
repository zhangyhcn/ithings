use crate::config::DeviceConfig;
use crate::transport::driver_comm::{DriverServer, DriverClient, DriverCommType};
use crate::transport::zmq_server::ZmqDriverServer;
use crate::transport::zmq_client::ZmqDriverClient;
use anyhow::Result;

pub struct DriverServerFactory;

impl DriverServerFactory {
    pub fn create(config: &DeviceConfig) -> Result<Option<Box<dyn DriverServer>>> {
        if !config.driver.enabled {
            return Ok(None);
        }

        if let Some(comm_type) = config.custom.get("driver_comm_type") {
            let comm_type: DriverCommType = serde_json::from_value(comm_type.clone())?;
            
            match comm_type {
                DriverCommType::Zmq => {
                    let server = ZmqDriverServer::new(&config.driver)?;
                    Ok(Some(Box::new(server)))
                }
                DriverCommType::InMemory => {
                    Err(anyhow::anyhow!("InMemory communication not implemented yet"))
                }
                DriverCommType::TcpSocket => {
                    Err(anyhow::anyhow!("TcpSocket communication not implemented yet"))
                }
            }
        } else {
            let server = ZmqDriverServer::new(&config.driver)?;
            Ok(Some(Box::new(server)))
        }
    }
}

pub struct DriverClientFactory;

impl DriverClientFactory {
    pub fn create(config: &DeviceConfig) -> Result<Option<Box<dyn DriverClient>>> {
        if !config.driver.enabled {
            return Ok(None);
        }

        if let Some(comm_type) = config.custom.get("driver_comm_type") {
            let comm_type: DriverCommType = serde_json::from_value(comm_type.clone())?;
            
            match comm_type {
                DriverCommType::Zmq => {
                    let client = ZmqDriverClient::new(&config.driver)?;
                    Ok(Some(Box::new(client)))
                }
                DriverCommType::InMemory => {
                    Err(anyhow::anyhow!("InMemory communication not implemented yet"))
                }
                DriverCommType::TcpSocket => {
                    Err(anyhow::anyhow!("TcpSocket communication not implemented yet"))
                }
            }
        } else {
            let client = ZmqDriverClient::new(&config.driver)?;
            Ok(Some(Box::new(client)))
        }
    }
}
