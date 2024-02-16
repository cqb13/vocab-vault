pub mod cli;
pub mod dictionary_structures;
pub mod translators;
pub mod use_data;
pub mod utils;

use cli::{Arg, Cli, Command, Property};
use dictionary_structures::dictionary_keys::PartOfSpeech;
use translators::english_to_latin::translate_english_to_latin;
use translators::latin_to_english::translate_latin_to_english;
use translators::{DisplayType, Language, Translation, TranslationType};
use use_data::WordType;
use utils::sanitize_word;

use crate::cli::ArgValue;

fn main() {
    let global_args_for_translation = vec![
        Arg::new()
            .with_name("words")
            .with_value_name("WORDS")
            .with_help("The words to translate"),
        Arg::new()
            .with_name("max")
            .with_short('m')
            .with_long("max")
            .with_value_name("MAX")
            .default("6")
            .with_help("The maximum number of translations per definition"),
        Arg::new()
            .with_name("sort")
            .with_short('s')
            .with_long("sort")
            .with_help("Sort the output by word frequency"),
        Arg::new()
            .with_name("pretty")
            .with_short('p')
            .with_long("pretty")
            .with_help("Prints the output in a pretty format"),
        Arg::new()
            .with_name("detailed")
            .with_short('d')
            .with_long("detailed")
            .with_help("Adds more information to the pretty output")
            .requires("pretty"),
    ];

    let cli = Cli::new(
        Property::Auto,
        Property::Auto,
        Property::Auto,
        Property::Auto,
        Property::Auto,
        vec![
            Command::new("transEng", "Translate english to latin")
                .with_args(&global_args_for_translation),
            Command::new("transLat", "Translate latin to english")
                .with_args(&global_args_for_translation)
                .with_arg(
                    Arg::new()
                        .with_name("tricks")
                        .with_short('t')
                        .with_long("tricks")
                        .with_help("Will attempt to use various tricks to find the translation"),
                ),
            Command::new("getList", "Gets a list of words based on the options provided")
                .with_arg(
                    Arg::new()
                    .with_name("type")
                    .with_value_name("TYPE")
                    .with_help("The type of words to get. Options: english, latin, inflections, not-packons, packons, prefixes, stems, suffixes, tackons, tickons, unique-latin"),
                )
                .with_arg(
                    Arg::new()
                    .with_name("pos")
                    .with_short('p')
                    .with_long("pos")
                    .with_value_name("POS")
                    .with_help("The part of speeches to include, separated by commas"),
                )
                .with_arg(
                    Arg::new()
                    .with_name("max")
                    .with_short('m')
                    .with_long("max")
                    .with_value_name("MAX")
                    .with_help("The maximum word length"),
                )
                .with_arg(
                    Arg::new()
                    .with_name("min")
                    .with_short('n')
                    .with_long("min")
                    .with_value_name("MIN")
                    .with_help("The minimum word length"),
                )
                .with_arg(
                    Arg::new()
                    .with_name("exact")
                    .with_short('e')
                    .with_long("exact")
                    .with_value_name("EXACT")
                    .with_help("The exact word length"),
                )
                .with_arg(
                    Arg::new()
                    .with_name("amount")
                    .with_short('a')
                    .with_long("amount")
                    .with_value_name("AMOUNT")
                    .with_help("The amount of words to get"),
                )
                .with_arg(
                    Arg::new()
                    .with_name("random")
                    .with_short('r')
                    .with_long("random")
                    .with_value_name("RANDOM")
                    .with_help("Get words from a random position")
                    .requires("amount"),
                )
                .with_arg(
                    Arg::new()
                    .with_name("to")
                    .with_short('t')
                    .with_long("to")
                    .with_value_name("TO")
                    .with_help("The file to export the results to"),
                ),
            Command::new("help", "Helps you"),
        ],
    );

    let command = cli.match_commands();

    match command.name {
        "transEng" => {
            let words = command.get_value().throw_if_none();
            let max = command
                .get_value_of("max")
                .throw_if_none()
                .parse::<usize>()
                .unwrap();
            let sort = command.has("sort");
            let pretty = command.has("pretty");
            let detailed = command.has("detailed");

            english_to_latin(&words, max, sort, pretty, detailed);
        }
        "transLat" => {
            let words = command.get_value().throw_if_none();
            let max = command
                .get_value_of("max")
                .throw_if_none()
                .parse::<usize>()
                .unwrap();
            let sort = command.has("sort");
            let pretty = command.has("pretty");
            let detailed = command.has("detailed");
            let tricks = command.has("tricks");

            latin_to_english(&words, max, tricks, sort, pretty, detailed);
        }
        "getList" => {
            let type_of_words = command.get_value().throw_if_none();
            let pos = command.get_value_of("pos");
            let max = command.get_value_of("max");
            let min = command.get_value_of("min");
            let exact = command.get_value_of("exact");
            let amount = command.get_value_of("amount");
            let random = command.has("random");
            let to = command.get_value_of("to");

            if type_of_words != "english"
                && type_of_words != "latin"
                && type_of_words != "inflections"
                && type_of_words != "not-packons"
                && type_of_words != "packons"
                && type_of_words != "prefixes"
                && type_of_words != "stems"
                && type_of_words != "suffixes"
                && type_of_words != "tackons"
                && type_of_words != "tickons"
                && type_of_words != "unique-latin"
            {
                println!(
                    "Invalid type of words. Please use `help` to see the available types of words."
                );
                return;
            }

            let word_type = WordType::from_str(type_of_words.as_str()).unwrap_or_else(|e| {
                println!("{}", e);
                std::process::exit(0);
            });

            let pos_list = match pos {
                ArgValue::Present(pos) => {
                    let pos_list: Vec<PartOfSpeech> = pos
                        .split(",")
                        .map(|pos| PartOfSpeech::dict_key_to_part_of_speech(pos))
                        .collect();
                    Some(pos_list)
                }
                ArgValue::Missing(_) => None,
            };

            if pos_list.is_some() && pos_list.as_ref().unwrap().contains(&PartOfSpeech::Unknown) {
                println!("Invalid part of speech entered.");
                println!("Please use the following: noun, verb, participle, adjective, preposition, pronoun, interjection, numeral, conjunction, adverb, number, supine, packon, tackon, prefix, suffix");
                std::process::exit(0);
            }

            println!(
                "type: {:?}, pos: {:?}, max: {:?}, min: {:?}, exact: {:?}, amount: {:?}, random: {:?}, to: {:?}",
                word_type, pos_list, max, min, exact, amount, random, to
            );
        }
        "help" => {
            cli.help();
        }
        _ => {
            println!("Invalid command. Please use `help` to see the available commands.");
        }
    }
}

