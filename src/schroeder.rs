use crate::all_pass::AllPass;
use crate::comb::Comb;

const NUM_COMBS: usize = 4;
const NUM_APF: usize = 2;

const COMB_DELAYS_MS: [f64; NUM_COMBS] = [29.7, 32.2, 38.1, 45.6];
const APF_DELAYS_MS: [f64; NUM_APF] = [2.3, 3.7];
const COMB_MAX_DELAY_MS: f64 = 50.0;
const APF_MAX_DELAY_MS: f64 = 20.0;

fn get_length_in_samples(length_ms: f64, sample_rate_hz: f64) -> f64 {
    sample_rate_hz * length_ms / 1000.0
}

fn get_gain_from_rt60(delay_ms: f64, rt60_ms: f64) -> f64 {
    let base: f64 = 10.0;
    base.powf(3.0 * delay_ms / rt60_ms)
}

pub struct Schroeder {
    combs: [(Comb, f64); NUM_COMBS],
    all_passes: [(AllPass, f64); NUM_APF],
    sample_rate: f64,
    dry_wet_mix: f64,
}

impl Schroeder {
    pub fn new(sample_rate: f64) -> Self {
        let comb_delay_length = get_length_in_samples(COMB_MAX_DELAY_MS, sample_rate) as usize;
        let apf_delay_length = get_length_in_samples(COMB_MAX_DELAY_MS, sample_rate) as usize;

        let schroeder = Schroeder {
            combs: [
                (Comb::new(comb_delay_length, true), COMB_DELAYS_MS[0]),
                (Comb::new(comb_delay_length, true), COMB_DELAYS_MS[1]),
                (Comb::new(comb_delay_length, true), COMB_DELAYS_MS[2]),
                (Comb::new(comb_delay_length, true), COMB_DELAYS_MS[3]),
            ],
            all_passes: [
                (AllPass::new(apf_delay_length), APF_DELAYS_MS[0]),
                (AllPass::new(apf_delay_length), APF_DELAYS_MS[1]),
            ],
            sample_rate,
            dry_wet_mix: 0.5,
        };
        schroeder
    }

    pub fn prepare(&mut self, sample_rate: f64, rt60_ms: f64) {
        for (comb, delay_ms) in self.combs.iter_mut() {
            let delay_samples = get_length_in_samples(*delay_ms, sample_rate);
            let gain = get_gain_from_rt60(*delay_ms, rt60_ms);
            comb.prepare(delay_samples, gain);
        }

        for (all_pass, delay_ms) in self.all_passes.iter_mut() {
            let delay_samples = get_length_in_samples(*delay_ms, sample_rate);
            all_pass.prepare(delay_samples, 0.5 * (2f64).sqrt());
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_creation(){
        let sample_rate = 44100.0;
        let rt60_ms = 20.0;
        let mut uut = super::Schroeder::new(sample_rate);
        uut.prepare(sample_rate, rt60_ms);
    }
}

