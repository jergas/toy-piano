pub mod audio;
pub mod midi;
pub mod ui;

use anyhow::Result;
use log::info;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    env_logger::init();
    info!("Toy Piano starting up...");

    // Initialize Audio Engine
    // We keep the engine alive by binding it to a variable
    let _audio_engine = audio::AudioEngine::init()?;
    
    info!("Audio Engine initialized. Playing test note...");
    
    // Keep the main thread alive so the audio stream continues
    // Later this will be replaced by the UI event loop
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
