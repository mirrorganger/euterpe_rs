use crate::processor::AudioProcessor;

pub struct OnePoleLPF {
    a0: f64,
    b1: f64,
    z1: f64,
}

impl OnePoleLPF {
    pub fn new() -> Self {
        OnePoleLPF {
            a0: 1.0,
            b1: 0.0,
            z1: 0.0,
        }
    }

    pub fn set_fb_gain(&mut self, gain: f64) {
        self.b1 = gain;
        self.a0 = 1.0 - gain;
    }
}

impl AudioProcessor<f64> for OnePoleLPF {
    fn process(&mut self, input: f64) -> f64 {
        self.z1 = self.a0 * input + self.b1 * self.z1;
        self.z1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onepole_lpf() {
        let mut uut = OnePoleLPF::new();
        uut.set_fb_gain(0.5);
        assert_eq!(uut.process(1.0), 0.5);
        assert_eq!(uut.process(0.0), 0.25);
        assert_eq!(uut.process(0.0), 0.125);
        assert_eq!(uut.process(0.0), 0.0625);
    }
}
