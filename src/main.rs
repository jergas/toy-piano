pub mod audio;
pub mod midi;
pub mod ui;

use log::info;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("Toy Piano starting up...");
    
    println!("Hello from Toy Piano! The build works.");
    println!("Dependencies loaded: cpal, midir, rustysynth, iced.");
    
    Ok(())
}
