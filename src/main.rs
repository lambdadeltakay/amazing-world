mod context;
mod message;

use std::io::Write;

use crate::context::AmazingWorldServer;
use crate::message::decode_message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::runtime::{Handle, Runtime};

#[tokio::main]
async fn main() {
    env_logger::init();

    let mut server = AmazingWorldServer::new("0.0.0.0".parse().unwrap()).await;

    loop {
        server.poll().await;
    }
}
