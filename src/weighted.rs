use anyhow::Result;
use rand::prelude::*;
use rand_distr::WeightedAliasIndex;

pub struct WeightedList<T> {
	elements: Vec<T>,
	dist: WeightedAliasIndex<f32>,
}

impl<T> WeightedList<T> {
	pub fn new(element_weights: Vec<(T, f32)>) -> Result<WeightedList<T>> {
		let weights = element_weights.iter().map(|ew| ew.1).collect();
		let elements = element_weights.into_iter().map(|ew| ew.0).collect();
		Ok(WeightedList {
			elements,
			dist: WeightedAliasIndex::new(weights)?,
		})
	}

	pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> &T {
		&self.elements[self.dist.sample(rng)]
	}
}
