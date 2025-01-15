use crate::all_pass::AllPass;
use crate::comb::Comb;

const NUM_COMBS: usize = 4;
const NUM_ALLPASSES: usize = 2;

pub struct Schroeder {
    combs: [(Comb, f64); NUM_COMBS],
    all_passes: [(AllPass, f64); NUM_ALLPASSES],
    sample_rate: f64,
    dry_wet_mxi: f64,
}
