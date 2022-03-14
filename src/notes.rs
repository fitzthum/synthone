// Everything we need to keep track of notes.

use vst::event::MidiEvent;
use std::collections::HashMap;

// for now just keep track of the notes that are on
// but this is probably going to cause some problems 
// when we try to implement envelope stuff

#[derive(Clone, Copy)]
pub struct Note {
    pub number: u8,
    pub velocity: u8,
    pub time: f32,
}

impl Note {
    pub fn from_midi(e: MidiEvent) -> Note {
       Note {
            number: e.data[1],
            velocity: e.data[2],
            time: 0.0,
       }
    }

    pub fn update_time(&mut self, time: f32) {
        self.time += time;
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

    pub fn add_note(&mut self, e: MidiEvent) {
        self.notes.insert(e.data[1], Note::from_midi(e));
    }

    pub fn remove_note(&mut self, e: MidiEvent) {
        self.notes.remove(&e.data[1]);
    }

    pub fn get_notes(&self) -> Vec<Note> {
        self.notes.values().cloned().collect()
    }

    pub fn update_note_times(&mut self, time: f32) {
        for note in self.notes.values_mut() {
            note.update_time(time);
        }
    }
}

// TODO: make some kind of note container wrapper around a dictionary that will hold multiple notes
// and have some handy traits to add and remove 
