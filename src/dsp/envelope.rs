// going postal

pub trait Envelope {
    // *time* is the time since note on.
    // *on* is whether or not the note is on
    // *time_off* is the time stamp that the note was turned off
    fn process(&self, time: f32, on: bool, time_off: f32) -> f32;
}

pub struct ADSR {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
}

impl ADSR {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        ADSR {
            attack,
            decay,
            sustain,
            release,
        }
    }
}

impl Envelope for ADSR {
    fn process(&self, time: f32, on: bool, time_off: f32) -> f32 {
        let mut alpha = 0.0;

        if on {
            if time < self.attack {
                alpha = time * (1.0 / self.attack)
            } else if time < self.attack + self.decay {
                alpha = 1.0 - (time - self.attack) * ((1.0 - self.sustain) / self.decay)
            } else {
                alpha = self.sustain;
            }
        } else {
            let time_since_off = time - time_off;
            if time_since_off < self.release {
                alpha = self.sustain - (time_since_off * (self.sustain / self.release))
            }
        }
        alpha
    }
}
