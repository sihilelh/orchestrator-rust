use crate::orchestrator::{Note, Orchestrator};
use std::error::Error;

pub fn get_music_input() -> Result<Orchestrator, Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let input_file = args.get(1).unwrap();
    let input_data = std::fs::read_to_string(input_file)?;
    let orchestrator: Orchestrator = serde_json::from_str(&input_data)?;
    Ok(orchestrator)
}
