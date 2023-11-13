mod phonotactics;
mod weighted;

use anyhow::Result;
use clap::Parser;

use crate::phonotactics::Phonotactics;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	#[arg(index = 1)]
	filename: String,
	#[arg(short, long, default_value_t = 1)]
	count: u8,
}

fn main() -> Result<()> {
	let args = Args::parse();
	let phonotactics = Phonotactics::load(args.filename)?;
	for _ in 0..args.count {
		let word = phonotactics.gen_word()?;
		print!("{word} ");
	}
	println!();
	Ok(())
}
