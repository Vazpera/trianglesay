extern crate clap;
use clap::*;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_delimiter = ',')]
    say: Option<Vec<String>>,
}
fn main() {
    let args = Args::parse();
    let arg_word: Option<Vec<String>> = args.say;

    let default_word: Vec<String> = vec!["no freaking way".to_string()];
    let say_art: String = "     \\  /\\\n      \\/  \\\n      /*-* \\\n      ------\n".to_string();

    let words = arg_word
        .unwrap_or(default_word)
        .iter()
        .map(|x| x.split("\\n").map(|x| x.to_string()))
        .flatten()
        .collect::<Vec<String>>();
    let len = words
        .clone()
        .iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .expect("No max length string")
        .len();
    let bars = std::iter::repeat('-').take(len + 2).collect::<String>();

    println!(" {bars}");
    words.iter().for_each(|x| println!("< {x:len$} >"));
    println!(" {bars}");
    println!("{say_art}")
}
