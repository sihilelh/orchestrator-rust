use crate::errors::OrchestratorError;
use crate::oscillator::{BezierOscillator, SinOscillator};
use crate::validation::{validate_bpm, validate_control_points, validate_notes};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Note {
    id: u8,
    octave: u8,
    beats: f64,
    amplitude: f64,
}

impl Note {
    // Public getters for validation
    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn octave(&self) -> u8 {
        self.octave
    }

    pub fn amplitude(&self) -> f64 {
        self.amplitude
    }

    pub fn frequency(&self) -> Result<f64, OrchestratorError> {
        if self.id > 11 {
            return Err(OrchestratorError::InvalidNoteId(self.id));
        }
        let multiplier =
            (2_f64).powf(((self.id as f64 - 9.0) + 12.0 * (self.octave as f64 - 4.0)) / 12.0);
        let frequency = 440.0 * multiplier;
        Ok(frequency)
    }
}

pub enum Orchestrator {
    Sine(SineOrchestrator),
    Bezier(BezierOrchestrator),
}

impl Orchestrator {
    pub fn pcm_samples(&self, sample_rate: u32) -> Result<Vec<i16>, OrchestratorError> {
        match self {
            Orchestrator::Sine(sine) => sine.pcm_samples(sample_rate),
            Orchestrator::Bezier(bezier) => bezier.pcm_samples(sample_rate),
        }
    }

    pub fn new(
        bpm: u8,
        notes: Vec<Note>,
        control_points: Option<Vec<f64>>,
    ) -> Result<Self, OrchestratorError> {
        // Validate inputs
        validate_bpm(bpm)?;
        validate_notes(&notes)?;

        if let Some(ref points) = control_points {
            validate_control_points(points)?;
            Ok(Orchestrator::Bezier(BezierOrchestrator {
                bpm,
                notes,
                control_points: points.clone(),
            }))
        } else {
            Ok(Orchestrator::Sine(SineOrchestrator { bpm, notes }))
        }
    }

    pub fn is_bezier(&self) -> bool {
        matches!(self, Orchestrator::Bezier(_))
    }

    pub fn note_count(&self) -> usize {
        match self {
            Orchestrator::Sine(sine) => sine.notes.len(),
            Orchestrator::Bezier(bezier) => bezier.notes.len(),
        }
    }
}

pub struct SineOrchestrator {
    bpm: u8, //beats per min
    notes: Vec<Note>,
}

impl SineOrchestrator {
    pub fn pcm_samples(&self, sample_rate: u32) -> Result<Vec<i16>, OrchestratorError> {
        let mut samples: Vec<i16> = Vec::new();
        let seconds_per_beat = 60.0 / self.bpm as f64;

        for note in &self.notes {
            let wave = SinOscillator {
                amplitude: note.amplitude,
                frequency: note.frequency()?,
                sample_rate: sample_rate,
            };
            let duration = note.beats * seconds_per_beat;
            let samples_per_note = (duration * sample_rate as f64) as u32;
            for i in 0..samples_per_note {
                samples.push(wave.pcm_sample(i));
            }
        }
        Ok(samples)
    }
}

pub struct BezierOrchestrator {
    bpm: u8, //beats per min
    notes: Vec<Note>,
    control_points: Vec<f64>,
}

impl BezierOrchestrator {
    pub fn pcm_samples(&self, sample_rate: u32) -> Result<Vec<i16>, OrchestratorError> {
        let mut samples: Vec<i16> = Vec::new();
        let seconds_per_beat = 60.0 / self.bpm as f64;

        for note in &self.notes {
            let wave = BezierOscillator::new(
                note.frequency()?,
                note.amplitude,
                sample_rate,
                self.control_points.clone(),
            )?;
            let duration = note.beats * seconds_per_beat;
            let samples_per_note = (duration * sample_rate as f64) as u32;
            for i in 0..samples_per_note {
                samples.push(wave.pcm_sample(i));
            }
        }
        Ok(samples)
    }
}
