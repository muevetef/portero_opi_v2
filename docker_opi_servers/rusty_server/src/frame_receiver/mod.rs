use std::{net::SocketAddr, time::Duration, sync::Arc};

use chrono::Utc;
use tokio::{net::UdpSocket, sync::broadcast::Sender};
use tracing::{info, error};

use crate::utils::Frame;

pub async fn run(frame_sx: Sender<Arc<Frame>>) {

    let socket_addr = SocketAddr::from(([0,0,0,0], 12000));
    let mut current_frame = 0;

    let _socket = match UdpSocket::bind(&socket_addr).await {
        Ok(socket) => socket,
        Err(err) => {
            error!("Error binding frame_receiver udp socket: {err}");
            panic!("{err}");
        },
    };

    info!("Frame receiver started, listening on {socket_addr}");

    let frame_data = include_bytes!("full.jpg");
    let frame_flipped_data = include_bytes!("full_mirror.jpg");
    loop {
        tokio::time::sleep(Duration::from_millis(80)).await;

        current_frame += 1;

        let data = if (200..400).contains(&current_frame) {
            frame_flipped_data.to_vec()
        } else if current_frame < 200 {
            frame_data.to_vec()
        } else {
            current_frame = 0;
            frame_data.to_vec()
        };

        let frame = Frame {
            data,
            timestamp: Utc::now(),
        };

        match frame_sx.send(Arc::new(frame)) {
            Ok(_) => (),
            Err(err) => {
                error!("Error sending frame: {err}");
                continue;
            }
        }
    }
}