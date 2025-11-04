mod orchestrator;
mod oscillator;
mod wav;

pub use orchestrator::Note;
use orchestrator::{BezierOrchestrator, Orchestrator};
use oscillator::{BezierOscillator, SinOscillator};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn single_sine_wave(
    frequency: f64,
    amplitude: f64,
    sample_rate: u32,
    duration: f64,
) -> Vec<u8> {
    let oscillator = SinOscillator {
        frequency,
        amplitude,
        sample_rate,
    };

    let num_samples = (duration * sample_rate as f64) as u32;
    let mut pcm_samples = Vec::with_capacity(num_samples as usize);

    for i in 0..num_samples {
        pcm_samples.push(oscillator.pcm_sample(i));
    }

    wav::generate_bytes(&pcm_samples, sample_rate)
}

#[wasm_bindgen]
pub fn single_bezier_wave(
    control_points: Vec<f64>,
    frequency: f64,
    amplitude: f64,
    sample_rate: u32,
    duration: f64,
) -> Vec<u8> {
    let oscillator = BezierOscillator::new(frequency, amplitude, sample_rate, control_points);

    let num_samples = (duration * sample_rate as f64) as u32;
    let mut pcm_samples = Vec::with_capacity(num_samples as usize);

    for i in 0..num_samples {
        pcm_samples.push(oscillator.pcm_sample(i));
    }

    wav::generate_bytes(&pcm_samples, sample_rate)
}

#[wasm_bindgen]
pub fn sine_orchestrator(bpm: u8, notes: JsValue, sample_rate: u32) -> Result<Vec<u8>, JsValue> {
    let notes: Vec<Note> = serde_wasm_bindgen::from_value(notes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse notes: {}", e)))?;

    let orchestrator = Orchestrator::new(bpm, notes);
    let pcm_samples = orchestrator.pcm_samples(sample_rate);

    Ok(wav::generate_bytes(&pcm_samples, sample_rate))
}

#[wasm_bindgen]
pub fn bezier_orchestrator(
    bpm: u8,
    notes: JsValue,
    control_points: Vec<f64>,
    sample_rate: u32,
) -> Result<Vec<u8>, JsValue> {
    let notes: Vec<Note> = serde_wasm_bindgen::from_value(notes)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse notes: {}", e)))?;

    let orchestrator = BezierOrchestrator::new(bpm, notes, control_points);
    let pcm_samples = orchestrator.pcm_samples(sample_rate);

    Ok(wav::generate_bytes(&pcm_samples, sample_rate))
}
