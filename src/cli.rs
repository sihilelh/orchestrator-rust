use crate::orchestrator::Note;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct MusicInputJson {
    bpm: u8, //beats per min
    notes: Vec<Note>,
}

pub fn get_music_input() -> Result<MusicInputJson, Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let input_file = args.get(1).unwrap();
    let input_data = std::fs::read_to_string(input_file)?;
    let input_json: MusicInputJson = serde_json::from_str(&input_data)?;
    Ok(input_json)
}
