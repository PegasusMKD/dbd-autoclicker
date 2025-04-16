use anyhow::Result;
use async_channel::Sender;

pub enum KeyboardEvent {
    Toggle,
    Kill,
    Unmapped,
}

impl KeyboardEvent {
    pub fn from_stroke(stroke: String) -> Self {
        match stroke.to_lowercase().as_str() {
            "f6" => Self::Toggle,
            "esc" => Self::Kill,
            _ => Self::Unmapped,
        }
    }
}

pub async fn track(keyboard_tx: Sender<KeyboardEvent>) -> Result<()> {
    // TODO: Track keyboard input
    // If input is unmapped, skip
    // If input mapped, send over to mouse/processor for handling
    Ok(())
}
