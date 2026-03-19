use std::path::PathBuf;

// todo: add serde
// todo: add documentation

type Bank<T> = [Option<T>; 16];

#[derive(Debug, Clone)]
pub struct NoteConfig {
	samples: Vec<SampleConfig>,
	channels: Bank<ChannelConfig>,
	// todo: implement
	// scales: Vec<ScaleConfig>,
}

#[derive(Debug, Clone)]
pub struct SampleConfig {
	id: String,
	mode: Option<TriggerMode>,
	path: PathBuf,
	/// optional duration in ms
	duration: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ChannelConfig {
	name: Option<String>,
	polyphony: bool,
	voice: VoiceMode,
}

#[derive(Debug, Clone)]
pub enum VoiceMode {
	Bank(SampleBank),
	// todo: add Pitched with sample ID and Key
	// idea: add Velocity with sample ID
}

#[derive(Debug, Clone)]
pub struct SampleBank {
	default_mode: Option<TriggerMode>,
	samples: Bank<SampleConfig>,
}

#[derive(Debug, Clone, Default)]
pub enum TriggerMode {
	#[default]
	Retrigger,
	Gate,
	OneShot,
	Loop,
	GatedLoop,
}

// todo: implement
// #[derive(Debug, Clone, Default)]
// pub struct Key {
// 	/// root note, 0 meaning middle C
// 	root: i8,
// 	/// ID of the scale, defined in Config
// 	scale: String,
// }

// todo: implement
// #[derive(Debug, Clone, Default)]
// pub struct ScaleConfig {
// 	id: String,
// 	intervals: Vec<u8>,
// }

// todo: implement default scales:
// pub const SCALES: [Scale; 3]
