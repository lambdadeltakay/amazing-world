use crate::message::{decode_message, MessageType};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;

#[derive(Debug)]
pub struct AmazingWorldServer {
    listener: TcpListener,
    socket: Vec<TcpStream>,
    message_handlers: HashMap<MessageType, fn()>,
}

impl AmazingWorldServer {
    pub async fn new(binding_address: IpAddr) -> Self {
        let me = Self {
            listener: TcpListener::bind((binding_address, 8182)).await.unwrap(),
            socket: Vec::new(),
            message_handlers: HashMap::new(),
        };

        me
    }

    pub fn register_message_handler(&mut self, message: MessageType, handler: fn()) {
        self.message_handlers.insert(message, handler);
    }

    pub async fn poll(&mut self) {
        let mut buf = [0; u8::MAX as usize];

        if let Ok(result) = timeout(Duration::from_secs(1), self.listener.accept()).await {
            if let Ok((stream, _)) = result {
                self.socket.push(stream);
            }
        }

        for socket in self.socket.iter_mut() {
            let n = match socket.read(&mut buf).await {
                // socket closed
                Ok(n) if n == 0 => continue,
                Ok(n) => n,
                Err(e) => {
                    continue;
                }
            };

            let message = decode_message(&buf[0..n]);
            log::info!("{:?}", message);

            // Write the data back
            if let Err(e) = socket.write_all(&buf[0..n]).await {
                continue;
            }
        }
    }
}
