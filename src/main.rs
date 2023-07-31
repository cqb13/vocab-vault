use clap::{App, Arg};
use latin_to_english::LatinTranslationInfo;
use serde::{Deserialize, Serialize, Serializer};
use serde_json;
use utils::data::EnglishWordInfo;

mod english_to_latin;
mod latin_to_english;
pub mod utils {
    pub mod data;
    pub mod formatter;
    pub mod key_translator;
    pub mod principle_part_generator;
    pub mod tricks;
}

use utils::formatter::{format_output, sanitize_word};

#[derive(Serialize, Deserialize, Debug)]
pub struct Translation {
    word: String,
    #[serde(serialize_with = "serialize_translation")]
    pub definitions: TranslationType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TranslationType {
    #[serde(rename = "Latin")]
    Latin(Vec<LatinTranslationInfo>),
    #[serde(rename = "English")]
    English(Vec<EnglishWordInfo>),
}

pub enum Language {
    Latin,
    English,
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Language::Latin, Language::Latin) => true,
            (Language::English, Language::English) => true,
            _ => false,
        }
    }
}

fn serialize_translation<S>(def: &TranslationType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match def {
        TranslationType::Latin(info) => info.serialize(serializer),
        TranslationType::English(info) => info.serialize(serializer),
    }
}

fn main() {
    let matches = App::new("Translator CLI")
        .version("0.1.0")
        .author("cqb13")
        .about("A CLI for interacting with the Whitaker's Words Dictionary")
        .subcommand(
            App::new("transEng")
                .about("Translate English to Latin")
                .arg(
                    Arg::with_name("text")
                        .help("The Latin text to translate to English")
                        .required(true),
                )
                .arg(
                    Arg::with_name("formatted")
                        .short('f')
                        .long("formatted")
                        .help("Determines if the output should be formatted")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("clean")
                        .short('c')
                        .long("clean")
                        .help("Cleans the output, by removing objects with vague values, such as 'unknown'.")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("predict macrons")
                        .short('m')
                        .long("predict macrons")
                        .help("Tries to predict where macrons should be in the word")
                        .takes_value(false),
                ),
        )
        .subcommand(
            App::new("transLat")
                .about("Translate Latin to English")
                .arg(
                    Arg::with_name("text")
                        .help("The English text to translate to Latin")
                        .required(true),
                )
                .arg(
                    Arg::with_name("formatted")
                        .short('f')
                        .long("formatted")
                        .help("Determines if the output should be formatted")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("clean")
                        .short('c')
                        .long("clean")
                        .help("Cleans the output, by removing objects with vague values, such as 'unknown'.")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("predict macrons")
                        .short('m')
                        .long("predict macrons")
                        .help("Tries to predict where macrons should be in the word")
                        .takes_value(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("transEng", trans_eng_matches)) => {
            let text = trans_eng_matches.value_of("text").unwrap();
            let formatted_output = trans_eng_matches.is_present("formatted");
            let clean = trans_eng_matches.is_present("clean");
            translate_to_latin(text, formatted_output, clean);
        }
        Some(("transLat", trans_lat_matches)) => {
            let text = trans_lat_matches.value_of("text").unwrap();
            let formatted_output = trans_lat_matches.is_present("formatted");
            let clean = trans_lat_matches.is_present("clean");
            translate_to_english(text, formatted_output, clean);
        }
        _ => println!("Please provide a valid command: transEng or transLat"),
    }
}

fn translate_to_english(latin_text: &str, formatted_output: bool, clean: bool) {
    let latin_words: Vec<&str> = latin_text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    for latin_word in latin_words {
        let output = latin_to_english::translate_to_english(&sanitize_word(latin_word));
        if output.len() > 0 {
            translations.push(Translation {
                word: latin_word.to_string(),
                definitions: TranslationType::Latin(output),
            });
        }
    }

    if formatted_output {
        translations = format_output(translations, Language::Latin, clean);
    }

    let json_output = serde_json::to_string_pretty(&translations).unwrap();
    println!("{}", json_output);
}

fn translate_to_latin(english_text: &str, formatted_output: bool, clean: bool) {
    let english_words: Vec<&str> = english_text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    for word in english_words {
        let output = english_to_latin::translate_to_latin(&sanitize_word(word));
        if output.len() > 0 {
            translations.push(Translation {
                word: word.to_string(),
                definitions: TranslationType::English(output),
            });
        }
    }

    if formatted_output {
        translations = format_output(translations, Language::English, clean);
    }
    let json_output = serde_json::to_string_pretty(&translations).unwrap();
    println!("{}", json_output);
}
