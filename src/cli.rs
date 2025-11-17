use crate::orchestrator::{Note, Orchestrator};
use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "orchestrator-rust")]
#[command(about = "A WAV file generator from JSON music notation", long_about = None)]
pub struct Args {
    /// Path to input JSON file
    #[arg(value_name = "INPUT_FILE")]
    pub input_file: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct JSONInput {
    bpm: u8, //beats per min
    notes: Vec<Note>,
    control_points: Option<Vec<f64>>,
}

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn get_filename(filepath: &Path) -> Result<String> {
    let filename = filepath
        .file_stem()
        .context("Failed to extract filename from path")?
        .to_str()
        .context("Filename contains invalid UTF-8 characters")?;
    Ok(filename.to_string())
}

pub fn get_music_input(filepath: &Path) -> Result<Orchestrator> {
    let input_data = std::fs::read_to_string(filepath)
        .context(format!("Failed to read input file: {}", filepath.display()))?;

    let orchestrator_input: JSONInput =
        serde_json::from_str(&input_data).context("Failed to parse JSON input file")?;

    Orchestrator::new(
        orchestrator_input.bpm,
        orchestrator_input.notes,
        orchestrator_input.control_points,
    )
    .map_err(|e| anyhow::anyhow!(e))
    .context("Failed to create orchestrator from input")
}
