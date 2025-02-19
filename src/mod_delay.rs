use crate::delay_line::{CircularBuffer, DelayLine};
use crate::lfo::{Lfo, WaveformType};

pub struct ModulatedDelay {
    delay_line: DelayLine,
    lfo: Lfo,
    lfo_depth: f32,
    mod_width_ms: f32,
    delay_ms: f32,
    sample_rate_hz: f32,
}

const MAX_DELAY_MS: f32 = 100.0;

impl ModulatedDelay {
    pub fn new(delay_ms: f32, mod_freq: f32, mod_wave: WaveformType, sample_rate_hz: f32) -> Self {
        ModulatedDelay {
            delay_line: DelayLine::new((MAX_DELAY_MS * sample_rate_hz / 1000.0) as usize), // 100 ms delay
            lfo: Lfo::new(mod_wave, mod_freq, sample_rate_hz),
            lfo_depth: 1.0,
            mod_width_ms: 0.1 * delay_ms,
            delay_ms,
            sample_rate_hz,
        }
    }

    pub fn prepare(&mut self, delay_ms: f32, mod_freq: f32, sample_rate_hz: f32) {
        self.sample_rate_hz = sample_rate_hz;
        self.delay_line.clear();
        self.set_delay(delay_ms);
        self.lfo.prepare(mod_freq, sample_rate_hz);
    }

    pub fn advance(&mut self) -> f64 {
        let mod_delay_ms = self.lfo.advance() * self.lfo_depth * self.mod_width_ms;
        let delay_samples = (self.delay_ms + mod_delay_ms) * self.sample_rate_hz / 1000.0;
        self.delay_line.get(delay_samples as f64)
    }

    pub fn push(&mut self, value: f64) {
        self.delay_line.push(value);
    }

    pub fn clear(&mut self) {
        self.delay_line.clear();
    }

    pub fn set_delay(&mut self, delay_ms: f32) {
        self.delay_ms = delay_ms;
        self.mod_width_ms = 0.1 * delay_ms;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_helpers::is_bounded_buffer;

    const DELAY_MS: f32 = 10.0;
    const MOD_FREQ: f32 = 100.0;
    const SAMPLE_RATE: f32 = 1000.0; // 1ms -> 1sample
    const DELAY_SAMPLES: usize = (DELAY_MS) as usize;
    const MOD_SAMPLES: usize = 2 * (0.1 * DELAY_SAMPLES as f32) as usize; // bipolar, +1, -1 sample
                                                                          //
    macro_rules! test_modulation {
        ($name:ident, $wave_type:expr) => {
            #[test]
            fn $name() {
                let mut mod_delay =
                    ModulatedDelay::new(DELAY_MS, MOD_FREQ, $wave_type, SAMPLE_RATE);
                let samples: Vec<f64> =
                    (0..DELAY_SAMPLES + MOD_SAMPLES).map(|x| x as f64).collect();
                let mut out_samples: Vec<f64> = Vec::new();

                for &sample in samples.iter() {
                    mod_delay.push(sample);
                }

                for _ in 0..12345 {
                    out_samples.push(mod_delay.advance());
                }
                let lower_bound = samples[0];
                let upper_bound = samples[MOD_SAMPLES];
                assert!(is_bounded_buffer(&out_samples, lower_bound, upper_bound));
            }
        };
    }

    test_modulation!(test_sine_modulation, WaveformType::Sine);
    test_modulation!(test_triangular_modulation, WaveformType::Triangle);
    test_modulation!(test_sawtooth_modulation, WaveformType::Sawtooth);
}
