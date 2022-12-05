use anyhow::{anyhow, Result};
use serde::Deserialize;

use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

#[derive(Clone, Eq, PartialEq, Hash, Deserialize, Debug)]
pub enum StateID {
	Start,
	State(String),
	End,
}

#[derive(Deserialize, Debug)]
pub struct StateConfig {
	pub emit: Option<PhonemeClassID>,
	pub transition_weights: HashMap<StateID, f32>,
}

pub type PhonemeClassID = String;

pub type Phoneme = String;

#[derive(Deserialize, Debug)]
pub struct Config {
	pub classes: HashMap<PhonemeClassID, HashMap<Phoneme, f32>>,
	pub states: HashMap<StateID, StateConfig>,
}

impl Config {
	pub fn load<P>(path: P) -> Result<Self>
	where
		P: AsRef<Path>,
	{
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		ron::de::from_reader(reader)
			.map_err(|err| anyhow!("Failed to parse config file: {err}"))
	}
}
