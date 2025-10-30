use crate::cli::get_music_input;
use crate::orchestrator::{Note, Orchestrator};
use std::error::Error;

mod cli;
mod orchestrator;
mod oscillator;
mod wav;

const SAMPLE_RATE: u32 = 44100;

fn main() -> Result<(), Box<dyn Error>> {
    let music_input = get_music_input()?;
    let pcm_samples = music_input.pcm_samples(SAMPLE_RATE);
    wav::write("output/simple_octave.wav", &pcm_samples, SAMPLE_RATE)?;
    Ok(())
}
