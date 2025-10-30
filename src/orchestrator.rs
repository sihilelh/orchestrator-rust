use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Note {
    id: u8,
    octave: u8,
    semitones: u8,
}

impl Note {
    pub fn frequency(&self) -> f64 {
        let multiplier =
            (2_f64).powf(((self.id as f64 - 9.0) + 12.0 * (self.octave as f64 - 4.0)) / 12.0);
        440.0 * multiplier
    }
}

pub struct Orchestrator {
    bpm: u8, //beats per min
    notes: Vec<Note>,
}
