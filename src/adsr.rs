pub enum ADSREnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

pub struct ADSREnvelope {
    attack: f64,  // time in seconds
    decay: f64,   // time in seconds
    sustain: f64, // amplitude 0.0 to 1.0
    release: f64, // time in seconds
    sample_rate: u32,
    current_state: ADSREnvelopeState,
    current_factor: f64,
    raw_duration_in_seconds: f64, // Note duration in seconds (not including release)
}

impl ADSREnvelope {
    pub fn new(
        attack: f64,
        decay: f64,
        sustain: f64,
        release: f64,
        sample_rate: u32,
        raw_duration_in_seconds: f64,
    ) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            sample_rate,
            current_state: ADSREnvelopeState::Attack,
            current_factor: 0.0,
            raw_duration_in_seconds,
        }
    }

    pub fn apply(&mut self, sample: f64, current_sample_index: u32) -> f64 {
        self.current_state = self.determine_state(current_sample_index);

        match self.current_state {
            ADSREnvelopeState::Attack => self.apply_attack(sample, current_sample_index),
            ADSREnvelopeState::Decay => self.apply_decay(sample, current_sample_index),
            ADSREnvelopeState::Sustain => self.apply_sustain(sample),
            ADSREnvelopeState::Release => self.apply_release(sample, current_sample_index),
        }
    }

    fn determine_state(&self, sample_index: u32) -> ADSREnvelopeState {
        // Release happens if sample index is greater than the duration in seconds
        if sample_index > (self.raw_duration_in_seconds * self.sample_rate as f64) as u32 {
            ADSREnvelopeState::Release
        }
        // Attack happens if sample index is less than the attack time
        else if sample_index < (self.attack * self.sample_rate as f64) as u32 {
            ADSREnvelopeState::Attack
        }
        // Decay happens if sample index is greater than the attack time and less than the decay time
        else if sample_index < ((self.attack + self.decay) * self.sample_rate as f64) as u32 {
            ADSREnvelopeState::Decay
        }
        // Sustain happens if any other case
        else {
            ADSREnvelopeState::Sustain
        }
    }

    fn apply_attack(&mut self, sample: f64, current_sample_index: u32) -> f64 {
        let t_a = self.attack * self.sample_rate as f64;
        let factor = current_sample_index as f64 / t_a;
        let amplitude = sample * factor;
        self.current_factor = factor;
        amplitude
    }

    fn apply_decay(&mut self, sample: f64, current_sample_index: u32) -> f64 {
        let t_d = self.decay * self.sample_rate as f64;
        let attack_end = self.attack * self.sample_rate as f64;
        let decay_start_index = current_sample_index as f64 - attack_end;
        let factor = 1.0 - (1.0 - self.sustain) * (decay_start_index / t_d);
        let amplitude = sample * factor;
        self.current_factor = factor;
        amplitude
    }

    fn apply_sustain(&mut self, sample: f64) -> f64 {
        let amplitude = sample * self.sustain;
        self.current_factor = self.sustain;
        amplitude
    }

    fn apply_release(&mut self, sample: f64, current_sample_index: u32) -> f64 {
        let t_r = self.release * self.sample_rate as f64;
        let t_release_at = self.raw_duration_in_seconds * self.sample_rate as f64;
        let factor =
            self.current_factor * (1.0 - ((current_sample_index as f64 - t_release_at) / t_r));
        let amplitude = sample * factor;
        self.current_factor = factor;
        amplitude
    }
}
