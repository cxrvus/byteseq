pub mod playback;

use crate::config::{Config, WaveConfig};

pub struct SongPlayer {
	// song: Song, // todo: implement
	config: Config,
}

pub struct NotePlayer {
	// todo: implement
	config: Config,
}

impl NotePlayer {
	pub fn tick(self, value: u8) {}
}

pub struct NoteConverter {
	prev_notes: Vec<u8>,
	current_notes: Vec<u8>,
}

impl NoteConverter {
	pub fn convert(config: Config, value: u8) -> Note {
		// todo: remove duplicate values
		todo!()
	}
}

pub struct Note {
	sound: Sound,
	msg: Message,
}

pub enum Message {
	// TODO: better datatype for pitch / frequency
	On(f32),
	Off,
	Nop,
}

pub enum Sound {
	Wave(WaveConfig),
	Sample { id: u8 },
}
