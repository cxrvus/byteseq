use crate::player::Message;
use crate::player::Note;
use crate::player::Sound;
use anyhow::Result;
use rodio::MixerDeviceSink;
use rodio::play;
use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use std::time::Duration;

pub trait SoundPlayer {
	fn tick(&mut self, note: Note);
}

pub struct NativeSoundPlayer {
	sink: MixerDeviceSink,
	players: Vec<Player>,
}

impl NativeSoundPlayer {
	pub fn new() -> Result<Self> {
		let sink = DeviceSinkBuilder::open_default_sink()?;
		let players = vec![];
		Ok(Self { sink, players })
	}
}

impl SoundPlayer for NativeSoundPlayer {
	fn tick(&mut self, note: Note) {
		let Note { sound, msg } = note;

		let mixer = self.sink.mixer();
	}
}
