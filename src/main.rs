use arboard::Clipboard;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about = "Copy data from STDIN to the clipboard.")]
#[command(version, long_about = None)]
struct Args {
    string: Option<String>,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    
    let the_string = clipboard.get_text().unwrap_or_else(|_| {
        println!("Clipboard is empty");
        String::new()
    });

    let args = Args::parse();
    let the_string = &args.string.unwrap_or(the_string);

    for _ in 0..args.count  {
        println!("Hello, {}!", the_string);
        clipboard.set_text(the_string.to_owned()).unwrap();
    }
}
