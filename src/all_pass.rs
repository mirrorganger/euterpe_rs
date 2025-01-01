use crate::delay_line::{CircularBuffer, DelayLine};
use crate::processor::AudioProcessor;
struct AllPass {
    delay_line: DelayLine,
    gain: f64,
    delay_index: f64,
    freq_hz: f64,
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
    pub fn new(freq_hz: f64, delay_max_ms: f64, gain: f64) -> Self {
        AllPass {
            delay_line: DelayLine::new(get_length(delay_max_ms, freq_hz)),
            gain,
            delay_index: 0.0,
            freq_hz,
        }
    }

    pub fn set_gain(&mut self, gain: f64) {
        self.gain = gain;
    }

    pub fn set_delay(&mut self, delay_ms: f64) {
        self.delay_index = delay_ms * self.freq_hz / 1000.0;
    }
}

impl AudioProcessor<f64> for AllPass {
    fn process(&mut self, input: f64) -> f64 {
        let delayed = self.delay_line[self.delay_index as usize];
        let current = input + delayed * self.gain;
        self.delay_line.push(current);
        delayed - current * self.gain
    }
}
