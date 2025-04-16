use anyhow::Result;
use async_channel::Receiver;

use crate::mouse::MouseEvent;

pub async fn process(mouse_rx: Receiver<MouseEvent>) -> Result<()> {
    // TODO: Process a MouseEvent
    // Read the image
    // Compare it to the template
    // If it matches, then send a click
    // Put in a 0.8s to 1.2s timer for the next click event
    // If it doesn't match, just keep waiting for a match
    Ok(())
}
