use std::io::Write;

use vocab_vault::cli::{Arg, ArgValue, Cli, Command};
use vocab_vault::dictionary_structures::dictionary_keys::PartOfSpeech;
use vocab_vault::translators::english_to_latin::translate_english_to_latin;
use vocab_vault::translators::latin_to_english::translate_latin_to_english;
use vocab_vault::translators::{DisplayType, Language, Translation, TranslationType};
use vocab_vault::use_data::{get_list, WordType};
use vocab_vault::utils::data::{get_english_dictionary, get_latin_dictionary};
use vocab_vault::utils::sanitize_word;

//TODO: add a command for searching a word by id in english or latin dictionary
//TODO: display the amount of time it took for a command to execute
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

    let cli = Cli::new().with_default_command("tui").with_commands(vec![
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
                .with_help("The type of words to get. Options: english, latin, inflections, not_packons, packons, prefixes, stems, suffixes, tackons, tickons, unique_latin"),
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
                .with_help("Get words from a random position")
                .requires("amount"),
            )
            .with_arg(
                Arg::new()
                .with_name("display")
                .with_short('d')
                .with_long("display")
                .with_help("Will display as json"),
            )
            .with_arg(
                Arg::new()
                .with_name("to")
                .with_short('t')
                .with_long("to")
                .with_value_name("TO")
                .with_help("The file to export the results to"),
            ),
        Command::new("help", "Helps you")
            .with_arg(
                Arg::new()
                .with_name("command")
                .with_value_name("COMMAND")
                .with_help("A command to help with"),
            ),
        Command::new("tui", "Starts the tui (.help for info)"),
    ]);

    let command = cli.match_commands();

    match command.name {
        "transEng" => {
            let words = command.get_value().throw_if_none();
            let max = command
                .get_value_of("max")
                .throw_if_none()
                .parse::<usize>()
                .expect("--max must be a valid number");
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
                .expect("--max must be a valid number");
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
            let display = command.has("display");
            let to = command.get_value_of("to");

            if !WordType::is_valid_word_type(&type_of_words) {
                println!(
                    "Invalid type of words. Please use `help` to see the available types of words."
                );
                return;
            }

            let word_type = match WordType::from_str(&type_of_words) {
                Ok(t) => t,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            let pos_list = match pos {
                ArgValue::Present(pos) => {
                    let pos_list: Vec<PartOfSpeech> = pos
                        .split(",")
                        .map(|pos| PartOfSpeech::dict_key_to_part_of_speech(pos.trim()))
                        .collect();
                    Some(pos_list)
                }
                ArgValue::Missing(_) => None,
            };

            if pos_list
                .as_ref()
                .is_some_and(|list| list.contains(&PartOfSpeech::Unknown))
            {
                println!("Invalid part of speech entered.");
                println!("Please use the following: noun, verb, participle, adjective, preposition, pronoun, interjection, numeral, conjunction, adverb, number, supine, packon, tackon, prefix, suffix");
                std::process::exit(0);
            }

            let parse_usize = |s: String| s.parse::<usize>().ok().map(|v| v as i32);

            let max = match max {
                ArgValue::Present(max) => parse_usize(max),
                ArgValue::Missing(_) => None,
            };

            let min = match min {
                ArgValue::Present(min) => parse_usize(min),
                ArgValue::Missing(_) => None,
            };

            let exact = match exact {
                ArgValue::Present(exact) => parse_usize(exact),
                ArgValue::Missing(_) => None,
            };

            let amount = match amount {
                ArgValue::Present(amount) => parse_usize(amount),
                ArgValue::Missing(_) => None,
            };

            let to = match to {
                ArgValue::Present(to) => Some(to),
                ArgValue::Missing(_) => None,
            };

            get_list(
                word_type, pos_list, max, min, exact, amount, random, display, to,
            );
        }
        "help" => {
            let command = command.get_value().to_option();
            cli.help(command);
        }
        "tui" => {
            let mut input = String::new();
            let mut language = Language::Latin;
            loop {
                print!("> ");
                input.clear();
                let _ = std::io::stdout().flush();
                if std::io::stdin().read_line(&mut input).is_err() {
                    break;
                }
                let input = input.trim();

                match input {
                    ".exit" | ".quit" | "q" => {
                        break;
                    }
                    ".help" => {
                        println!("Commands:");
                        println!(".help - Displays this message");
                        println!(".exit - Exits the program");
                        println!(".switch - Switches between latin and english");
                        println!("enter a word to translate it")
                    }
                    ".switch" => {
                        language = match language {
                            Language::Latin => Language::English,
                            Language::English => Language::Latin,
                        };
                        println!("Switched to {:?}", language.as_str());
                    }
                    ".clear" => {
                        print!("\x1B[2J\x1B[1;1H");
                    }
                    _ => match language {
                        Language::Latin => {
                            latin_to_english(input, 6, true, true, true, false);
                        }
                        Language::English => {
                            english_to_latin(input, 6, true, true, true);
                        }
                    },
                }
            }
        }
        _ => {
            println!("Invalid command. Please use `help` to see the available commands.");
        }
    }
}

//TODO: get dictionaries here, to not repeat getting them for each word
fn latin_to_english(
    latin_text: &str,
    max: usize,
    tricks: bool,
    sort: bool,
    pretty_output: bool,
    detailed_pretty_output: bool,
) {
    let mut translations: Vec<Translation> = Vec::new();

    for word in latin_text.split_whitespace() {
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
        match serde_json::to_string_pretty(&translations) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Failed to serialize translations: {}", e),
        }
    }
}

fn english_to_latin(
    english_text: &str,
    max: usize,
    sort: bool,
    pretty_output: bool,
    detailed_pretty_output: bool,
) {
    let latin_dictionary = get_latin_dictionary();
    let english_dictionary = get_english_dictionary();
    let mut translations: Vec<Translation> = Vec::new();

    for word in english_text.split_whitespace() {
        let definitions = translate_english_to_latin(
            english_dictionary,
            latin_dictionary,
            &sanitize_word(word),
            max,
            sort,
        );
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
        match serde_json::to_string_pretty(&translations) {
            Ok(json) => println!("{}", json),
            Err(e) => eprintln!("Failed to serialize translations: {}", e),
        }
    }
}
