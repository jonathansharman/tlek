# Tlek

A flexible tool to help build lexicons for
[conlangs](https://en.wikipedia.org/wiki/Constructed_language). It takes a
[probabilistic context-free
grammar](https://en.wikipedia.org/wiki/Probabilistic_context-free_grammar) and
churns out random words.

Tlek is written in [Rust](https://www.rust-lang.org/) and uses [Rusty Object
Notation](https://github.com/ron-rs/ron). If you have Rust and Cargo installed,
you can build and run it with `cargo run <path-to-.ron-file>`.

For example, passing the following file would generate words like `baa`,
`baabaa`, etc.

```rust
(
	// Indicates that after a word has been produced, consecutive graphemes
	// should be preserved rather than collapsed.
	collapse_duplicates: false,
	// Start with the string "SheepNoise" and make weighted random substitutions
	// until none of the symbols in the word have associated production rules.
	start: ["SheepNoise"],
	// Production rules map symbols to substitutions along with their
	// probabilistic weights.
	rules: {
		"SheepNoise": {
			// Replace "SheepNoise" with "baa" 3/4 of the time...
			["baa"]: 3,
			// ...and with "baa SheepNoise" 1/4 of the time.
			["baa", "SheepNoise"]: 1,
		},
	},
)
```

See the [examples](/examples) directory for more example grammars.
