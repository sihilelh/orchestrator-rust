use crate::oscillator::{BezierOscillator, SinOscillator};
use serde::Deserialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Deserialize, Clone)]
pub struct Note {
    id: u8,
    octave: u8,
    beats: f64,
    amplitude: f64,
}

#[wasm_bindgen]
impl Note {
    #[wasm_bindgen(constructor)]
    pub fn new(id: u8, octave: u8, beats: f64, amplitude: f64) -> Note {
        if id > 11 {
            panic!("Invalid note id: {}", id);
        }
        Note {
            id,
            octave,
            beats,
            amplitude,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> u8 {
        self.id
    }

    #[wasm_bindgen(getter)]
    pub fn octave(&self) -> u8 {
        self.octave
    }

    #[wasm_bindgen(getter)]
    pub fn beats(&self) -> f64 {
        self.beats
    }

    #[wasm_bindgen(getter)]
    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

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
    pub fn new(bpm: u8, notes: Vec<Note>) -> Self {
        Self { bpm, notes }
    }

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

#[derive(Debug, Deserialize)]
pub struct BezierOrchestrator {
    bpm: u8,
    notes: Vec<Note>,
    control_points: Vec<f64>,
}

impl BezierOrchestrator {
    pub fn new(bpm: u8, notes: Vec<Note>, control_points: Vec<f64>) -> Self {
        Self {
            bpm,
            notes,
            control_points,
        }
    }

    pub fn pcm_samples(&self, sample_rate: u32) -> Vec<i16> {
        let mut samples: Vec<i16> = Vec::new();
        let seconds_per_beat = 60.0 / self.bpm as f64;

        for note in &self.notes {
            let wave = BezierOscillator::new(
                note.frequency(),
                note.amplitude,
                sample_rate,
                self.control_points.clone(),
            );
            let duration = note.beats * seconds_per_beat;
            let samples_per_note = (duration * sample_rate as f64) as u32;
            for i in 0..samples_per_note {
                samples.push(wave.pcm_sample(i));
            }
        }
        samples
    }
}
