use std::fs::File;
use std::io::{BufReader};
use rodio::{Decoder, OutputStream, Sink};

pub struct AudioProcessor {
    pub waveform: Arc<Mutex<Vec<f64>>>,
    pub fft_result: Arc<Mutex<Vec<f64>>>,
    pub dominant_frequency: Arc<Mutex<f64>>,
    stream: Option<cpal::Stream>,
}

impl AudioProcessor {
    pub fn new() -> Self {
        Self {
            waveform: Arc::new(Mutex::new(vec![0.0; 256])),
            fft_result: Arc::new(Mutex::new(vec![0.0; 128])),
            dominant_frequency: Arc::new(Mutex::new(0.0)),
            stream: None,
        }
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
        
        (max_index as f64) * (44100.0 / 256.0)
    }

    pub fn start_listening(&mut self) {
        let waveform_clone = Arc::clone(&self.waveform);
        let fft_result_clone = Arc::clone(&self.fft_result);
        let dominant_frequency_clone = Arc::clone(&self.dominant_frequency);

        let host = cpal::default_host();
        let device = host.default_input_device().expect("No input device found");
        let config = device.default_input_config().unwrap().into();

        let stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut waveform_data = waveform_clone.lock().unwrap();
                waveform_data.clear();
                waveform_data.extend(data.iter().map(|&s| s as f64));

                let mut fft_data = fft_result_clone.lock().unwrap();
                *fft_data = AudioProcessor::compute_fft(&waveform_data);

                let mut dominant_freq = dominant_frequency_clone.lock().unwrap();
                *dominant_freq = AudioProcessor::find_dominant_frequency(&fft_data);
            },
            |err| eprintln!("Stream error: {:?}", err),
            None,
        ).unwrap();

        stream.play().unwrap();
        self.stream = Some(stream);
    }

    pub fn stop_listening(&mut self) {
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }

    pub fn play_recorded_audio(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let file = BufReader::new(File::open("recorded_audio.wav").unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}