fn latin_to_english(
    latin_text: &str,
    max: usize,
    tricks: bool,
    sort: bool,
    pretty_output: bool,
    detailed_pretty_output: bool,
) {
    let latin_words: Vec<&str> = latin_text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    for word in latin_words {
        let mut definitions = translate_latin_to_english(&sanitize_word(word), tricks);
        definitions.truncate(max);
        let mut translation =
            Translation::new(word.to_string(), TranslationType::Latin(definitions));

        translation.post_process(Language::Latin, sort);
        translations.push(translation);
    }

    if pretty_output {
        for translation in translations {
            translation.display(DisplayType::Pretty(detailed_pretty_output));
        }
    } else {
        println!("{}", serde_json::to_string_pretty(&translations).unwrap());
    }
}

fn english_to_latin(
    english_text: &str,
    max: usize,
    sort: bool,
    pretty_output: bool,
    detailed_pretty_output: bool,
) {
    let english_words: Vec<&str> = english_text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    for word in english_words {
        let definitions = translate_english_to_latin(&sanitize_word(word), max, sort);
        let mut translation =
            Translation::new(word.to_string(), TranslationType::English(definitions));
        translation.post_process(Language::English, sort);
        translations.push(translation);
    }

    if pretty_output {
        for translation in translations {
            translation.display(DisplayType::Pretty(detailed_pretty_output));
        }
    } else {
        println!("{}", serde_json::to_string_pretty(&translations).unwrap());
    }
}
