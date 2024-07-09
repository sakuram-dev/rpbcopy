use arboard::Clipboard;
use clap::Parser;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(about = "Copy data from STDIN to the clipboard.")]
#[command(version, long_about = None)]
struct Args {}

fn main() {
    let _args = Args::parse();
    let mut clipboard = Clipboard::new().unwrap();
    let mut input_data = String::new();

    io::stdin()
        .read_to_string(&mut input_data)
        .expect("Failed to read from STDIN");

    println!("Copy to clipboard \n{}", input_data);
    clipboard
        .set_text(input_data.clone())
        .expect("Failed to copy to clipboard");
}
