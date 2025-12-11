use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use log::{error, info};
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::fs::File;
use std::sync::{Arc, Mutex};

pub struct AudioEngine {
    _stream: cpal::Stream,
    synthesizer: Arc<Mutex<Synthesizer>>,
}

impl AudioEngine {
    pub fn init() -> Result<Self> {
        info!("Initializing Audio Engine...");

        // 1. Load SoundFont
        let sf2_filename = "SalamanderGrandPiano-V3+20200602.sf2";
        
        let sf2_path = {
             let std_path = std::path::Path::new("assets").join(sf2_filename);
             if std_path.exists() {
                 std_path
             } else if let Ok(exe_path) = std::env::current_exe() {
                 exe_path.parent().unwrap().join("assets").join(sf2_filename)
             } else {
                 std_path // Fallback
             }
        };

        info!("Loading SoundFont from: {:?}", sf2_path);
        let mut sf2_file = File::open(&sf2_path)
            .with_context(|| format!("Failed to open SoundFont at {:?}", sf2_path))?;
        let sound_font = Arc::new(SoundFont::new(&mut sf2_file).context("Failed to parse SoundFont")?);

        // 2. Setup CPAL
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("No output audio device found")?;
        info!("Using audio device: {}", device.name().unwrap_or_default());

        let config = device.default_output_config().context("Failed to get default output config")?;
        let sample_rate = config.sample_rate().0 as i32;
        let channels = config.channels() as usize;

        info!("Audio Config: Sample Rate: {}, Channels: {}", sample_rate, channels);

        // 3. Initialize Synthesizer
        let settings = SynthesizerSettings::new(sample_rate);
        let synthesizer = Arc::new(Mutex::new(Synthesizer::new(&sound_font, &settings).context("Failed to create Synthesizer")?));

        // 4. Create Audio Stream
        let synth_clone = synthesizer.clone();
        let err_fn = |err| error!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    render_audio(data, channels, &synth_clone);
                },
                err_fn,
                None,
            )?,
            _ => anyhow::bail!("Unsupported sample format (only F32 supported for now)"),
        };

        stream.play().context("Failed to start audio stream")?;

        // Startup Jingle: Playful melodic phrase with dynamics
        let jingle_synth = synthesizer.clone();
        std::thread::spawn(move || {
            // Notes: A little "question-answer" motif
            // G-A-B-D (up) -> C-B-A-G (resolve down) but only 7 notes total
            // G4-B4-D5-G5 (leap up) -> F5-E5-D5 (step down to resolve)
            let notes_and_velocities = [
                (67, 70),   // G4 - soft start
                (71, 80),   // B4 - building
                (74, 90),   // D5 - peak approach
                (79, 100),  // G5 - peak! (loudest)
                (77, 85),   // F5 - start descent
                (76, 75),   // E5 - softer
                (74, 65),   // D5 - gentle landing
            ];
            let note_duration = std::time::Duration::from_millis(100);

            for (note, velocity) in notes_and_velocities {
                if let Ok(mut synth) = jingle_synth.lock() {
                    synth.note_on(0, note, velocity);
                }
                std::thread::sleep(note_duration);
                if let Ok(mut synth) = jingle_synth.lock() {
                    synth.note_off(0, note);
                }
            }
            info!("Startup jingle played!");
        });

        Ok(AudioEngine {
            _stream: stream,
            synthesizer,
        })
    }

    pub fn get_synthesizer(&self) -> Arc<Mutex<Synthesizer>> {
        self.synthesizer.clone()
    }
}

fn render_audio(output: &mut [f32], channels: usize, synthesizer: &Arc<Mutex<Synthesizer>>) {
    let mut synth = synthesizer.lock().unwrap();
    
    // rustysynth renders stereo (left, right)
    // We need to interleave it into the output buffer
    let frame_count = output.len() / channels;
    
    // Create a temporary buffer for the synthesizer to render into
    // rustysynth expects separate left and right buffers
    let mut left = vec![0.0; frame_count];
    let mut right = vec![0.0; frame_count];

    synth.render(&mut left, &mut right);

    for (i, frame) in output.chunks_mut(channels).enumerate() {
        if channels >= 2 {
            frame[0] = left[i];
            frame[1] = right[i];
        } else {
            // Mono fallback: mix down
            frame[0] = (left[i] + right[i]) * 0.5;
        }
    }
}
