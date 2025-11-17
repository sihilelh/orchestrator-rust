use crate::errors::OrchestratorError;
use crate::oscillator::{BezierOscillator, SinOscillator};
use crate::validation::{validate_bpm, validate_control_points, validate_timeline_notes};
use serde::Deserialize;

// For safe mixing we will condense the amplitude
const CONDENSE_CONSTANT: f64 = 0.9;
const PCM_BIT_RANGE: f64 = 32767.0; // 2^15 - 1

#[derive(Debug, Deserialize)]
pub struct TimelineNote {
    id: u8,
    octave: u8,
    start_time: f64,
    duration: f64,
    amplitude: f64,
}

impl TimelineNote {
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

pub enum TimelineOrchestrator {
    Sine(TimelineSineOrchestrator),
    Bezier(TimelineBezierOrchestrator),
}

impl TimelineOrchestrator {
    pub fn pcm_samples(&self, sample_rate: u32) -> Result<Vec<i16>, OrchestratorError> {
        match self {
            TimelineOrchestrator::Sine(sine) => sine.pcm_samples(sample_rate),
            TimelineOrchestrator::Bezier(bezier) => bezier.pcm_samples(sample_rate),
        }
    }

    pub fn new(
        bpm: u8,
        notes: Vec<TimelineNote>,
        control_points: Option<Vec<f64>>,
    ) -> Result<Self, OrchestratorError> {
        // Validate inputs
        validate_bpm(bpm)?;
        validate_timeline_notes(&notes)?;

        if let Some(ref points) = control_points {
            validate_control_points(points)?;
            Ok(TimelineOrchestrator::Bezier(TimelineBezierOrchestrator {
                bpm,
                notes,
                control_points: points.clone(),
            }))
        } else {
            Ok(TimelineOrchestrator::Sine(TimelineSineOrchestrator {
                bpm,
                notes,
            }))
        }
    }

    pub fn is_bezier(&self) -> bool {
        matches!(self, TimelineOrchestrator::Bezier(_))
    }

    pub fn note_count(&self) -> usize {
        match self {
            TimelineOrchestrator::Sine(sine) => sine.notes.len(),
            TimelineOrchestrator::Bezier(bezier) => bezier.notes.len(),
        }
    }
}

pub struct TimelineSineOrchestrator {
    bpm: u8, //beats per min
    notes: Vec<TimelineNote>,
}

impl TimelineSineOrchestrator {
    pub fn pcm_samples(&self, sample_rate: u32) -> Result<Vec<i16>, OrchestratorError> {
        let seconds_per_beat = 60.0 / self.bpm as f64;

        let mut total_duration_in_beats: f64 = 0.0;
        for note in &self.notes {
            total_duration_in_beats = total_duration_in_beats.max(note.start_time + note.duration);
        }

        let total_duration_in_seconds = total_duration_in_beats * seconds_per_beat;
        let total_samples: usize = (total_duration_in_seconds * sample_rate as f64).ceil() as usize;

        // Create a vector with specified capacity and with default value = 0 to avoid reallocations
        // Creating it f64 because these samples are not clipped
        // This acts like the timeline
        let mut pcm_sample_sums: Vec<f64> = vec![0.0; total_samples];

        // Process each note and mix it at the same time
        for note in &self.notes {
            let wave = SinOscillator {
                frequency: note.frequency()?,
                amplitude: note.amplitude * CONDENSE_CONSTANT,
                sample_rate: sample_rate,
            };

            let start_sample = (note.start_time * seconds_per_beat * sample_rate as f64) as usize;
            let samples_for_this_note =
                (note.duration * seconds_per_beat * sample_rate as f64) as usize;

            for i in 0..samples_for_this_note {
                let current_sample_index = start_sample + i;
                if current_sample_index < total_samples {
                    let current_sample = wave.pcm_sample(i as u32);
                    pcm_sample_sums[current_sample_index] += current_sample as f64;
                }
            }
        }

        // Normalize, apply soft clipping with tanh, and convert to PCM
        let pcm_samples: Vec<i16> = pcm_sample_sums
            .iter()
            .map(|&sum| {
                // Normalize from accumulated PCM range back to [-1, 1]
                let normalized = sum / PCM_BIT_RANGE;
                // Apply soft clipping with tanh
                let clipped = normalized.tanh();
                // Convert back to PCM i16 range
                (clipped * PCM_BIT_RANGE) as i16
            })
            .collect();

        Ok(pcm_samples)
    }
}

pub struct TimelineBezierOrchestrator {
    bpm: u8, //beats per min
    notes: Vec<TimelineNote>,
    control_points: Vec<f64>,
}

impl TimelineBezierOrchestrator {
    pub fn pcm_samples(&self, sample_rate: u32) -> Result<Vec<i16>, OrchestratorError> {
        let seconds_per_beat = 60.0 / self.bpm as f64;

        let mut total_duration_in_beats: f64 = 0.0;
        for note in &self.notes {
            total_duration_in_beats = total_duration_in_beats.max(note.start_time + note.duration);
        }

        let total_duration_in_seconds = total_duration_in_beats * seconds_per_beat;
        let total_samples: usize = (total_duration_in_seconds * sample_rate as f64).ceil() as usize;

        // Create a vector with specified capacity and with default value = 0 to avoid reallocations
        // Creating it f64 because these samples are not clipped
        // This acts like the timeline
        let mut pcm_sample_sums: Vec<f64> = vec![0.0; total_samples];

        // Process each note and mix it at the same time
        for note in &self.notes {
            let wave = BezierOscillator::new(
                note.frequency()?,
                note.amplitude * CONDENSE_CONSTANT,
                sample_rate,
                self.control_points.clone(),
            )?;

            let start_sample = (note.start_time * seconds_per_beat * sample_rate as f64) as usize;
            let samples_for_this_note =
                (note.duration * seconds_per_beat * sample_rate as f64) as usize;

            for i in 0..samples_for_this_note {
                let current_sample_index = start_sample + i;
                if current_sample_index < total_samples {
                    let current_sample = wave.pcm_sample(i as u32);
                    pcm_sample_sums[current_sample_index] += current_sample as f64;
                }
            }
        }

        // Normalize, apply soft clipping with tanh, and convert to PCM
        let pcm_samples: Vec<i16> = pcm_sample_sums
            .iter()
            .map(|&sum| {
                // Normalize from accumulated PCM range back to [-1, 1]
                let normalized = sum / PCM_BIT_RANGE;
                // Apply soft clipping with tanh
                let clipped = normalized.tanh();
                // Convert back to PCM i16 range
                (clipped * PCM_BIT_RANGE) as i16
            })
            .collect();

        Ok(pcm_samples)
    }
}
