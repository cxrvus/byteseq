use std::path::PathBuf;

// todo: add serde
// todo: add documentation

#[derive(Debug, Clone, Default)]
pub struct Config {
	samples: Vec<SampleConfig>,
	channels: Bank<ChannelConfig>,
	scales: Vec<ScaleConfig>,
}

#[derive(Debug, Clone, Default)]
struct Bank<T>([Option<T>; 16]);

#[derive(Debug, Clone, Default)]
pub struct SampleConfig {
	id: String,
	mode: TriggerMode,
	path: PathBuf,
	// optional duration in ms
	duration: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct ChannelConfig {
	name: Option<String>,
	polyphony: bool,
	voice: VoiceMode,
}

#[derive(Debug, Clone, Default)]
pub struct ScaleConfig {
	id: String,
	intervals: Vec<u8>,
}

// todo: implement default scales:
// pub const SCALES: [Scale; 3]

#[derive(Debug, Clone)]
pub enum VoiceMode {
	Wave(WaveConfig),
	Bank(SampleBank),
	// todo: add Pitched(Pitched) with sample ID and Key
}

impl Default for VoiceMode {
	fn default() -> Self {
		Self::Wave(Default::default())
	}
}

#[derive(Debug, Clone, Default)]
pub struct SampleBank {
	default_mode: Option<TriggerMode>,
	samples: Bank<SampleConfig>,
}

#[derive(Debug, Clone, Default)]
pub struct WaveConfig {
	mode: TriggerMode,
	wave_type: WaveType,
	key: Key,
	// duration in ms
	duration: u32,
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

#[derive(Debug, Clone, Default)]
pub enum WaveType {
	#[default]
	Sine,
	Saw,
	Triangle,
	Square,
}

#[derive(Debug, Clone, Default)]
pub struct Key {
	/// root note, 0 meaning middle C
	root: i8,
	/// ID of the scale, defined in Config
	scale: String,
}
