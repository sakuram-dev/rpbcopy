use arboard::Clipboard;
use clap::Parser;
use std::io::{self, Read};

#[derive(Parser, Debug)]
#[command(about = "Copy data from STDIN to the clipboard.")]
#[command(version, long_about = None)]
struct Args {
    #[clap(short, long)]
    #[clap(help = "copy from file instead of STDIN")]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut clipboard = Clipboard::new().unwrap();
    let mut input_data = String::new();

    if let Some(file) = args.file {
        let mut file_data = String::new();
        std::fs::File::open(file)
            .expect("Failed to open file")
            .read_to_string(&mut file_data)
            .expect("Failed to read file");

        println!("Copy to clipboard \n{}", file_data);
        clipboard
            .set_text(file_data.clone())
            .expect("Failed to copy to clipboard");
    } else {
        io::stdin()
            .read_to_string(&mut input_data)
            .expect("Failed to read from STDIN");

        println!("Copy to clipboard \n{}", input_data);
        clipboard
            .set_text(input_data.clone())
            .expect("Failed to copy to clipboard");
    }
}
