use std::f32::consts::PI;

pub trait Oscillator {
    fn process(&self, time: f32) -> f32;
}

// Debatable whether frequency should go in the struct 
// or be passed in via process.
pub struct Sine {
    frequency: f32
}

impl Sine {
    pub fn new(frequency: f32) -> Self {
        Sine {
            frequency,
        }
    }   
}

impl Oscillator for Sine {
    fn process(&self, time: f32) -> f32 {
       (time * self.frequency * PI * 2.0).sin()  
    }
}
