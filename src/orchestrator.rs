use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Note {
    id: u8,
    octave: u8,
    beats: f64,
}

impl Note {
    pub fn frequency(&self) -> f64 {
        let multiplier =
            (2_f64).powf(((self.id as f64 - 9.0) + 12.0 * (self.octave as f64 - 4.0)) / 12.0);
        440.0 * multiplier
    }
}

#[derive(Debug, Deserialize)]
pub struct Orchestrator {
    bpm: u8, //beats per min
    notes: Vec<Note>,
}

// impl Orchestrator {
//     pub fn pcm_samples(&self) -> Vec<i16> {}
// }
