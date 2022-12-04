use anyhow::{anyhow, Result};
use serde::Deserialize;

use std::{fs::File, io::BufReader};

#[derive(Deserialize, Debug)]
struct Class {
	pub name: String,
	pub disparity: f32,
	pub phonemes: Vec<String>,
}

#[derive(Deserialize, Debug)]
enum State {
	Start,
	Class(String),
	End,
}

#[derive(Deserialize, Debug)]
struct Phonotactics {
	classes: Vec<Class>,
	transitions: Vec<(State, Vec<(f32, State)>)>,
}

fn main() -> Result<()> {
	let file = File::open("phonotactics.ron")?;
	let reader = BufReader::new(file);
	let phonotactics: Phonotactics = ron::de::from_reader(reader)
		.map_err(|err| anyhow!("Failed to parse phonotactics file: {err}"))?;
	println!("{phonotactics:?}");
	Ok(())
}
