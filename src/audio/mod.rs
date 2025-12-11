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
        let sf2_path = "assets/SalamanderGrandPiano-V3+20200602.sf2";
        info!("Loading SoundFont from: {}", sf2_path);
        let mut sf2_file = File::open(sf2_path)
            .with_context(|| format!("Failed to open SoundFont at {}", sf2_path))?;
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

        // TEST: Play Middle C (MIDI 60) for verification
        {
            let mut synth = synthesizer.lock().unwrap();
            synth.note_on(0, 60, 100); // Channel 0, Note 60, Velocity 100
            info!("Test Note ON: Middle C");
        }

        Ok(AudioEngine {
            _stream: stream,
            synthesizer,
        })
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
