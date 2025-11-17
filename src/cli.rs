use crate::feedback;
use crate::orchestrator::{Note, Orchestrator};
use crate::timeline_orchestrator::{TimelineNote, TimelineOrchestrator};
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

#[derive(Debug, Deserialize)]
pub struct TimelineJSONInput {
    bpm: u8, //beats per min
    notes: Vec<TimelineNote>,
    control_points: Option<Vec<f64>>,
}

/// Enum to represent either orchestrator type
pub enum AnyOrchestrator {
    Regular(Orchestrator),
    Timeline(TimelineOrchestrator),
}

impl AnyOrchestrator {
    pub fn pcm_samples(
        &self,
        sample_rate: u32,
    ) -> Result<Vec<i16>, crate::errors::OrchestratorError> {
        match self {
            AnyOrchestrator::Regular(orch) => orch.pcm_samples(sample_rate),
            AnyOrchestrator::Timeline(orch) => orch.pcm_samples(sample_rate),
        }
    }

    pub fn is_bezier(&self) -> bool {
        match self {
            AnyOrchestrator::Regular(orch) => orch.is_bezier(),
            AnyOrchestrator::Timeline(orch) => orch.is_bezier(),
        }
    }

    pub fn note_count(&self) -> usize {
        match self {
            AnyOrchestrator::Regular(orch) => orch.note_count(),
            AnyOrchestrator::Timeline(orch) => orch.note_count(),
        }
    }
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

pub fn get_music_input(filepath: &Path) -> Result<AnyOrchestrator> {
    let input_data = std::fs::read_to_string(filepath)
        .context(format!("Failed to read input file: {}", filepath.display()))?;

    // First, parse as a generic JSON value to check the timeline field
    let json_value: serde_json::Value =
        serde_json::from_str(&input_data).context("Failed to parse JSON input file")?;

    // Check if timeline field exists and is true
    let is_timeline = json_value
        .get("timeline")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if is_timeline {
        feedback::info("Using timeline orchestrator");
        // Parse as timeline input
        let timeline_input: TimelineJSONInput = serde_json::from_value(json_value)
            .context("Failed to parse timeline JSON input - ensure notes have 'start_time' and 'duration' fields")?;

        let orchestrator = TimelineOrchestrator::new(
            timeline_input.bpm,
            timeline_input.notes,
            timeline_input.control_points,
        )
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to create timeline orchestrator from input")?;

        Ok(AnyOrchestrator::Timeline(orchestrator))
    } else {
        // Parse as regular input
        let orchestrator_input: JSONInput = serde_json::from_value(json_value)
            .context("Failed to parse JSON input - ensure notes have 'beats' field")?;

        let orchestrator = Orchestrator::new(
            orchestrator_input.bpm,
            orchestrator_input.notes,
            orchestrator_input.control_points,
        )
        .map_err(|e| anyhow::anyhow!(e))
        .context("Failed to create orchestrator from input")?;

        Ok(AnyOrchestrator::Regular(orchestrator))
    }
}
