use std::{net::SocketAddr, sync::Arc};

use chrono::Utc;
use tokio::{net::UdpSocket, sync::broadcast::Sender};
use tracing::{error, info};

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
    loop {
        let mut buf = vec![0u8; u16::MAX as usize];
        
        let frame_length = {
            let mut buf_pos = 0;

            while buf_pos < 4 {
                buf_pos += socket.recv(&mut buf[buf_pos..]).await?;
            }

            i32::from_le_bytes(<[u8; 4]>::try_from(&buf[..4])?)
        };

        let frame = {
            let mut buf_pos = 0;
            
            while buf_pos < frame_length as usize {
                buf_pos += socket.recv(&mut buf[buf_pos..]).await?;
            }

            &buf[..buf_pos]
        };

        frame_sx.send(Arc::new(Frame { data: frame.to_owned(), timestamp: Utc::now() }))?;
    }
}