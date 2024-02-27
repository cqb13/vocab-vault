// based on: https://github.com/mk270/whitakers-words/blob/9b11477e53f4adfb17d6f6aa563669dc71e0a680/src/support_utils/support_utils-dictionary_form.adb
mod generate_for_adjectives;
mod generate_for_nouns;
mod generate_for_numerals;
mod generate_for_pronouns;
mod generate_for_verbs;

use self::generate_for_adjectives::generate_for_adjectives;
use self::generate_for_nouns::generate_for_nouns;
use self::generate_for_numerals::generate_for_numerals;
use self::generate_for_pronouns::generate_for_pronouns;
use self::generate_for_verbs::generate_for_verbs;
use crate::dictionary_structures::dictionary_keys::{Comparison, Gender, Numeral, Verb};

pub enum Generator {
    Noun,
    Pronoun,
    Adjective,
    Verb,
    Numeral,
}

// fn takes in Generator, and required args for all generators, then Options for values specific to each generator
pub fn generate_principle_parts(
    generator: Generator,
    num_type_1: i8,
    num_type_2: i8,
    parts: Vec<String>,
    gender: Option<Gender>,
    comparison: Option<Comparison>,
    verb_type: Option<Verb>,
    numeral_type: Option<Numeral>,
) -> Vec<String> {
    match generator {
        Generator::Noun => {
            if gender.is_none() {
                println!("A gender is required for generating principle parts for a noun, but none was provided");
                std::process::exit(0);
            }

            generate_for_nouns(num_type_1, num_type_2, gender.unwrap(), parts)
        }
        Generator::Adjective => {
            if comparison.is_none() {
                println!("A comparison is required for generating principle parts for an adjective, but none was provided");
                std::process::exit(0);
            }
            generate_for_adjectives(num_type_1, num_type_2, parts, comparison.unwrap())
        }
        Generator::Verb => {
            if verb_type.is_none() {
                println!("A verb type is required for generating principle parts for a verb, but none was provided");
                std::process::exit(0);
            }
            generate_for_verbs(num_type_1, num_type_2, parts, verb_type.unwrap())
        }
        Generator::Numeral => {
            if numeral_type.is_none() {
                println!("A numeral type is required for generating principle parts for a numeral, but none was provided");
                std::process::exit(0);
            }
            generate_for_numerals(num_type_1, num_type_2, parts, numeral_type.unwrap())
        }
        Generator::Pronoun => generate_for_pronouns(num_type_1, num_type_2, parts),
    }
}

pub fn set_principle_parts(
    parts: Vec<String>,
    endings: Vec<(&str, i8)>,
    special_case: Option<&str>,
) -> Vec<String> {
    let mut principle_parts = Vec::new();

    if endings.iter().all(|x| x.0 == "" && x.1 == 0) {
        if special_case.is_none() {
            println!("No Endings or Special Case provided");
            std::process::exit(0);
        }
        return vec![parts[0].clone() + " | " + special_case.unwrap_or("")];
    }

    // number in ending is referring to principle part number to add ending to
    for ending in endings {
        let ending_to_add_to_part = ending.0;
        let part_to_add_ending_to = ending.1;

        if ending_to_add_to_part == "" && part_to_add_ending_to == 0 {
            principle_parts.push("---".to_string());
            continue;
        }

        if ending_to_add_to_part != "" && part_to_add_ending_to == 0 {
            principle_parts.push(ending_to_add_to_part.to_string());
            continue;
        }

        let mut part = parts[part_to_add_ending_to as usize - 1].to_string();

        if part == "zzz" {
            principle_parts.push("---".to_string());
            continue;
        }

        part.push_str(ending_to_add_to_part);
        principle_parts.push(part);
    }
    principle_parts
}
