use crate::cli::{get_filename, get_filepath, get_music_input};
use std::error::Error;

mod cli;
mod orchestrator;
mod oscillator;
mod wav;

const SAMPLE_RATE: u32 = 44100;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = get_filepath()?;
    let orchestrator = get_music_input(input_file.as_str())?;
    let pcm_samples = orchestrator.pcm_samples(SAMPLE_RATE);
    let filename = get_filename(&input_file)?;
    wav::write(
        format!("output/{}.wav", filename).as_str(),
        &pcm_samples,
        SAMPLE_RATE,
    )?;
    Ok(())
}
