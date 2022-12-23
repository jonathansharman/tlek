mod phonotactics;
mod weighted;

use anyhow::{anyhow, Result};

use crate::phonotactics::Phonotactics;

fn main() -> Result<()> {
	let path = std::env::args()
		.nth(1)
		.ok_or_else(|| anyhow!("No config file provided"))?;
	let phonotactics = Phonotactics::load(path)?;
	let word = phonotactics.gen_word()?;
	println!("{word}");
	Ok(())
}
