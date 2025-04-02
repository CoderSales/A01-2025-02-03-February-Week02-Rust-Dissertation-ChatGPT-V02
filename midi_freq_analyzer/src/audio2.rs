use std::sync::{Arc, Mutex};
use rustfft::{FftPlanner, num_complex::Complex};
use midi_freq_analyzer::device_routing::{select_input_device, select_output_device};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fs::File;
use std::io::{BufReader, Write};
use rodio::{Decoder, OutputStream, Sink};
use std::fmt::Debug;
use std::time::Instant;




const CHUNK_SIZE: usize = 256;
const SAMPLE_RATE: f64 = 44100.0;

pub struct AudioProcessor2 {
    pub gain: f64,
    pub waveform: Arc<Mutex<Vec<f64>>>,
    pub fft_result: Arc<Mutex<Vec<f64>>>,
    pub dominant_frequency: Arc<Mutex<f64>>,
    stream: Option<cpal::Stream>,
    recorded_audio: Arc<Mutex<Vec<f32>>>, // ✅ Store recorded audio
}

impl AudioProcessor2 {
    pub fn new() -> Self {
        let waveform = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE]));
        let fft_result = Arc::new(Mutex::new(vec![0.0; CHUNK_SIZE / 2]));
        let dominant_frequency = Arc::new(Mutex::new(0.0));
        let recorded_audio = Arc::new(Mutex::new(Vec::new()));

        Self {
            gain: 1.0,
            waveform,
            fft_result,
            dominant_frequency,
            stream: None,
            recorded_audio,
        }
    }

    pub fn start_listening(&mut self) {
        let waveform_clone = Arc::clone(&self.waveform);
        let fft_result_clone = Arc::clone(&self.fft_result);
        let dominant_frequency_clone = Arc::clone(&self.dominant_frequency);
        let recorded_audio_clone = Arc::clone(&self.recorded_audio);

        let host = cpal::default_host();
        println!("🔍 Available input devices:");


        let devices = host.input_devices().unwrap().collect::<Vec<_>>();

        println!("🔎 Input Devices:");
        for (i, d) in devices.iter().enumerate() {
            println!("{}: 🎙️ {}", i, d.name().unwrap());
        }

        // let device = devices[0].clone(); // try 0, 1, or 2
        // let device = select_input_device(&host);
        //  🎧 Got 960 samples. First: [0.013, 0.015, ...]

        let maybe_device = host.input_devices().unwrap().find(|d| {
            let name = d.name().unwrap_or_default();
            name.contains("Headset") || name.contains("Microphone") || name.contains("Array")
        });
        
        let device = match maybe_device {
            Some(d) => {
                println!("✅ Auto-selected: {}", d.name().unwrap_or_default());
                d
            },
            None => {
                println!("❓ Select input device index:");
                let devices: Vec<_> = host.input_devices().unwrap().collect();
                for (i, d) in devices.iter().enumerate() {
                    println!("{i}: 🎙️ {}", d.name().unwrap());
                }
                use std::io::{self, Write};
                print!("Enter input device number: ");
                io::stdout().flush().unwrap();
                let mut choice = String::new();
                io::stdin().read_line(&mut choice).unwrap();
                let index: usize = choice.trim().parse().expect("Invalid number");
                devices.get(index).expect("Invalid index").clone()
            }
        };
        

        println!("🔎 Input Devices:");
        for (i, device) in host.input_devices().unwrap().enumerate() {
            println!("{}: 🎙️ {}", i, device.name().unwrap());
        }
        
        println!("🔊 Output Devices:");
        for (i, device) in host.output_devices().unwrap().enumerate() {
            println!("{}: 🔈 {}", i, device.name().unwrap());
        }
        

        println!("❓ Select input device index:");
        let devices: Vec<_> = host.input_devices().unwrap().collect();
        for (i, d) in devices.iter().enumerate() {
            println!("{i}: 🎙️ {}", d.name().unwrap());
        }
        use std::io::{self, Write};
        print!("Enter input device number: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let index: usize = choice.trim().parse().expect("Invalid number");
        let device = devices.get(index).expect("Invalid index").clone();
        println!("🎤 Using: {}", device.name().unwrap());
        // let config: cpal::StreamConfig = cpal::StreamConfig {
        //     channels: 1,
        //     sample_rate: cpal::SampleRate(44100),
        //     buffer_size: cpal::BufferSize::Fixed(2048),
        // };
        let config = device.default_input_config().unwrap().into();
        
        println!("🔧 Input config: {:?}", &config);

        println!("🔍 All available input devices:");
        for (i, device) in host.input_devices().unwrap().enumerate() {
            println!("{}: 🎙️ {}", i, device.name().unwrap());
        }
        

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut waveform_data = waveform_clone.lock().unwrap();
                let mut recorded_audio = recorded_audio_clone.lock().unwrap();
                
                waveform_data.clear();
                for chunk in data.chunks(2) {
                    if let [left, _right] = chunk {
                        // println!("🎙️ incoming left = {}", left);
                        waveform_data.push(*left as f64);
                        // println!("after waveform_data.push(*left as f64);");
                        // println!("🛠 finished one input callback frame");
                    }
                }
                                                
                recorded_audio.extend(data.iter());

                let mut fft_data = fft_result_clone.lock().unwrap();
                let spectrum = AudioProcessor2::compute_fft(&waveform_data);
                let threshold = 0.01; // tune as needed
                let sustained: Vec<(usize, f64)> = spectrum
                    .iter()
                    .enumerate()
                    .filter(|(_, &mag)| mag > threshold)
                    .map(|(i, &mag)| (i, mag))
                    .collect();
                
                if let Some((i, _)) = sustained.first() {
                    let freq = *i as f64 * SAMPLE_RATE / CHUNK_SIZE as f64;
                    println!("🎵 First sustained frequency: {:.2} Hz", freq);
                }
                *fft_data = spectrum;
                

                let mut dominant_freq = dominant_frequency_clone.lock().unwrap();
                *dominant_freq = AudioProcessor2::find_dominant_frequency(&fft_data);

                static mut LAST_AUDIO_TIME: Option<Instant> = None;
                
                let now = Instant::now();
                unsafe {
                    if let Some(prev) = LAST_AUDIO_TIME {
                        println!("🔁 Audio frame Δt = {:?}", now.duration_since(prev));
                    }
                    LAST_AUDIO_TIME = Some(now);
                }
                
                use std::io::{stdout, Write};
                let amp = data.iter().fold(0.0_f32, |a, &b| a.max(b.abs()));
                print!("\r🎧 in: {:.6}  ", amp);
                stdout().flush().unwrap();
            },
            |err| eprintln!("Stream error: {:?}", err),
            None,
        ).unwrap();

        println!("🚀 Before stream.play()");
        let output_device = select_output_device(&host);
        println!("🎯 Selected output: {}", output_device.name().unwrap());
        
        stream.play().unwrap();
        println!("✅ After stream.play()");

        self.stream = Some(stream);
    }

    pub fn stop_listening(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }

    pub fn play_recorded_audio(&self) {
        use cpal::default_host;
        let host = default_host();
        let output_device = select_output_device(&host);
        let (_stream, stream_handle) = OutputStream::try_from_device(&output_device).unwrap();
        println!("🔊 Output device: {}", output_device.name().unwrap());
        let sink = Sink::try_new(&stream_handle).unwrap();
        
        let recorded_audio = self.recorded_audio.lock().unwrap();

        let path = "recorded_audio.wav";
        let mut writer = hound::WavWriter::create(path, hound::WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE as u32,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        }).unwrap();

        for &sample in recorded_audio.iter() {
            writer.write_sample((sample * i16::MAX as f32) as i16).unwrap();
        }
        writer.finalize().unwrap();

        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }

    fn compute_fft(samples: &[f64]) -> Vec<f64> {
        let len = samples.len().next_power_of_two();
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(len);
        let mut buffer: Vec<Complex<f64>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
        buffer.resize(len, Complex::new(0.0, 0.0));
        fft.process(&mut buffer);
        buffer.iter().take(len / 2).map(|c| c.norm()).collect()
    }

    fn find_dominant_frequency(fft_data: &[f64]) -> f64 {
        let max_index = fft_data.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap()
            .0;
        
        (max_index as f64) * (SAMPLE_RATE / CHUNK_SIZE as f64)
    }
}