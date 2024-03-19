# Rust Character Set Extractor

Takes a UTF-8 encoded input String and returns all unique characters used in the String, in the order they appear.
Also returns the first of each type of whitespace that appears.

Case-sensitive, optionally convert to either uppercase or lowercase with `--uppercase` or `--lowercase` respectfully.

## Usage
Use with `cargo run` or use `cargo build --release` to build an executable file,
which is located in `/target/release`, which can be run in a terminal with `./rust_charset_extractor`
> [!IMPORTANT]
> By default, this will parse an example String.
> 
> To set a custom input, pass in `-i INPUT` or`--input INPUT` (`-- -i INPUT` or `-- -input INPUT` when using `cargo run`, i.e. `cargo run -- --input INPUT`)
>
> Running with `--help` (`-- --help` when using `cargo run`) opens a help menu with instructions