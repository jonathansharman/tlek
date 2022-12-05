use std::{collections::HashMap, path::Path};

use anyhow::{anyhow, Result};
use rand::prelude::*;

use crate::{
	config::{Config, Phoneme, PhonemeClassID, StateID},
	weighted::WeightedList,
};

struct State {
	emit: Option<PhonemeClassID>,
	transitions: WeightedList<StateID>,
}

pub struct Phonotactics {
	classes: HashMap<PhonemeClassID, WeightedList<Phoneme>>,
	states: HashMap<StateID, State>,
}

impl Phonotactics {
	pub fn new(config: Config) -> Result<Self> {
		let mut classes = HashMap::new();
		for (class_id, phoneme_weights) in config.classes {
			classes.insert(
				class_id,
				WeightedList::new(phoneme_weights.into_iter().collect())?,
			);
		}
		let mut states = HashMap::new();
		for (state_id, state_config) in config.states {
			states.insert(
				state_id,
				State {
					emit: state_config.emit,
					transitions: WeightedList::new(
						state_config.transition_weights.into_iter().collect(),
					)?,
				},
			);
		}
		Ok(Self { classes, states })
	}

	pub fn load<P>(path: P) -> Result<Self>
	where
		P: AsRef<Path>,
	{
		Self::new(Config::load(path)?)
	}

	pub fn gen_word(&self) -> Result<String> {
		let mut word = String::new();
		let mut state_id = StateID::Start;
		let mut state = self.get_state(&state_id)?;
		let mut rng = thread_rng();
		loop {
			// Transition to next state.
			state_id = state.transitions.sample(&mut rng).clone();
			if let StateID::End = state_id {
				return Ok(word);
			}
			state = self.get_state(&state_id)?;
			// Emit a phoneme, if applicable.
			if let Some(class_name) = &state.emit {
				let Some(phonemes) = self.classes.get(class_name) else {
					return Err(anyhow!("Word class '{class_name:?}' not found"));
				};
				// Choose and add phoneme.
				let phoneme = phonemes.sample(&mut rng);
				word += phoneme;
			}
		}
	}

	fn get_state(&self, state_id: &StateID) -> Result<&State> {
		self.states
			.get(state_id)
			.ok_or_else(|| anyhow!("State '{state_id:?}' not found"))
	}
}
