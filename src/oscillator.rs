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
