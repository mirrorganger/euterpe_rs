use crate::delay_line::{CircularBuffer, DelayLine};
use crate::onepole_lpf::OnePoleLPF;
use crate::processor::AudioProcessor;
struct Comb {
    delay_line: DelayLine,
    gain: f64,
    delay_index: f64,
    lpf_enabled: bool,
    onepole_lpf: OnePoleLPF,
}

impl Comb {
    pub fn new(delay_max_samples: usize, lpf_enabled: bool) -> Self {
        Comb {
            delay_line: DelayLine::new(delay_max_samples),
            gain: 0.0,
            delay_index: 0.0,
            lpf_enabled,
            onepole_lpf: OnePoleLPF::new(),
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
        self.onepole_lpf.set_fb_gain(dampening);
    }
}

impl AudioProcessor<f64> for Comb {
    fn process(&mut self, input: f64) -> f64 {
        let mut yn = self.delay_line[self.delay_index as usize];
        if self.lpf_enabled {
            yn = self.onepole_lpf.process(yn);
        }
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
        let mut uut = Comb::new(4,false);
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
