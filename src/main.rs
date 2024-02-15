pub mod cli;
pub mod dictionary_structures;
pub mod translators;
pub mod utils;

use cli::{Arg, Cli, Command, Property};
use translators::english_to_latin::translate_english_to_latin;
use translators::latin_to_english::translate_latin_to_english;
use translators::{DisplayType, Language, Translation, TranslationType};
use utils::sanitize_word;

fn main() {
    let global_args = vec![
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
            Command::new("transEng", "Translate english to latin").with_args(&global_args),
            Command::new("transLat", "Translate latin to english")
                .with_args(&global_args)
                .with_arg(
                    Arg::new()
                        .with_name("tricks")
                        .with_short('t')
                        .with_long("tricks")
                        .with_help("Will attempt to use various tricks to find the translation"),
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
