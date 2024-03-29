use std::sync::Arc;

#[macro_use]
extern crate lazy_static;

use vst::{
    api::Events,
    api::Supported,
    buffer::AudioBuffer,
    editor::Editor,
    event::Event,
    plugin::{CanDo, Category, HostCallback, Info, Plugin, PluginParameters},
};

mod dsp;
use dsp::PluginDsp;

mod editor;

mod plugin_state;
use plugin_state::PluginState;

mod notes;
use editor::PluginEditor;

use log::*;
use simplelog::*;

use std::fs::File;

/// Top level wrapper that exposes a full `vst::Plugin` implementation.
struct Synth1Vst {
    host: HostCallback,
    dsp: PluginDsp,
    editor: Option<PluginEditor>,
    params: Arc<PluginState>,
}

impl Synth1Vst {
    fn new_maybe_host(maybe_host: Option<HostCallback>) -> Self {
        // a bit janky, but it can tricky to find logs via VST host
        CombinedLogger::init(vec![WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("/tmp/SynthOne.log").unwrap(),
        )])
        .unwrap();
        info!("STARTING PLUGIN");

        let host = maybe_host.unwrap_or_default();

        let params = Arc::new(PluginState::default());

        let editor = Some(PluginEditor {
            params: params.clone(),
            window_handle: None,
            is_open: false,
        });

        let dsp = PluginDsp::new(params.clone());

        Self {
            host,
            dsp,
            editor,
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
            name: "SynthOne".to_string(),
            vendor: "fitzthum".to_string(),
            unique_id: 234292,
            category: Category::Synth,
            inputs: 0,
            outputs: 2,
            parameters: 12,
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
                        self.params.note_on(ev);
                    }
                    0x80 => {
                        // note off
                        self.params.note_off(ev);
                    }
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
        info!("get editor");
        if let Some(editor) = self.editor.take() {
            Some(Box::new(editor) as Box<dyn Editor>)
        } else {
            None
        }
    }
}

vst::plugin_main!(Synth1Vst);
