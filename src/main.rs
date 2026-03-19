use clap::{self, Parser};
use std::path::PathBuf;

fn main() {
	println!("Hello, world!");
	// TODO: parse args
}

#[derive(Parser, Debug, Default, Clone)]
#[command(version, about, long_about = None)]
struct Args {
	pub config: Option<PathBuf>,
	pub song: PathBuf,
}
