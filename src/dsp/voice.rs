// A synth voice
//
// A voice plays one note. All voices in the synth are the same.
// Each one can have multiple oscillators and other moodules.

use crate::dsp::envelope::{Envelope, ADSR};
use crate::dsp::oscillator::{Oscillator, WaveTableOscillator};
use crate::notes::Note;
use crate::plugin_state::PluginState;

use std::sync::Arc;

fn midi_pitch_to_freq(pitch: u8) -> f32 {
    const A4_PITCH: i8 = 69;
    const A4_FREQ: f32 = 440.0;

    // Midi notes can be 0-127
    ((f32::from(pitch as i8 - A4_PITCH)) / 12.).exp2() * A4_FREQ
}

pub struct Voice {
    note: Note,
    params: Arc<PluginState>,
    sample_rate: f32,
    wave_warp: f32,
}

impl Voice {
    pub fn new(note: Note, params: Arc<PluginState>) -> Voice {
        let sample_rate = params.sample_rate.get();
        let wave_warp = params.wave_warp.get();
        Voice {
            note,
            params,
            sample_rate,
            wave_warp,
        }
    }
    pub fn play(&self, buffer_len: usize) -> Vec<f32> {
        // TODO should probably make a local copy of sample rate
        // so that we don't have to get this lock every time
        let time_per_sample = 1.0 / self.sample_rate;

        let warp_envelope = ADSR::new(
            self.params.warp_attack.get(),
            self.params.warp_delay.get(),
            self.params.warp_sustain.get(),
            self.params.warp_release.get(),
        );
        let warp_alpha = (self.params.warp_ratio.get() - 0.5)
            * warp_envelope.process(self.note.time, self.note.on, self.note.off_time);

        let oscillator = WaveTableOscillator::new(
            midi_pitch_to_freq(self.note.number),
            self.sample_rate,
            (self.wave_warp + warp_alpha).max(-1.0).min(1.0),
        );
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
