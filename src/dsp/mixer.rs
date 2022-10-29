// for combining signals
//
// split into multiple files if we ever get more than two

pub trait Mixer {
    fn add_input(&mut self, channel: Vec<f32>, gain: f32);
    fn process(&self) -> Vec<f32>;
}

pub struct SimpleMixer {
    channels: Vec<Vec<f32>>,
    gains: Vec<f32>,
}

impl SimpleMixer {
    pub fn new() -> Self {
        SimpleMixer {
            channels: Vec::new(),
            gains: Vec::new(),
        }
    }
}

impl Mixer for SimpleMixer {
    fn add_input(&mut self, channel: Vec<f32>, gain: f32) {
        self.channels.push(channel);
        self.gains.push(gain);
    }

    fn process(&self) -> Vec<f32> {
        let buffer_size = self.channels[0].len();

        let mut output = vec![0.0; buffer_size];

        for (i, channel) in self.channels.iter().enumerate() {
            for j in 0..buffer_size {
                output[j] += channel[j] * self.gains[i];
            }
        }

        output
    }
}

pub trait Amp {
    fn new(volume: f32) -> Self;
    fn process(&self, buffer: &mut Vec<f32>);
}

pub struct SimpleAmp {
    volume: f32,
}

impl Amp for SimpleAmp {
    // move this into the trait to standardize the volume scale
    fn new(volume: f32) -> Self {
        SimpleAmp { volume }
    }

    fn process(&self, buffer: &mut Vec<f32>) {
        for i in 0..buffer.len() {
            buffer[i] *= self.volume;
        }
    }
}
