use crate::orchestrator::Orchestrator;
use std::error::Error;

pub fn get_filepath() -> Result<String, Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let input_file = args.get(1).unwrap();
    Ok(input_file.to_string())
}

pub fn get_filename(filepath: &str) -> Result<String, Box<dyn Error>> {
    let filename = filepath.split("/").last().unwrap();
    let filename = filename.split(".").next().unwrap();
    Ok(filename.to_string())
}

pub fn get_music_input(filename: &str) -> Result<Orchestrator, Box<dyn Error>> {
    let input_data = std::fs::read_to_string(filename)?;
    let orchestrator: Orchestrator = serde_json::from_str(&input_data)?;
    Ok(orchestrator)
}
