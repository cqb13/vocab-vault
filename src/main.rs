use clap::{App, Arg};
use serde_json;

mod english_to_latin;

fn main() {
    let matches = App::new("Translator CLI")
        .version("0.1.0")
        .author("cqb13")
        .about("A simple CLI for translation")
        .subcommand(
            App::new("transEng")
                .about("Translate Latin to English")
                .arg(
                    Arg::with_name("text")
                        .help("The Latin text to translate to English")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("transLat")
                .about("Translate English to Latin")
                .arg(
                    Arg::with_name("text")
                        .help("The English text to translate to Latin")
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("transEng") {
        if let Some(text) = matches.value_of("text") {
            translate_to_latin(text);
        }
    } else if let Some(matches) = matches.subcommand_matches("transLat") {
        if let Some(text) = matches.value_of("text") {
            translate_to_english(text);
        }
    } else {
        println!("Please provide a valid command: transEng or transLat");
    }
}

fn translate_to_english(latin_text: &str) {
    // Replace this with your Latin to English translation logic
    println!(
        "English Translation of '{}': This is just a placeholder.",
        latin_text
    );
}

fn translate_to_latin(english_text: &str) {
    let output = english_to_latin::translate_to_latin(english_text);
    let json_output = serde_json::to_string_pretty(&output).unwrap();
    println!("{}", json_output);
}
