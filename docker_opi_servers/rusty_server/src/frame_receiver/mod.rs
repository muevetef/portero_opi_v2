use crossbeam_channel::Sender;
use tracing::info;

use crate::Frame;

pub async fn run(frame_sx: Sender<Frame>) {
    info!("Frame receiver started")
}