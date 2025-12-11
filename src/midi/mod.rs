use anyhow::{Context, Result};
use log::{info, warn};
use midir::{MidiInput, MidiInputConnection};
use rustysynth::Synthesizer;
use std::sync::{Arc, Mutex};

pub struct MidiEngine {
    _connection: Option<MidiInputConnection<()>>,
}

impl MidiEngine {
    pub fn init(synthesizer: Arc<Mutex<Synthesizer>>) -> Result<Self> {
        info!("Initializing MIDI Engine...");

        let mut midi_in = MidiInput::new("Toy Piano Input").context("Failed to create MIDI input")?;
        midi_in.ignore(midir::Ignore::None);

        let ports = midi_in.ports();
        let connection = if let Some(port) = ports.first() {
            let port_name = midi_in.port_name(port).unwrap_or_else(|_| "Unknown".to_string());
            info!("Connecting to MIDI port: {}", port_name);

            let conn = midi_in.connect(
                port,
                "toy-piano-input",
                move |_stamp, message, _| {
                    handle_midi_message(message, &synthesizer);
                },
                (),
            ).map_err(|e| anyhow::anyhow!("Failed to connect to MIDI port: {}", e))?;
            
            Some(conn)
        } else {
            warn!("No available MIDI ports found.");
            None
        };

        Ok(MidiEngine {
            _connection: connection,
        })
    }
}

pub fn handle_midi_message(message: &[u8], synthesizer: &Arc<Mutex<Synthesizer>>) {
    if message.len() < 3 {
        return;
    }

    let status = message[0] & 0xF0;
    let note = message[1] as i32;
    let velocity = message[2] as i32;

    // Use a short scope for the lock to avoid blocking the audio thread too long
    // Ideally, we would use a ring buffer here too, but for a "simple" app with direct locking,
    // rustysynth's mutex is generally fast enough if we don't do I/O.
    match status {
        0x90 => { // Note On
            if velocity > 0 {
                if let Ok(mut synth) = synthesizer.lock() {
                    synth.note_on(0, note, velocity);
                }
            } else {
                // Velocity 0 is effectively Note Off
                if let Ok(mut synth) = synthesizer.lock() {
                    synth.note_off(0, note);
                }
            }
        }
        0x80 => { // Note Off
            if let Ok(mut synth) = synthesizer.lock() {
                synth.note_off(0, note);
            }
        }
        _ => {}
    }
}
