use crate::errors::OrchestratorError;
use crate::oscillator::{BezierOscillator, SinOscillator};
use crate::validation::{validate_bpm, validate_control_points, validate_timeline_notes};
use serde::Deserialize;

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

        // Calculate total duration needed (convert beats to seconds)
        let total_duration = self
            .notes
            .iter()
            .map(|note| (note.start_time + note.duration) * seconds_per_beat)
            .fold(0.0, f64::max);

        let total_samples = (total_duration * sample_rate as f64).ceil() as usize;
        let mut samples = vec![0i16; total_samples];

        // Process each note and mix into the timeline
        for note in &self.notes {
            let wave = SinOscillator {
                amplitude: note.amplitude,
                frequency: note.frequency()?,
                sample_rate: sample_rate,
            };

            // Convert beats to seconds
            let start_time_seconds = note.start_time * seconds_per_beat;
            let duration_seconds = note.duration * seconds_per_beat;

            let start_sample = (start_time_seconds * sample_rate as f64) as usize;
            let samples_per_note = (duration_seconds * sample_rate as f64) as usize;

            for i in 0..samples_per_note {
                let sample_index = start_sample + i;
                if sample_index < total_samples {
                    // Mix the samples (simple addition, with clamping)
                    let new_sample = wave.pcm_sample(i as u32) as f64;
                    let existing_sample = samples[sample_index] as f64;
                    let mixed = (new_sample + existing_sample).clamp(-32768.0, 32767.0) as i16;
                    samples[sample_index] = mixed;
                }
            }
        }
        Ok(samples)
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

        // Calculate total duration needed (convert beats to seconds)
        let total_duration = self
            .notes
            .iter()
            .map(|note| (note.start_time + note.duration) * seconds_per_beat)
            .fold(0.0, f64::max);

        let total_samples = (total_duration * sample_rate as f64).ceil() as usize;
        let mut samples = vec![0i16; total_samples];

        // Process each note and mix into the timeline
        for note in &self.notes {
            let wave = BezierOscillator::new(
                note.frequency()?,
                note.amplitude,
                sample_rate,
                self.control_points.clone(),
            )?;

            // Convert beats to seconds
            let start_time_seconds = note.start_time * seconds_per_beat;
            let duration_seconds = note.duration * seconds_per_beat;

            let start_sample = (start_time_seconds * sample_rate as f64) as usize;
            let samples_per_note = (duration_seconds * sample_rate as f64) as usize;

            for i in 0..samples_per_note {
                let sample_index = start_sample + i;
                if sample_index < total_samples {
                    // Mix the samples (simple addition, with clamping)
                    let new_sample = wave.pcm_sample(i as u32) as f64;
                    let existing_sample = samples[sample_index] as f64;
                    let mixed = (new_sample + existing_sample).clamp(-32768.0, 32767.0) as i16;
                    samples[sample_index] = mixed;
                }
            }
        }
        Ok(samples)
    }
}
