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
    pub decay: AtomicFloat,
    pub sustain: AtomicFloat,
    pub release: AtomicFloat,

    // simple filter
    pub filter_cutoff: AtomicFloat,

    // wavetable oscillator
    pub wave_warp: AtomicFloat,

    // wavetable oscillator envelope
    pub warp_attack: AtomicFloat,
    pub warp_decay: AtomicFloat,
    pub warp_sustain: AtomicFloat,
    pub warp_release: AtomicFloat,

    pub warp_ratio: AtomicFloat,
}

impl PluginState {
    pub fn default() -> Self {
        Self {
            notebook: RwLock::new(Notebook::new()),
            // TODO update this with TimeInfo
            sample_rate: AtomicFloat::new(48000.0),
            main_volume: AtomicFloat::new(0.5),
            attack: AtomicFloat::new(0.05),
            decay: AtomicFloat::new(0.0),
            sustain: AtomicFloat::new(1.0),
            release: AtomicFloat::new(0.05),
            filter_cutoff: AtomicFloat::new(1.0),
            wave_warp: AtomicFloat::new(1.0),
            warp_attack: AtomicFloat::new(0.0),
            warp_decay: AtomicFloat::new(0.0),
            warp_sustain: AtomicFloat::new(1.0),
            warp_release: AtomicFloat::new(0.0),
            warp_ratio: AtomicFloat::new(0.5),
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
            2 => self.decay.set(value),
            3 => self.sustain.set(value),
            4 => self.release.set(value),
            5 => self.filter_cutoff.set(value),
            6 => self.wave_warp.set(value),
            7 => self.warp_attack.set(value),
            8 => self.warp_decay.set(value),
            9 => self.warp_sustain.set(value),
            10 => self.warp_release.set(value),
            11 => self.warp_ratio.set(value),

            _ => (),
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.main_volume.get(),
            1 => self.attack.get(),
            2 => self.decay.get(),
            3 => self.sustain.get(),
            4 => self.release.get(),
            5 => self.filter_cutoff.get(),
            6 => self.wave_warp.get(),
            7 => self.warp_attack.get(),
            8 => self.warp_decay.get(),
            9 => self.warp_sustain.get(),
            10 => self.warp_release.get(),
            11 => self.warp_ratio.get(),

            _ => 0.0,
        }
    }

    fn get_parameter_label(&self, index: i32) -> String {
        match index {
            0 => "Main Volume".to_string(),
            1 => "Attack".to_string(),
            2 => "Decay".to_string(),
            3 => "Sustain".to_string(),
            4 => "Release".to_string(),
            5 => "Filter Cutoff".to_string(),
            6 => "Wave Warp".to_string(),
            7 => "Warp Attack".to_string(),
            8 => "Warp Decay".to_string(),
            9 => "Warp Sustain".to_string(),
            10 => "Warp Release".to_string(),
            11 => "Warp Ratio".to_string(),

            _ => unreachable!(),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => "Main Volume".to_string(),
            1 => "Attack".to_string(),
            2 => "Decay".to_string(),
            3 => "Sustain".to_string(),
            4 => "Release".to_string(),
            5 => "Filter Cutoff".to_string(),
            6 => "Wave Warp".to_string(),
            7 => "Warp Attack".to_string(),
            8 => "Warp Decay".to_string(),
            9 => "Warp Sustain".to_string(),
            10 => "Warp Release".to_string(),
            11 => "Warp Ratio".to_string(),

            _ => unreachable!(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Main Volume".to_string(),
            1 => "Attack".to_string(),
            2 => "Decay".to_string(),
            3 => "Sustain".to_string(),
            4 => "Release".to_string(),
            5 => "Filter Cutoff".to_string(),
            6 => "Wave Warp".to_string(),
            7 => "Warp Attack".to_string(),
            8 => "Warp Decay".to_string(),
            9 => "Warp Sustain".to_string(),
            10 => "Warp Release".to_string(),
            11 => "Warp Ratio".to_string(),

            _ => unreachable!(),
        }
        .to_string()
    }
}
