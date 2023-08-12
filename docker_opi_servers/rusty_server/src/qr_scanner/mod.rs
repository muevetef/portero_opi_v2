use crossbeam_channel::{Receiver, Sender};
use tracing::info;

use crate::{Frame, QR};

pub async fn run(frame_rx: Receiver<Frame>, qr_sx: Sender<QR>) {
    info!("QR scanner started")
}