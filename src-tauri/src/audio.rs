use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use rodio::{OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct AudioState {
    pub stop_sender: Arc<Mutex<Option<std::sync::mpsc::Sender<()>>>>,
    pub recording_active: Arc<Mutex<bool>>,
}

impl AudioState {
    pub fn new() -> Self {
        Self {
            stop_sender: Arc::new(Mutex::new(None)),
            recording_active: Arc::new(Mutex::new(false)),
        }
    }
}

pub fn list_input_devices() -> Vec<String> {
    let host = cpal::default_host();
    match host.input_devices() {
        Ok(devices) => devices.filter_map(|d| d.name().ok()).collect(),
        Err(_) => vec![],
    }
}

pub fn play_sound(name: &str) {
    let name = name.to_string(); // Own it
    std::thread::spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        match name.as_str() {
            "Ping" => {
                let source = rodio::source::SineWave::new(880.0) // A5
                    .take_duration(Duration::from_millis(200))
                    .amplify(0.20);
                sink.append(source);
            }
            "Click" => {
                let source = rodio::source::SineWave::new(1200.0)
                    .take_duration(Duration::from_millis(50))
                    .amplify(0.10);
                sink.append(source);
            }
            "Swoosh" => {
                // Approximate with a lower freq
                let source = rodio::source::SineWave::new(400.0)
                    .take_duration(Duration::from_millis(300))
                    .amplify(0.20);
                sink.append(source);
            }
            "Success" => {
                let s1 = rodio::source::SineWave::new(523.25)
                    .take_duration(Duration::from_millis(150))
                    .amplify(0.20);
                let s2 = rodio::source::SineWave::new(659.25)
                    .take_duration(Duration::from_millis(150))
                    .amplify(0.20);
                let s3 = rodio::source::SineWave::new(783.99)
                    .take_duration(Duration::from_millis(300))
                    .amplify(0.20);
                sink.append(s1);
                sink.append(s2);
                sink.append(s3);
            }
            _ => {}
        }

        sink.sleep_until_end();
    });
}

pub fn start_recording(
    state: &AudioState,
    device_name: Option<String>,
    output_path: PathBuf,
) -> Result<(), String> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Store sender first to ensure we can stop it
    {
        let mut sender_guard = state.stop_sender.lock().unwrap();
        *sender_guard = Some(tx);
    }
    {
        let mut recording = state.recording_active.lock().unwrap();
        *recording = true;
    }

    // Spawn thread to handle stream lifetime
    std::thread::spawn(move || {
        let host = cpal::default_host();

        // Find device
        let device = if let Some(name) = device_name {
            host.input_devices()
                .ok()
                .and_then(|mut d| d.find(|x| x.name().unwrap_or_default() == name))
        } else {
            host.default_input_device()
        };

        if let Some(device) = device {
            if let Ok(config) = device.default_input_config() {
                let spec = hound::WavSpec {
                    channels: config.channels(),
                    sample_rate: config.sample_rate().0,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int,
                };

                if let Ok(writer) = hound::WavWriter::create(output_path, spec) {
                    let writer = Arc::new(Mutex::new(Some(writer)));
                    let writer_2 = writer.clone();

                    let err_fn = move |err| {
                        eprintln!("an error occurred on stream: {}", err);
                    };

                    let stream_res = match config.sample_format() {
                        cpal::SampleFormat::F32 => device.build_input_stream(
                            &config.into(),
                            move |data: &[f32], _: &_| {
                                write_input_data::<f32, i16>(data, &writer_2)
                            },
                            err_fn,
                            None,
                        ),
                        cpal::SampleFormat::I16 => device.build_input_stream(
                            &config.into(),
                            move |data: &[i16], _: &_| {
                                write_input_data::<i16, i16>(data, &writer_2)
                            },
                            err_fn,
                            None,
                        ),
                        cpal::SampleFormat::U16 => device.build_input_stream(
                            &config.into(),
                            move |data: &[u16], _: &_| {
                                write_input_data::<u16, i16>(data, &writer_2)
                            },
                            err_fn,
                            None,
                        ),
                        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                    };

                    if let Ok(stream) = stream_res {
                        if stream.play().is_ok() {
                            // Wait for stop signal
                            let _ = rx.recv();
                            // Stream drops here, stopping recording
                        }
                    }
                }
            }
        }
    });

    Ok(())
}

pub fn stop_recording(state: &AudioState) {
    let mut sender_guard = state.stop_sender.lock().unwrap();
    if let Some(tx) = sender_guard.take() {
        let _ = tx.send(()); // Signal thread to exit
    }

    let mut recording = state.recording_active.lock().unwrap();
    *recording = false;
}

fn write_input_data<T, U>(
    input: &[T],
    writer: &Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>,
) where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}
