use chrono::Utc;
use tokio::{net::UdpSocket, sync::broadcast::Sender};
use tracing::{error, info, debug};
use std::{sync::Arc, net::SocketAddr};

use crate::utils::Frame;

pub async fn run(frame_sx: Sender<Arc<Frame>>) {

    let socket_addr = SocketAddr::from(([0,0,0,0], 12001));

    let socket = match UdpSocket::bind(&socket_addr).await {
        Ok(socket) => socket,
        Err(err) => {
            error!("Error binding frame_receiver udp socket: {err}");
            panic!("{err}");
        },
    };

    info!("Started frame_receiver udp socket. Listening on udp://{socket_addr}");

    receive_frames(socket, frame_sx).await.unwrap();
}

async fn receive_frames(socket: UdpSocket, frame_sx: Sender<Arc<Frame>>) -> anyhow::Result<()> {
    const PACK_SIZE: usize = 4096;

    loop {
        let total_pack = {
            let mut buf = vec![0u8; u16::MAX as usize];
            loop {
                let read = socket.recv(&mut buf).await?;

                if read == std::mem::size_of::<i32>() {
                    break;
                }
            }

            i32::from_le_bytes(<[u8; 4]>::try_from(&buf[..4])?)
        };

        let frame = {
            let mut buf = Vec::new();
            buf.resize(total_pack as usize * PACK_SIZE, 0u8);

            for i in 0..total_pack as usize {
                let read = socket.recv(&mut buf[i * PACK_SIZE..]).await?;

                if read != PACK_SIZE {
                    error!("Unexpected frame size!");
                }
            }

            buf
        };

        debug!("Got frame!");

        frame_sx.send(Arc::new(Frame { data: frame, timestamp: Utc::now() }))?;
    }
}