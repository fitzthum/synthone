use log::*;
use rust_embed::RustEmbed;
use std::f32::consts::PI;
use std::fs;

use serde::Deserialize;

const WAVE_TABLE_LENGTH: usize = 4096;

lazy_static! {
    pub static ref WAVE_TABLE: WaveTable = WaveTable::new();
}

pub trait Oscillator {
    fn process(&self, time: f32) -> f32;
}

// Debatable whether frequency should go in the struct
// or be passed in via process.
pub struct Sine {
    frequency: f32,
}

impl Sine {
    pub fn new(frequency: f32) -> Self {
        Sine { frequency }
    }
}

impl Oscillator for Sine {
    fn process(&self, time: f32) -> f32 {
        (time * self.frequency * PI * 2.0).sin()
    }
}

#[derive(RustEmbed)]
#[folder = "waves"]
#[include = "*.json"]
struct WaveFiles;

// Struct to store the waves
pub struct WaveTable {
    waves: Vec<Wave>,
}

impl WaveTable {
    pub fn new() -> Self {
        let mut waves: Vec<Wave> = vec![];

        for path in WaveFiles::iter() {
            let f = WaveFiles::get(&path).unwrap().data;
            waves.push(serde_json::from_str(std::str::from_utf8(&f).unwrap()).unwrap());
        }

        WaveTable { waves }
    }
}

// might be overkill having two different structs
#[derive(Deserialize)]
pub struct Wave {
    samples: Vec<f32>,
}

/*
 * Basic Wave Table Oscillator
 *
 */
pub struct WaveTableOscillator {
    sample_rate: f32,
    samples_per_cycle: f32,
    scale_factor: f32,
    scaled_warp: f32,
    wave_index_a: f32,
    wave_index_b: f32,
}

impl WaveTableOscillator {
    pub fn new(frequency: f32, sample_rate: f32, wave_warp: f32) -> Self {
        let samples_per_cycle = sample_rate / frequency;
        let scale_factor = WAVE_TABLE_LENGTH as f32 / samples_per_cycle;

        // wave_warp is a float between 0.0 and 1.0. We want to use this to
        // switch between N waves
        let mut wave_index_a = 0.0;
        let mut wave_index_b = 1.0;
        let mut scaled_warp = wave_warp;

        let n_waves = WAVE_TABLE.waves.len();
        let wave_width = 1.0 / (n_waves - 1) as f32;

        // there is probably a way to do this arithmetically
        if n_waves > 2 {
            for i in 0..(n_waves - 1) {
                let maybe_a = i as f32;
                let maybe_b = (i + 1) as f32;

                let maybe_a_threshold = maybe_a * wave_width;
                let maybe_b_threshold = maybe_b * wave_width;

                if wave_warp <= maybe_b_threshold {
                    wave_index_a = maybe_a;
                    wave_index_b = maybe_b;

                    scaled_warp = (wave_warp - maybe_a_threshold) / wave_width;
                    break;
                }
            }
        }

        WaveTableOscillator {
            sample_rate,
            samples_per_cycle,
            scale_factor,
            scaled_warp,
            wave_index_a,
            wave_index_b,
        }
    }
}

impl Oscillator for WaveTableOscillator {
    fn process(&self, time: f32) -> f32 {
        let total_sample_offset = time * self.sample_rate;
        let unscaled_sample_offset = total_sample_offset % self.samples_per_cycle;

        // hopefully the way we do the rounding won't land us out of bounds
        let table_offset = unscaled_sample_offset * self.scale_factor;

        let sample_a = WAVE_TABLE.waves[self.wave_index_a as usize].samples[table_offset as usize];
        let sample_b = WAVE_TABLE.waves[self.wave_index_b as usize].samples[table_offset as usize];

        let delta = sample_b - sample_a;
        sample_a + delta * self.scaled_warp
    }
}
