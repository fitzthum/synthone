// module for beep boop

use crate::plugin_state::PluginState;

mod voice;
use voice::Voice;

mod mixer;
use mixer::{Amp, Mixer, SimpleAmp, SimpleMixer};

mod filter;
use filter::{Filter, SimpleFilter};

use std::sync::Arc;
use vst::buffer::AudioBuffer;

pub mod envelope;
pub mod oscillator;

pub(super) struct PluginDsp {
    params: Arc<PluginState>,
}

impl PluginDsp {
    pub fn new(params: Arc<PluginState>) -> Self {
        Self { params }
    }

    pub fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let num_samples = buffer.samples();
        let num_channels = buffer.output_count();
        let (_inputs, mut outputs) = buffer.split();

        let mut mixer = SimpleMixer::new();

        let notes = self.params.notebook.read().unwrap().get_notes();
        if notes.is_empty() {
            return;
        }

        for note in notes {
            let voice = Voice::new(note, self.params.clone());
            let voice_output = voice.play(num_samples);
            mixer.add_input(voice_output, 1.0);
        }
        let mut mixed_output = mixer.process();

        let filter = SimpleFilter::new(
            self.params.filter_cutoff.get(),
            self.params.sample_rate.get(),
        );

        filter.process(&mut mixed_output);

        let amp = SimpleAmp::new(self.params.main_volume.get());
        amp.process(&mut mixed_output);

        // put the main output into the output channels
        // there is no support for true stereo synths
        for i in 0..num_samples {
            for n in 0..num_channels {
                let channel = outputs.get_mut(n);
                channel[i] = mixed_output[i];
            }
        }

        let block_time = num_samples as f32 / self.params.sample_rate.get();
        let mut notebook = self.params.notebook.write().unwrap();
        notebook.update_note_times(block_time);

        // TODO: dynamically calculate this threshold based on envelope parameters
        notebook.purge_old_notes(0.5);
    }
}
