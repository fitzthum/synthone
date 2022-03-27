// Everything we need to keep track of notes.

use std::collections::HashMap;
use vst::event::MidiEvent;

use log::*;

// Note must be public so it can be used in Voice
#[derive(Clone, Copy)]
pub struct Note {
    pub number: u8,
    pub velocity: u8,
    pub time: f32,
    pub off_time: f32,
    pub on: bool,
}

impl Note {
    fn from_midi(e: MidiEvent) -> Note {
        Note {
            number: e.data[1],
            velocity: e.data[2],
            time: 0.0,
            off_time: 0.0,
            on: true,
        }
    }

    fn update_time(&mut self, time: f32) {
        self.time += time;
    }

    fn turn_off(&mut self) {
        self.on = false;
        self.off_time = self.time;
    }
}

// Keeps track of the notes we're supposed to be playing.
pub struct Notebook {
    notes: HashMap<u8, Note>,
}

impl Notebook {
    pub fn new() -> Notebook {
        Notebook {
            notes: HashMap::new(),
        }
    }

    pub fn note_on(&mut self, e: MidiEvent) {
        self.notes.insert(e.data[1], Note::from_midi(e));
    }

    pub fn note_off(&mut self, e: MidiEvent) {
        if let Some(note) = self.notes.get_mut(&e.data[1]) {
            note.turn_off();
        }
    }

    pub fn get_notes(&self) -> Vec<Note> {
        self.notes.values().cloned().collect()
    }

    pub fn update_note_times(&mut self, time: f32) {
        for note in self.notes.values_mut() {
            note.update_time(time);
        }
    }

    // since we don't delete the note on note off events anymore
    // we need to make sure we get rid of notes that have been off
    // long enough that they can't make a sound anymore
    pub fn purge_old_notes(&mut self, threshold: f32) {
        self.notes
            .retain(|_, note| note.on || (note.time - note.off_time) < threshold);
    }
}
