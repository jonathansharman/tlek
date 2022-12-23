use std::marker::PhantomData;

use anyhow::Result;
use rand::prelude::*;
use rand_distr::WeightedAliasIndex;
use serde::{
	de::{MapAccess, Visitor},
	Deserialize,
};

#[derive(Debug)]
pub struct WeightedList<T> {
	elements: Vec<T>,
	dist: WeightedAliasIndex<f32>,
}

impl<T> WeightedList<T> {
	pub fn new(element_weights: Vec<(T, f32)>) -> Result<WeightedList<T>> {
		let (elements, weights) = element_weights.into_iter().unzip();
		Ok(WeightedList {
			elements,
			dist: WeightedAliasIndex::new(weights)?,
		})
	}

	pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> &T {
		&self.elements[self.dist.sample(rng)]
	}
}

impl<'de, T> Deserialize<'de> for WeightedList<T>
where
	T: serde::Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		deserializer.deserialize_map(WeightedListVisitor {
			phantom_data: PhantomData,
		})
	}
}

struct WeightedListVisitor<T> {
	phantom_data: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for WeightedListVisitor<T>
where
	T: Deserialize<'de>,
{
	type Value = WeightedList<T>;

	fn expecting(
		&self,
		formatter: &mut std::fmt::Formatter,
	) -> std::fmt::Result {
		formatter.write_str("a mapping to numeric weights")
	}

	fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut element_weights = Vec::new();
		while let Some((k, v)) = map.next_entry()? {
			element_weights.push((k, v));
		}
		WeightedList::new(element_weights).map_err(|err| {
			<M::Error as serde::de::Error>::custom(err.to_string())
		})
	}
}
