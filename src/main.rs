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
    let audio_engine = audio::AudioEngine::init()?;
    
    // Initialize MIDI Engine
    // We pass the synthesizer (shared state) to the MIDI engine
    let _midi_engine = midi::MidiEngine::init(audio_engine.get_synthesizer())?;

    info!("Audio and MIDI initialized. Ready to play!");
    
    // Keep the main thread alive so the audio stream continues
    // Later this will be replaced by the UI event loop
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
