use crate::oscillator::SinOscillator;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Note {
    id: u8,
    octave: u8,
    beats: f64,
    amplitude: f64,
}

impl Note {
    pub fn frequency(&self) -> f64 {
        if self.id > 11 {
            panic!("Invalid note id: {}", self.id);
        }
        let multiplier =
            (2_f64).powf(((self.id as f64 - 9.0) + 12.0 * (self.octave as f64 - 4.0)) / 12.0);
        let frequency = 440.0 * multiplier;
        frequency
    }
}

#[derive(Debug, Deserialize)]
pub struct Orchestrator {
    bpm: u8, //beats per min
    notes: Vec<Note>,
}

impl Orchestrator {
    pub fn pcm_samples(&self, sample_rate: u32) -> Vec<i16> {
        let mut samples: Vec<i16> = Vec::new();
        let seconds_per_beat = 60.0 / self.bpm as f64;

        for note in &self.notes {
            let wave = SinOscillator {
                amplitude: note.amplitude,
                frequency: note.frequency(),
                sample_rate: sample_rate,
            };
            let duration = note.beats * seconds_per_beat;
            let samples_per_note = (duration * sample_rate as f64) as u32;
            for i in 0..samples_per_note {
                samples.push(wave.pcm_sample(i));
            }
        }
        samples
    }
}
