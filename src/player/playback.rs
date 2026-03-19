use crate::config::WaveType;
use crate::player::Message;
use crate::player::Note;
use crate::player::Sound;
use anyhow::Result;
use rodio::MixerDeviceSink;
use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, Player};
use std::time::Duration;

pub trait SoundPlayer {
	fn tick(&mut self, note: Note);
}

pub struct NativeSoundPlayer {
	sink: MixerDeviceSink,
}

impl NativeSoundPlayer {
	pub fn new() -> Result<Self> {
		let sink = DeviceSinkBuilder::open_default_sink()?;
		Ok(Self { sink })
	}
}

impl SoundPlayer for NativeSoundPlayer {
	fn tick(&mut self, note: Note) {
		let Note { sound, msg } = note;

		let frequency_hz = match msg {
			Message::On(frequency_hz) => frequency_hz,
			_ => return,
		};

		let player = Player::connect_new(self.sink.mixer());

		match sound {
			Sound::Wave(wave_config) => {
				let source = match wave_config.wave_type {
					WaveType::Sine => SineWave::new(frequency_hz),
					_ => todo!(),
				}
				.take_duration(Duration::from_millis(100))
				.amplify(0.20);

				player.append(source);
			}
			Sound::Sample { id: _ } => todo!("sample playback"),
		}
	}
}
