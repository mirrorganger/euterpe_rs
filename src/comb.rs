use crate::delay_line::{CircularBuffer, DelayLine};
use crate::processor::AudioProcessor;
struct Comb {
    delay_line: DelayLine,
    gain: f64,
    delay_index: f64,
}

impl Comb {
    pub fn new(delay_max_samples: usize) -> Self {
        Comb {
            delay_line: DelayLine::new(delay_max_samples),
            gain: 0.0,
            delay_index: 0.0,
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
}

impl AudioProcessor<f64> for Comb {
    fn process(&mut self, input: f64) -> f64 {
        let yn = self.delay_line[self.delay_index as usize];
        let current = input + yn * self.gain;
        self.delay_line.push(current);
        yn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb() {
        let mut uut = Comb::new(4);
        uut.prepare(2.0, 0.5);
        assert_eq!(uut.process(1.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 1.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.5);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.25);
    }
}
