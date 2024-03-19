use clap::Parser;
use nu_ansi_term::Color::{Green, LightCyan, Purple};

fn main() {
    let args = Arguments::parse();
    
    if args.uppercase {
        println!(
            "{} {}",
            LightCyan.paint("Parsing input (uppercase) :"),
            Purple.paint(args.input.to_uppercase())
        );
        println!(
            "{} {}",
            LightCyan.paint("Found charset:"),
            Green.paint(extract_charset(args.input.to_uppercase().as_str()))
        );
    } else if args.lowercase {
        println!(
            "{} {}",
            LightCyan.paint("Parsing input (lowercase) :"),
            Purple.paint(args.input.to_lowercase())
        );
        println!(
            "{} {}",
            LightCyan.paint("Found charset:"),
            Green.paint(extract_charset(args.input.to_lowercase().as_str()))
        );
    } else {
        println!(
            "{} {}",
            LightCyan.paint("Parsing input:"),
            Purple.paint(&args.input)
        );
        println!(
            "{} {}",
            LightCyan.paint("Found charset:"),
            Green.paint(extract_charset(args.input.as_str()))
        );
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Input [`String`] to extract a charset from.
    #[arg(short, long, default_value_t = String::from("1c'<8p4OYH-0&^M|l^=n`KJfw#hu_?1M+8$CHcc7fib|=;m(&`"))]
    input : String,

    /// Whether to convert the input [`String`] to uppercase
    #[arg(short = 'u', long = "uppercase", default_value_t = false, default_missing_value = "true")]
    uppercase : bool,

    /// Whether to convert the input [`String`] to lowercase
    #[arg(short = 'l', long = "lowercase", default_value_t = false, default_missing_value = "true")]
    lowercase : bool,
}

/// Returns a [`String`] of every unique character in the input &str in the order they appear.
/// Also returns the first of every type of whitespace
fn extract_charset(input: &str) -> String {
    let mut charset : String = String::new();

    for character in input.chars() {
        if !charset.contains(character) {
            charset.push(character);
        }
    }

    charset
}