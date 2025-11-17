use crate::cli::{get_filename, get_music_input, parse_args, AnyOrchestrator};
use anyhow::{Context, Result};

mod cli;
mod errors;
mod feedback;
mod orchestrator;
mod oscillator;
mod timeline_orchestrator;
mod validation;
mod wav;

const SAMPLE_RATE: u32 = 44100;

fn main() -> Result<()> {
    // Parse command-line arguments
    let args = parse_args();

    // Step 1: Load input file
    feedback::info(&format!(
        "Loading input file: {}",
        args.input_file.display()
    ));
    let orchestrator: AnyOrchestrator =
        get_music_input(&args.input_file).context("Failed to load and parse music input")?;

    // Step 2: Validate and show configuration
    feedback::success("Input validated successfully");
    let oscillator_type = if orchestrator.is_bezier() {
        "Bezier curves"
    } else {
        "sine waves"
    };
    feedback::info(&format!(
        "Generating sounds using {} ({} notes)",
        oscillator_type,
        orchestrator.note_count()
    ));

    // Step 3: Generate PCM samples
    feedback::processing("Processing notes and generating samples...");
    let pcm_samples: Vec<i16> = orchestrator
        .pcm_samples(SAMPLE_RATE)
        .context("Failed to generate PCM samples")?;
    feedback::success(&format!("Generated {} samples", pcm_samples.len()));

    // Step 4: Prepare output file
    let filename: String =
        get_filename(&args.input_file).context("Failed to extract filename from input path")?;
    let output_path = format!("output/{}.wav", filename);

    // Step 5: Write WAV file
    feedback::processing(&format!("Writing WAV file to {}...", output_path));
    wav::write(&output_path, &pcm_samples, SAMPLE_RATE).context("Failed to write WAV file")?;

    // Success!
    feedback::success(&format!("Successfully created: {}", output_path));

    Ok(())
}
