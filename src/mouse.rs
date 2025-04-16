use anyhow::Result;
use async_channel::{Receiver, Sender};
use image::DynamicImage;

use crate::keyboard::KeyboardEvent;

pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub image: DynamicImage,
}

pub async fn track(
    keyboard_rx: Receiver<KeyboardEvent>,
    mouse_tx: Sender<MouseEvent>,
) -> Result<()> {
    // TODO: Keyboard events
    // If toggled, then stop taking screenshots
    // If kill, then stop the thread
    //
    // TODO: Mouse events
    // Find the current position of the mouse
    // Take a screenshot around the mouse
    // Store that image into the `MouseEvent::image` property
    // Send it over to the Processor for handling
    Ok(())
}
