mod glob;

use flume::{Receiver, Sender};
use gdnative::{
    api::{Time, OS},
    prelude::*,
};
use std::{net::SocketAddr, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time,
};
use tokio_kcp::{KcpConfig, KcpListener, KcpNoDelayConfig};

use crate::frame::DataFrame;

#[tokio::main]
pub async fn start(
    port: u16,
    max_connection: u16,
    data_frame_tx_receiver: Receiver<DataFrame>,
    data_frame_rx_sender: Sender<DataFrame>,
) {
    let mut config = KcpConfig::default();
    config.nodelay = KcpNoDelayConfig::fastest();
    let server_addr = format!("127.0.0.1:{}", port).parse::<SocketAddr>().unwrap();
    let mut listener = KcpListener::bind(config, server_addr).await.unwrap();
    godot_print!(
        "[KCP] Kcp listener binded, port: {}, max_connection: {}",
        port,
        max_connection
    );

    loop {
        let (mut stream, peer_addr) = match listener.accept().await {
            Ok(s) => s,
            Err(err) => {
                godot_error!("accept failed, error: {}", err);
                time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        };

        godot_print!("[KCP] Kcp stream accepted {}", peer_addr);
        let conv = rand::random();
        let time = Time::godot_singleton().get_ticks_msec() as u32;
        let handeshake: Vec<u8> = DataFrame::handshake(conv, time).into();
        stream.write_all(&handeshake).await.unwrap();

        let data_frame_tx_receiver_inner = data_frame_tx_receiver.clone();
        let data_frame_rx_sender_inner = data_frame_rx_sender.clone();

        tokio::spawn(async move {
            let mut buffer = [0u8; 8192];
            let conv = conv;
            loop {
                tokio::select! {
                    Ok(data_frame) = data_frame_tx_receiver_inner.recv_async() => {
                        if data_frame.conv == conv {
                            let buffer: Vec<u8> = data_frame.into();
                            stream.write_all(&buffer).await.unwrap();
                        }
                    }

                    Ok(n) = stream.read(&mut buffer) => {
                        let data_frame: DataFrame = buffer[..n].to_owned().into();
                        data_frame_rx_sender_inner.send_async(data_frame).await.unwrap();
                    }
                }
            }
            godot_warn!("[KCP] client {} closed", peer_addr);
        });
    }
}
