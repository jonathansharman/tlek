use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use anyhow::{anyhow, Result};
use rand::prelude::*;
use serde::Deserialize;

use crate::weighted::WeightedList;

#[derive(Deserialize, Debug)]
pub struct Phonotactics {
	collapse_duplicates: bool,
	start: Vec<String>,
	rules: HashMap<String, WeightedList<Vec<String>>>,
}

impl Phonotactics {
	pub fn load<P>(path: P) -> Result<Self>
	where
		P: AsRef<Path>,
	{
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		ron::de::from_reader(reader)
			.map_err(|err| anyhow!("Failed to parse config file: {err}"))
	}

	pub fn gen_word(&self) -> Result<String> {
		let mut rng = thread_rng();
		let mut word = self.start.clone();
		let mut idx = 0;
		while idx < word.len() {
			if let Some(substitutions) = self.rules.get(&word[idx]) {
				let substitution = substitutions.sample(&mut rng);
				word.splice(idx..idx + 1, substitution.clone());
			} else {
				idx += 1;
			}
		}
		if self.collapse_duplicates {
			word.dedup()
		}
		Ok(word.join(""))
	}
}
