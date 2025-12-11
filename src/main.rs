pub mod audio;
pub mod midi;
pub mod ui;

use anyhow::Result;
use iced::{Application, Settings};
use log::info;
use std::sync::Arc;
use ui::ToyPianoApp;

fn main() -> Result<()> {
    env_logger::init();
    info!("Toy Piano starting up...");

    // Initialize Audio Engine first
    // We wrap it in Arc to share it with the UI (and keep it alive)
    let audio_engine = Arc::new(audio::AudioEngine::init()?);
    info!("Audio Engine initialized.");

    // Launch GUI
    // The UI handles Midi connection dynamically
    ToyPianoApp::run(Settings::with_flags(audio_engine))?;

    Ok(())
}
