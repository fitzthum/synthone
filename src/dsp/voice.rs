// A synth voice
//
// A voice plays one note. All voices in the synth are the same.
// Each one can have multiple oscillators and other moodules.

use crate::dsp::envelope::{Envelope, ADSR};
use crate::dsp::oscillator::{Oscillator, Sine};
use crate::notes::Note;
use crate::plugin_state::PluginState;

use std::sync::Arc;

use log::*;

fn midi_pitch_to_freq(pitch: u8) -> f32 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f32 = 440.0;

    // Midi notes can be 0-127
    ((f32::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

pub struct Voice {
    note: Note,
    params: Arc<PluginState>,
}

impl Voice {
    pub fn new(note: Note, params: Arc<PluginState>) -> Voice {
        Voice { note, params }
    }
    // TODO: this should call an oscillator rather than do the computation
    //
    // might be cool to use generics to support f32 and f64 buffers
    pub fn play(&self, buffer_len: usize) -> Vec<f32> {
        // TODO should probably make a local copy of sample rate
        // so that we don't have to get this lock every time
        let time_per_sample = 1.0 / self.params.get_sample_rate();

        let oscillator = Sine::new(midi_pitch_to_freq(self.note.number));
        let envelope = ADSR::new(
            self.params.attack.get(),
            self.params.delay.get(),
            self.params.sustain.get(),
            self.params.release.get(),
        );

        let mut output = Vec::with_capacity(buffer_len);

        for i in 0..buffer_len {
            let time = self.note.time + (time_per_sample * (i as f32));
            let alpha = envelope.process(time, self.note.on, self.note.off_time);
            let sample = alpha * oscillator.process(time);

            output.push(sample);
        }
        output
    }
}
