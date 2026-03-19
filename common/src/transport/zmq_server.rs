use crate::config::DriverClientConfig;
use crate::transport::driver_comm::{DriverServer, ReadRequest, ReadResponse};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use zmq::{Context, Socket, ROUTER};

pub struct ZmqDriverServer {
    socket: Arc<Mutex<Socket>>,
    config: DriverClientConfig,
    connected: bool,
}

impl ZmqDriverServer {
    pub fn new(config: &DriverClientConfig) -> Result<Self> {
        let context = Context::new();
        let socket = context.socket(ROUTER)?;
        
        socket.bind(&config.server_address)?;
        
        tracing::info!(
            "ZMQ driver server bound to {}",
            config.server_address
        );

        Ok(Self {
            socket: Arc::new(Mutex::new(socket)),
            config: config.clone(),
            connected: true,
        })
    }
}

#[async_trait]
impl DriverServer for ZmqDriverServer {
    async fn recv_request(&self) -> Result<(Vec<u8>, ReadRequest)> {
        let socket = self.socket.lock().await;
        
        let mut identity = zmq::Message::new();
        socket.recv(&mut identity, 0)?;
        
        let mut empty = zmq::Message::new();
        socket.recv(&mut empty, 0)?;
        
        let mut request = zmq::Message::new();
        socket.recv(&mut request, 0)?;
        
        let identity_bytes: Vec<u8> = identity.iter().cloned().collect();
        let req: ReadRequest = serde_json::from_slice(&request.iter().cloned().collect::<Vec<u8>>())?;
        Ok((identity_bytes, req))
    }

    fn send_response(&self, identity: &[u8], response: &ReadResponse) -> Result<()> {
        let socket = self.socket.blocking_lock();
        
        let ident_msg = zmq::Message::from(identity);
        socket.send(ident_msg, zmq::SNDMORE)?;
        
        let empty = zmq::Message::new();
        socket.send(empty, zmq::SNDMORE)?;
        
        let json = serde_json::to_string(response)?;
        let payload = zmq::Message::from(json.into_bytes());
        socket.send(payload, 0)?;
        
        Ok(())
    }

    fn connected(&self) -> bool {
        self.connected
    }
}
