use std::sync::Arc;

use vst::{
    api::Supported,
    buffer::AudioBuffer,
    editor::Editor,
    plugin::{CanDo, HostCallback, Info, Plugin, PluginParameters, Category},
    event::Event,
    api::Events,
};

mod dsp;
use dsp::PluginDsp;

mod plugin_state;
use plugin_state::PluginState;

mod notes;


use log::*;
use simplelog::*;

use std::fs::File;

/// Top level wrapper that exposes a full `vst::Plugin` implementation.
struct Synth1Vst {
    host: HostCallback,
    dsp: PluginDsp,
    params: Arc<PluginState>,

}

impl Synth1Vst {
    fn new_maybe_host(maybe_host: Option<HostCallback>) -> Self {
        // a bit janky, but it can tricky to find logs via VST host
        CombinedLogger::init(
            vec![
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("~/Documents/vst.log").unwrap()),
            ]
        ).unwrap();
        //info!("STARTING PLUGIN");

        let host = maybe_host.unwrap_or_default();

        let params = Arc::new(PluginState::default());
        let dsp = PluginDsp::new(params.clone());

        Self {
            host,
            dsp,
            params,
        }
    }
}

impl Default for Synth1Vst {
    fn default() -> Self {
        Self::new_maybe_host(None)
    }
}
impl Plugin for Synth1Vst {
    fn new(host: HostCallback) -> Self {
        Self::new_maybe_host(Some(host))
    }

    fn get_info(&self) -> Info {
        Info {
            name: "synth1".to_string(),
            vendor: "fitzthum".to_string(),
            unique_id: 234292,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: 1,
            initial_delay: 0,
            preset_chunks: true,
            ..Info::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        self.dsp.process(buffer);
    }

    fn process_events(&mut self, events: &Events) {
        for event in events.events() {
            if let Event::Midi(ev) = event {
                match ev.data[0] {
                    0x90 => {
                        // note on
                        self.params.add_note(ev);
                
                    },
                    0x80 => {
                        // note off
                        self.params.remove_note(ev);
                    },
                    _ => (),
                }
            }
        }
    }

    fn can_do(&self, capability: CanDo) -> Supported {
        match capability {
            CanDo::ReceiveMidiEvent => Supported::Yes, 
            //CanDo::ReceiveTimeInfo => Supported::Yes, 
            _ => Supported::Maybe,
        }

    }

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }

    fn get_editor(&mut self) -> Option<Box<dyn Editor>> {
        // TODO worry about the editor later
        None
    }
}

vst::plugin_main!(Synth1Vst);
