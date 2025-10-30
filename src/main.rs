use crate::oscillator::SinOscillator;
use std::error::Error;

mod oscillator;
mod wav;

const SAMPLE_RATE: u32 = 44100;
const DURATION: u32 = 3; // 3 sec

fn main() -> Result<(), Box<dyn Error>> {
    let sin_440 = SinOscillator {
        frequency: 440.0,
        amplitude: 1.0,
        sample_rate: SAMPLE_RATE,
    };

    let sample_count: u32 = DURATION * SAMPLE_RATE;

    let mut pcm_samples: Vec<i16> = Vec::new();

    for i in 0..sample_count {
        pcm_samples.push(sin_440.pcm_sample(i));
    }

    wav::write("output/first_440.wav", &pcm_samples, SAMPLE_RATE)?;

    Ok(())
}
