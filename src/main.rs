use crate::cli::get_music_input;
use crate::orchestrator::{Note, Orchestrator};
use std::error::Error;

mod cli;
mod orchestrator;
mod wav;

const SAMPLE_RATE: u32 = 44100;

fn main() -> Result<(), Box<dyn Error>> {
    let music_input = get_music_input()?;
    println!("{:#?}", music_input);
    Ok(())
}
