use crate::delay_line::{CircularBuffer, DelayLine};
use crate::processor::AudioProcessor;
struct AllPass {
    delay_line: DelayLine,
    gain: f64,
    delay_index: f64,
}

fn get_length(delay_ms: f64, freq_hz: f64) -> usize {
    let delay_samples = (delay_ms * freq_hz / 1000.0) as usize;
    if delay_samples == 0 {
        1
    } else {
        delay_samples
    }
}

impl AllPass {
    pub fn new(delay_max_samples: usize) -> Self {
        AllPass {
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

impl AudioProcessor<f64> for AllPass {
    fn process(&mut self, input: f64) -> f64 {
        let delayed = self.delay_line[self.delay_index as usize];
        let current = input + delayed * self.gain;
        self.delay_line.push(current);
        delayed - (current * self.gain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_pass() {
        let mut uut = AllPass::new(4);
        uut.prepare(2.0, 0.5);
        assert_eq!(uut.process(1.0), -0.5);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.75);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.375);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.0);
        assert_eq!(uut.process(0.0), 0.1875);
    }
}
