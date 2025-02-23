use crate::mod_delay::ModulatedDelay;
use crate::lfo::WaveformType;
use crate::processor::AudioProcessor;

pub struct ModAllPass{
    mod_delay: ModulatedDelay,
    gain: f64,
}


impl ModAllPass{
    pub fn new (delay_ms : f32, mod_freq: f32, mod_wave: WaveformType, sample_rate_hz: f32) -> Self{
        ModAllPass{
            mod_delay: ModulatedDelay::new(delay_ms, mod_freq, mod_wave, sample_rate_hz),
            gain: 0.5
        }
    }

    pub fn prepare(&mut self, delay_ms: f32, mod_freq: f32, sample_rate_hz: f32){
        self.mod_delay.prepare(delay_ms, mod_freq,sample_rate_hz);
    }

    pub fn set_lfo_freq(&mut self, freq: f32){
        self.mod_delay.set_lfo_freq(freq);
    }
}


impl AudioProcessor<f64> for ModAllPass{
    fn process(&mut self, input: f64) -> f64{
        let delayed = self.mod_delay.advance();
        let current = input + delayed * self.gain;
        self.mod_delay.push(current);
        delayed - (current * self.gain)
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_instantiate_mod_all_pass(){
        let mut uut = ModAllPass::new(10.0, 1.0, WaveformType::Sine, 100.0);
        uut.prepare(10.0, 1.0, 100.0);
        uut.process(1.0);
    }
}
