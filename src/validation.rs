use crate::errors::OrchestratorError;
use crate::orchestrator::Note;
use crate::timeline_orchestrator::TimelineNote;

/// Validates BPM is within reasonable range (1-240)
pub fn validate_bpm(bpm: u8) -> Result<(), OrchestratorError> {
    if bpm == 0 {
        return Err(OrchestratorError::InvalidBpm(bpm));
    }
    Ok(())
}

/// Validates that notes array is not empty and each note is valid
pub fn validate_notes(notes: &[Note]) -> Result<(), OrchestratorError> {
    if notes.is_empty() {
        return Err(OrchestratorError::EmptyNotes);
    }

    for note in notes.iter() {
        validate_note(note)?;
    }

    Ok(())
}

/// Validates a single note's properties
pub fn validate_note(note: &Note) -> Result<(), OrchestratorError> {
    // Validate note ID (0-11 for 12 chromatic notes)
    if note.id() > 11 {
        return Err(OrchestratorError::InvalidNoteId(note.id()));
    }

    // Validate octave (0-8 is standard piano range)
    if note.octave() > 8 {
        return Err(OrchestratorError::InvalidOctave(note.octave()));
    }

    // Validate amplitude (0.0 to 1.0)
    if note.amplitude() < 0.0 || note.amplitude() > 1.0 {
        return Err(OrchestratorError::InvalidAmplitude(note.amplitude()));
    }

    Ok(())
}

/// Validates control points for Bezier oscillator
pub fn validate_control_points(points: &[f64]) -> Result<(), OrchestratorError> {
    // Must have exactly 4 control points
    if points.len() != 4 {
        return Err(OrchestratorError::InvalidControlPoints(format!(
            "Expected 4 control points, got {}",
            points.len()
        )));
    }

    // Each control point must be in range [-1.0, 1.0]
    for (index, &point) in points.iter().enumerate() {
        if point < -1.0 || point > 1.0 {
            return Err(OrchestratorError::InvalidControlPoints(format!(
                "Control point {} has value {}, must be between -1.0 and 1.0",
                index, point
            )));
        }
    }

    Ok(())
}

/// Validates that timeline notes array is not empty and each note is valid
pub fn validate_timeline_notes(notes: &[TimelineNote]) -> Result<(), OrchestratorError> {
    if notes.is_empty() {
        return Err(OrchestratorError::EmptyNotes);
    }

    for note in notes.iter() {
        validate_timeline_note(note)?;
    }

    Ok(())
}

/// Validates a single timeline note's properties
pub fn validate_timeline_note(note: &TimelineNote) -> Result<(), OrchestratorError> {
    // Validate note ID (0-11 for 12 chromatic notes)
    if note.id() > 11 {
        return Err(OrchestratorError::InvalidNoteId(note.id()));
    }

    // Validate octave (0-8 is standard piano range)
    if note.octave() > 8 {
        return Err(OrchestratorError::InvalidOctave(note.octave()));
    }

    // Validate amplitude (0.0 to 1.0)
    if note.amplitude() < 0.0 || note.amplitude() > 1.0 {
        return Err(OrchestratorError::InvalidAmplitude(note.amplitude()));
    }

    Ok(())
}
