pub enum WaveformType {
    Sine,
    Triangle,
    Sawtooth,
}

pub struct Lfo {
    phase: f32,
    phase_increment: f32,
    waveform_type: WaveformType,
}

impl Lfo {
    pub fn new(waveform_type: WaveformType, frequency: f32, sample_rate: f32) -> Self {
        Lfo {
            phase: 0.0,
            phase_increment: frequency / sample_rate,
            waveform_type,
        }
    }

    pub fn prepare(&mut self, frequency: f32, sample_rate_hz: f32) {
        self.phase_increment = frequency / sample_rate_hz;
    }

    pub fn advance(&mut self) -> f32 {
        let out = match self.waveform_type {
            WaveformType::Sine => (self.phase * std::f32::consts::PI * 2.0).sin(),
            WaveformType::Triangle => {
                let out = (2.0 * self.phase - 1.0).abs();
                2.0 * out - 1.0
            }
            WaveformType::Sawtooth => 2.0 * self.phase - 1.0,
        };
        self.advance_counter();
        out
    }

    fn advance_counter(&mut self) {
        self.phase += self.phase_increment;

        if self.phase_increment > 0.0 && self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        if self.phase_increment < 0.0 && self.phase <= 0.0 {
            self.phase += 1.0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_helpers::{is_bounded_buffer, is_decreasing_buffer, is_increasing_buffer};
    use approx::assert_abs_diff_eq;
    const FREQ: f32 = 1.0;
    const SAMPLE_RATE: f32 = 100.0;
    const MAX_ERROR: f32 = 1.0e-4;
    const STEPS_PER_CYCLE: usize = (SAMPLE_RATE / FREQ) as usize;
    #[test]
    fn test_lfo_saw() {
        let mut lfo = Lfo::new(WaveformType::Sawtooth, FREQ, SAMPLE_RATE);
        let mut modulation: Vec<f32> = Vec::new();

        for _ in 0..STEPS_PER_CYCLE + 1 {
            modulation.push(lfo.advance());
        }

        assert_abs_diff_eq!(modulation[0], -1.0, epsilon = MAX_ERROR);
        assert_abs_diff_eq!(modulation[STEPS_PER_CYCLE / 2], 0.0, epsilon = MAX_ERROR);
        assert_abs_diff_eq!(modulation[STEPS_PER_CYCLE], 1.0, epsilon = MAX_ERROR);
        assert!(is_bounded_buffer(&modulation, -1.0, 1.0));
        assert!(is_increasing_buffer(&modulation));
    }

    #[test]
    fn test_lfo_triangle() {
        let mut lfo = Lfo::new(WaveformType::Triangle, FREQ, SAMPLE_RATE);
        let mut modulation: Vec<f32> = Vec::new();
        for _ in 0..STEPS_PER_CYCLE + 1 {
            modulation.push(lfo.advance());
        }

        assert_abs_diff_eq!(modulation[0], 1.0, epsilon = MAX_ERROR);
        assert_abs_diff_eq!(modulation[STEPS_PER_CYCLE / 2], -1.0, epsilon = MAX_ERROR);
        assert_abs_diff_eq!(modulation[STEPS_PER_CYCLE], 1.0, epsilon = MAX_ERROR);
        assert!(is_bounded_buffer(&modulation, -1.0, 1.0));
        assert!(is_decreasing_buffer(&modulation[0..STEPS_PER_CYCLE / 2]));
        assert!(is_increasing_buffer(&modulation[STEPS_PER_CYCLE / 2..]));
    }

    #[test]
    fn test_lfo_sine() {
        let mut lfo = Lfo::new(WaveformType::Sine, FREQ, SAMPLE_RATE);
        let mut modulation: Vec<f32> = Vec::new();
        for _ in 0..STEPS_PER_CYCLE + 1 {
            modulation.push(lfo.advance());
        }
        assert_abs_diff_eq!(modulation[0], 0.0, epsilon = MAX_ERROR);
        assert_abs_diff_eq!(modulation[STEPS_PER_CYCLE / 4], 1.0, epsilon = MAX_ERROR);
        assert_abs_diff_eq!(modulation[STEPS_PER_CYCLE / 2], 0.0, epsilon = MAX_ERROR);
        assert_abs_diff_eq!(
            modulation[STEPS_PER_CYCLE * 3 / 4],
            -1.0,
            epsilon = MAX_ERROR
        );
        assert_abs_diff_eq!(modulation[STEPS_PER_CYCLE], 0.0, epsilon = MAX_ERROR);
        assert!(is_bounded_buffer(&modulation, -1.0, 1.0));
    }
}
