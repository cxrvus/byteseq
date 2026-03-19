pub mod playback;

use anyhow::Result;

use crate::{config::NoteConfig, player::playback::SoundPlayer};

pub struct SongPlayer {
	// song: Song, // todo: implement
	config: NoteConfig,
}

pub struct NotePlayer<P: SoundPlayer> {
	sounds: Vec<P>,
}

impl<P: SoundPlayer> NotePlayer<P> {
	pub fn new(config: NoteConfig) -> Result<Self> {
		todo!()
	}
	pub fn tick(self, value: u8) {}
}

pub struct NoteConverter {
	prev_notes: Vec<u8>,
	current_notes: Vec<u8>,
}

impl NoteConverter {
	pub fn convert(config: NoteConfig, value: u8) -> Note {
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
	Sample { id: u8 },
}
