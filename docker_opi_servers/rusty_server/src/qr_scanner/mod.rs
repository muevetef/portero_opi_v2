use std::sync::Arc;

use chrono::Utc;
use tokio::sync::broadcast;
use tracing::{error, info};

use opencv::{
    imgcodecs, objdetect,
    prelude::*,
    types::VectorOfPoint
};

use crate::{Frame, QR, Point};

pub async fn run(mut frame_rx: broadcast::Receiver<Arc<Frame>>, qr_tx: broadcast::Sender<QR>) {
    info!("QR scanner started");

    let mut last_decode = Utc::now();

    const DECODE_INTERVL_MS: i64 = 250;

    loop {
        let frame = match frame_rx.recv().await {
            Ok(frame) => frame,
            Err(err) => {
                error!("Error while receiving frame: {err}");
                panic!()
            }
        };

        if (Utc::now() - last_decode).num_milliseconds() < DECODE_INTERVL_MS {
            continue;
        }

        last_decode = Utc::now();

        let qr = match decode_qr(&frame) {
            Ok(qr) => qr,
            Err(err) => {
                error!("Error decoding qr: {err}");
                continue;
            }
        };

        if let Some(qr) = qr {
            info!("Detected QR: {}", qr.code);
            let _ = qr_tx.send(qr);
        }
    }
}

fn decode_qr(frame: &Arc<Frame>) -> anyhow::Result<Option<QR>> {
    let frame_mat = Mat::from_slice::<u8>(&frame.data)?;
    let frame_data = imgcodecs::imdecode(&frame_mat, imgcodecs::IMREAD_COLOR)?;

    let detector = objdetect::QRCodeDetector::default()?;
    let mut points = VectorOfPoint::new();
    let mut straight = Mat::default();
    let res = detector.detect_and_decode(&frame_data, &mut points, &mut straight)?;

    if points.len() < 4 {
        return Ok(None);
    }
    
    Ok(Some(QR { 
        code: String::from_utf8_lossy(&res).into_owned(), 
        timestamp: frame.timestamp,
        points: points.iter().map(|v| {
            Point {
                x: v.x,
                y: v.y
            }
        }).collect(),
        frame_size: match frame_data.size() {
            Ok(size) => Point {
                x: size.width,
                y: size.height
            },
            Err(_) => Point { x: 0, y: 0 },
        }
    }))
}
