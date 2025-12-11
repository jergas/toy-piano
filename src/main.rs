pub mod audio;
pub mod midi;
pub mod ui;

use anyhow::{Context, Result};
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
    let mut settings = Settings::with_flags(audio_engine);
    settings.window.size = iced::Size::new(800.0, 600.0); // Set a reasonable default size
    
    // Attempt to load icon
    match load_icon() {
        Ok(icon) => settings.window.icon = Some(icon),
        Err(e) => log::warn!("Failed to load icon: {}", e),
    }

    ToyPianoApp::run(settings)?;

    Ok(())
}

fn load_icon() -> Result<iced::window::icon::Icon> {
    let icon_path = get_asset_path("abstract-soundwave-icon.png")?;
    let img = image::open(&icon_path).context(format!("Failed to open icon file at {:?}", icon_path))?.to_rgba8();
    let (width, height) = img.dimensions();
    let rgba = img.into_raw();
    
    let icon = iced::window::icon::from_rgba(rgba, width, height).context("Failed to process icon")?;
    Ok(icon)
}

/// Helper to find assets whether running via cargo or as a bundle
pub fn get_asset_path(filename: &str) -> Result<std::path::PathBuf> {
    let std_path = std::path::Path::new("assets").join(filename);
    
    // 1. Check local assets (cargo run)
    if std_path.exists() {
        return Ok(std_path);
    }

    // 2. Check next to executable (compiled / bundle)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let bundle_path = exe_dir.join("assets").join(filename);
            if bundle_path.exists() {
                return Ok(bundle_path);
            }
            
            // 3. MacOS Bundle specific: ../Resources/assets (optional, but standard structure sometimes calls for this)
            // But my script puts them in MacOS/, so step 2 should cover it.
        }
    }

    anyhow::bail!("Asset not found: {}", filename)
}
