mod config;
mod phonotactics;
mod weighted;

use anyhow::{anyhow, Result};

use crate::phonotactics::Phonotactics;

fn main() -> Result<()> {
	let path = std::env::args()
		.nth(1)
		.ok_or_else(|| anyhow!("No config file provide"))?;
	let phonotactics = Phonotactics::load(path)?;
	for _ in 0..10 {
		let word = phonotactics.gen_word()?;
		println!("{word}");
	}
	Ok(())
}
