use std::error::Error;
extern crate clap;
use clap::*;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_delimiter = ',')]
    say: Option<Vec<String>>,
    #[arg(short, long)]
    weather: bool,
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let default_word: Vec<String> = vec!["no freaking way".to_string()];
    let say_art: String = "     \\  /\\\n      \\/  \\\n      /*-* \\\n      ------\n".to_string();

    let words: Vec<String> = if args.say.is_some() {
        args.say
            .unwrap_or(default_word)
            .iter()
            .flat_map(|s| s.split('\n').map(String::from).collect::<Vec<_>>())
            .collect()
    } else if args.weather {
        let response = reqwest::blocking::get("https://wttr.in/?format=\"%C %t in %l\"")?.text()?;
        response
            .lines()
            .map(String::from)
            .collect()
    } else {
        default_word.clone()
    };

    let len = words
        .iter()
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    let bars = std::iter::repeat('-').take(len + 2).collect::<String>();

    println!(" {bars}");
    for word in &words {
        println!("< {:len$} >", word);
    }
    println!(" {bars}");
    println!("{say_art}");

    Ok(())
}
