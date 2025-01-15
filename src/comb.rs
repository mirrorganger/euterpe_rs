use crate::delay_line::{CircularBuffer, DelayLine};
use crate::processor::AudioProcessor;
pub struct Comb {
    delay_line: DelayLine,
    gain: f64,
    delay_index: f64,
    lpf_enabled: bool,
    lpf_state: f64,
    dampening: f64,
}

impl Comb {
    pub fn new(delay_max_samples: usize, lpf_enabled: bool) -> Self {
        Comb {
            delay_line: DelayLine::new(delay_max_samples),
            gain: 0.0,
            delay_index: 0.0,
            lpf_enabled,
            lpf_state: 0.0,
            dampening: 0.5,
        }
    }

    pub fn prepare(&mut self, delay: f64, gain: f64) {
        self.delay_index = delay;
        self.gain = gain;
        self.delay_line.clear();
    }

    pub fn set_gain(&mut self, gain: f64) {
        self.gain = gain;
    }

    pub fn set_dampening(&mut self, dampening: f64) {
        self.dampening = dampening;
    }
}

impl AudioProcessor<f64> for Comb {
    fn process(&mut self, input: f64) -> f64 {
        let yn = self.delay_line[self.delay_index as usize - 1];
        let current: f64;
        if self.lpf_enabled {
            let g2 = self.dampening * (1.0 - self.gain);
            let filtered = yn + g2 * self.lpf_state;
            self.lpf_state = yn + g2 * self.lpf_state;
            current = input + self.gain * filtered;
        } else {
            current = input + self.gain * yn;
        }
        self.delay_line.push(current);
        yn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb() {
        let mut uut = Comb::new(4, false);
        uut.prepare(2.0, 0.5);
        assert_eq!(uut.process(1.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 1.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.5);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.25);
    }

    #[test]
    fn test_comb_lpf() {
        let mut uut = Comb::new(4, true);
        uut.prepare(2.0, 0.5);
        uut.set_dampening(0.5);
        assert_eq!(uut.process(1.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 1.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.5);
        assert_eq!(uut.process(0.0), 0.125);
    }
}
