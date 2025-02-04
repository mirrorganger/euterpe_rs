enum WaveformType {
    Sine,
    Triangle,
    Sawtooth
}

struct Lfo{
    phase: f32,
    phase_increment: f32,
    waveform_type: WaveformType
}

impl Lfo{

    pub fn new(waveform_type: WaveformType, frequency : f32, sample_rate: f32) -> Self{
        Lfo{
            phase: 0.0,
            phase_increment: frequency / sample_rate,
            waveform_type
        }
    }

    pub fn update(&mut self, frequency: f32, sample_rate: f32){
        self.phase_increment = frequency / sample_rate;
    }

    pub fn advance(&mut self) -> f32{
        self.advance_counter();
        match self.waveform_type{
            WaveformType::Sine =>{
                (self.phase * std::f32::consts::PI * 2.0).sin()
            },
            WaveformType::Triangle =>{
              let out = (2.0 *  self.phase - 1.0).abs();
              2.0 * out - 1.0
            },
            WaveformType::Sawtooth =>{
                2.0 * self.phase - 1.0
            }
        }
    }

    fn advance_counter(&mut self){
        self.phase += self.phase_increment;

        if self.phase_increment > 0.0 && self.phase >= 1.0{
            self.phase -= 1.0;
        }

        if self.phase_increment < 0.0 && self.phase <= 0.0{
            self.phase += 1.0;
        }
    }

}

