use std::net::SocketAddr;

use crate::frame::DataFrame;
use flume::{Receiver, Sender};
use gdnative::godot_print;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_kcp::{KcpConfig, KcpNoDelayConfig, KcpStream};

#[tokio::main]
pub async fn start(
    port: u16,
    data_frame_tx_receiver: Receiver<DataFrame>,
    data_frame_rx_sender: Sender<DataFrame>,
) {
    let mut config = KcpConfig::default();
    config.nodelay = KcpNoDelayConfig::fastest();
    let server_addr = format!("127.0.0.1:{}", port).parse::<SocketAddr>().unwrap();
    let mut stream = KcpStream::connect(&config, server_addr).await.unwrap();
    godot_print!("[KCP] Kcp stream connected");
    let mut buffer = [0u8; 65536];
    loop {
        tokio::select! {
            Ok(n) = stream.read(&mut buffer) => {
                let data_frame: DataFrame = buffer[..n].to_owned().into();
                data_frame_rx_sender.send_async(data_frame).await.unwrap();
            }
            Ok(data_frame) = data_frame_tx_receiver.recv_async() => {
                let buffer: Vec<u8> = data_frame.into();
                stream.write_all(&buffer).await.unwrap();
            }
        }
    }
}
