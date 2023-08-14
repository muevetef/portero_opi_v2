use std::sync::Arc;

use chrono::Utc;
use tokio::sync::broadcast;
use tracing::{error, info};

use crate::utils::{Frame, QR, Point};

pub async fn run(mut frame_rx: broadcast::Receiver<Arc<Frame>>, qr_tx: broadcast::Sender<QR>) {
    info!("QR scanner started");

    let mut last_decode = Utc::now();

    const DECODE_INTERVL_MS: i64 = 1000;

    loop {
        let frame = match frame_rx.recv().await {
            Ok(frame) => frame,
            Err(err) => {
                error!("Error while receiving frame: {err}");
                continue;
            }
        };

        if (Utc::now() - last_decode).num_milliseconds() < DECODE_INTERVL_MS {
            continue;
        }
        let qr_tx = qr_tx.clone();
        last_decode = Utc::now();
        tokio::spawn(async move {
            let qr = match decode_qr(&frame) {
                Ok(qr) => qr,
                Err(err) => {
                    error!("Error decoding qr: {err}");
                    return;
                }
            };
    
            if let Some(qr) = qr {
                info!("Detected QR: {}", qr.code);
                let _ = qr_tx.send(qr);
            }
        });
    }
}
/*
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
 */
fn decode_qr(frame: &Arc<Frame>) -> anyhow::Result<Option<QR>> {
    use zbar_rust::ZBarImageScanner;

    use image::GenericImageView;
    
    let img = image::load_from_memory(&frame.data)?;
    let (width, height) = img.dimensions();
    
    let mut scanner = ZBarImageScanner::new();
    
    let results = scanner.scan_y800(img.into_luma8().into_raw(), width, height)
    .map_err(|_| anyhow::Error::msg("Unknown error while decoding QR"))?;
    

    if let Some(result) = results.first() {
        return Ok(Some(QR {
            code: String::from_utf8(result.data.clone())?,
            timestamp: frame.timestamp,
            points: result.points.iter().map(|p| Point {
                x: p.0,
                y: p.1
            }).collect(),
            frame_size: Point { x: width as i32, y: height as i32 },
        }));
    }

    Ok(None)
}
