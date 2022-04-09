// Keep track of state for synth

use std::sync::RwLock;

use vst::{plugin::PluginParameters, util::AtomicFloat};

use vst::event::MidiEvent;

use crate::notes::Notebook;

pub struct PluginState {
    pub notebook: RwLock<Notebook>,
    pub sample_rate: AtomicFloat,

    // amp
    pub main_volume: AtomicFloat,

    // adsr envelope
    pub attack: AtomicFloat,
    pub delay: AtomicFloat,
    pub sustain: AtomicFloat,
    pub release: AtomicFloat,

    // simple filter
    pub filter_cutoff: AtomicFloat,

    // wavetable oscillator
    pub wave_warp: AtomicFloat,
}

impl PluginState {
    pub fn default() -> Self {
        Self {
            notebook: RwLock::new(Notebook::new()),
            // TODO update this with TimeInfo
            sample_rate: AtomicFloat::new(48000.0),
            main_volume: AtomicFloat::new(0.5),
            attack: AtomicFloat::new(0.0),
            delay: AtomicFloat::new(0.0),
            sustain: AtomicFloat::new(1.0),
            release: AtomicFloat::new(0.0),
            filter_cutoff: AtomicFloat::new(1.0),
            wave_warp: AtomicFloat::new(1.0),
        }
    }

    pub fn note_on(&self, e: MidiEvent) {
        self.notebook.write().unwrap().note_on(e);
    }

    pub fn note_off(&self, e: MidiEvent) {
        self.notebook.write().unwrap().note_off(e);
    }
}

impl PluginParameters for PluginState {
    fn set_parameter(&self, index: i32, value: f32) {
        match index {
            0 => self.main_volume.set(value),
            1 => self.attack.set(value),
            2 => self.delay.set(value),
            3 => self.sustain.set(value),
            4 => self.release.set(value),
            5 => self.filter_cutoff.set(value),
            6 => self.wave_warp.set(value),
            _ => (),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.main_volume.get(),
            1 => self.attack.get(),
            2 => self.delay.get(),
            3 => self.sustain.get(),
            4 => self.release.get(),
            5 => self.filter_cutoff.get(),
            6 => self.wave_warp.get(),
            _ => 0.0,
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "Main Volume".to_string(),
            1 => "Attack".to_string(),
            2 => "Delay".to_string(),
            3 => "Sustain".to_string(),
            4 => "Release".to_string(),
            5 => "Filter Cutoff".to_string(),
            6 => "Wave Warp".to_string(),
            _ => unreachable!(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => "Main Volume".to_string(),
            1 => "Attack".to_string(),
            2 => "Delay".to_string(),
            3 => "Sustain".to_string(),
            4 => "Release".to_string(),
            5 => "Filter Cutoff".to_string(),
            6 => "Wave Warp".to_string(),
            _ => unreachable!(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Main Volume".to_string(),
            1 => "Attack".to_string(),
            2 => "Delay".to_string(),
            3 => "Sustain".to_string(),
            4 => "Release".to_string(),
            5 => "Filter Cutoff".to_string(),
            6 => "Wave Warp".to_string(),
            _ => unreachable!(),
        }
        .to_string()
    }
}
