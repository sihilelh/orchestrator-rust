use thiserror::Error;

#[derive(Debug, Error)]
pub enum OrchestratorError {
    #[error("Invalid note ID: {0}. Note ID must be between 0 and 11 (12 chromatic notes)")]
    InvalidNoteId(u8),

    #[error("Invalid BPM: {0}. BPM must be between 1 and 255")]
    InvalidBpm(u8),

    #[error("Invalid octave: {0}. Octave must be between 0 and 8")]
    InvalidOctave(u8),

    #[error("Invalid amplitude: {0}. Amplitude must be between 0.0 and 1.0")]
    InvalidAmplitude(f64),

    #[error("No notes provided. At least one note is required")]
    EmptyNotes,

    #[error("Invalid control points: {0}")]
    InvalidControlPoints(String),
}

