// Keep track of state for synth

use std::sync::RwLock;

use vst::{
    plugin::PluginParameters,
    util::AtomicFloat,
};

use vst::event::MidiEvent;

use crate::notes::Notebook;

pub struct PluginState {
    pub notebook: RwLock<Notebook>,
    sample_rate: AtomicFloat,
    pub main_volume: AtomicFloat,
}

impl PluginState {
    pub fn default() -> Self {
        Self {
            notebook: RwLock::new(Notebook::new()),
            // TODO update this with TimeInfo
            sample_rate: AtomicFloat::new(48000.0),
            main_volume: AtomicFloat::new(0.0),
        }
    }

    pub fn add_note(&self, e: MidiEvent) {
        self.notebook.write().unwrap().add_note(e);
    }

    pub fn remove_note(&self, e: MidiEvent) {
        self.notebook.write().unwrap().remove_note(e);

    }

    pub fn get_sample_rate(&self) -> f32 {
        self.sample_rate.get() as f32
    }
}

impl PluginParameters for PluginState {
    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.main_volume.set(value),
            _ => ()
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.main_volume.get(),
            _ => 0.0
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "Main Volume".to_string(),
            _ => unreachable!(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => "Main Volume".to_string(),
            _ => unreachable!(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Main Volume",
            _ => unreachable!(),
        }
        .to_string()
    }
}
