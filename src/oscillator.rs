use std::f64::consts::PI;

const PCM_BIT_RANGE: u32 = 2_u32.pow(16 - 1) - 1;

pub struct SinOscillator {
    pub frequency: f64,
    pub amplitude: f64,
    pub sample_rate: u32,
}

impl SinOscillator {
    // Generate sample's sin value at given time
    pub fn sample(&self, sample_index: u32) -> f64 {
        let x = (2.0 * PI * self.frequency * sample_index as f64) / self.sample_rate as f64;
        self.amplitude * x.sin()
    }

    // Returns the sample converted to a 16bit PCM int
    pub fn pcm_sample(&self, sample_index: u32) -> i16 {
        // Clamp the value to handle clipping
        let float_sample = self.sample(sample_index).clamp(-1.0, 1.0);
        let pcm_value = (float_sample * (PCM_BIT_RANGE as f64)) as i16;
        pcm_value
    }
}

pub struct BezierOscillator {
    pub frequency: f64,
    pub amplitude: f64,
    pub sample_rate: u32,
    pub control_points: Vec<f64>,
}

impl BezierOscillator {
    pub fn new(frequency: f64, amplitude: f64, sample_rate: u32, control_points: Vec<f64>) -> Self {
        if control_points.len() != 4 {
            panic!("BezierOscillator requires exactly 4 control points");
        }
        for point in &control_points {
            if *point < -1.0 || *point > 1.0 {
                panic!("Control point must be between -1.0 and 1.0");
            }
        }
        Self {
            frequency,
            amplitude,
            sample_rate,
            control_points,
        }
    }

    pub fn sample(&self, sample_index: u32) -> f64 {
        // phase of the wave at the given sample index
        let phase: f64 = ((sample_index as f64 * self.frequency) / self.sample_rate as f64).fract();
        let bezier_value = self.calculate_bezier_value(phase);
        bezier_value * self.amplitude as f64
    }

    fn calculate_bezier_value(&self, t: f64) -> f64 {
        let p0 = self.control_points[0];
        let p1 = self.control_points[1];
        let p2 = self.control_points[2];
        let p3 = self.control_points[3];
        let one_minus_t = 1.0 - t;
        one_minus_t.powf(3.0) * p0
            + 3.0 * one_minus_t.powf(2.0) * t * p1
            + 3.0 * one_minus_t * t.powf(2.0) * p2
            + t.powf(3.0) * p3
    }

    pub fn pcm_sample(&self, sample_index: u32) -> i16 {
        let float_sample = self.sample(sample_index).clamp(-1.0, 1.0);
        let pcm_value = (float_sample * (PCM_BIT_RANGE as f64)) as i16;
        pcm_value
    }
}
