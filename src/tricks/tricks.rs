use std::char;

use crate::tricks::trick_list::Trick;
use crate::tricks::trick_list::{get_any_tricks, match_slur_trick_list, match_tricks_list};
use crate::tricks::word_mods::{flip, flip_flop, internal};

#[derive(Debug, Clone)]
pub enum Operation {
    FlipFlop,
    Flip,
    Internal,
    Slur,
}

pub fn is_vowel(c: char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

pub fn is_roman_digit(c: char) -> bool {
    match c.to_ascii_uppercase() {
        'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M' => true,
        _ => false,
    }
}

pub fn is_roman_number(possible_roman_number: &str) -> bool {
    possible_roman_number.chars().all(is_roman_digit)
}

pub fn is_common_prefix(prefix: String) -> bool {
    let constant_prefixes = [
        "dis", "ex", "in", "per", "prae", "pro", "re", "si", "sub", "super", "trans",
    ];

    constant_prefixes.contains(&prefix.as_str())
}

pub fn translate_roman_digit_to_number(c: char) -> u32 {
    match c.to_ascii_uppercase() {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => panic!("Invalid roman digit: {}", c),
    }
}

pub fn evaluate_roman_numeral(roman_numeral: &str) -> u32 {
    let mut result = 0;
    let mut last_digit = 0;
    for c in roman_numeral.chars().rev() {
        let digit = translate_roman_digit_to_number(c);
        if digit < last_digit {
            result -= digit;
        } else {
            result += digit;
        }
        last_digit = digit;
    }
    result
}

pub fn try_tricks(word: String) -> (String, Option<Vec<String>>) {
    let trick_chars = [
        'a', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 's', 't', 'u', 'y',
        'z',
    ];
    let slur_trick_chars = ['a', 'c', 'i', 'n', 'o', 'q', 's'];
    let mut applied_tricks: Option<Vec<String>> = None;
    let mut new_word = word.clone();
    let first_char = word.chars().next().unwrap();
    let mut explanations = vec![];

    if trick_chars.contains(&first_char) {
        let trick_list = match_tricks_list(first_char);
        let (updated_new_word, updated_explanations) =
            iterate_over_tricks(trick_list.clone(), new_word.to_string());
        new_word = updated_new_word;
        explanations.extend(updated_explanations);

        applied_tricks = Some(explanations.clone());
    }

    let any_tricks = get_any_tricks();

    if applied_tricks.is_none() {
        applied_tricks = Some(explanations.clone());
    }

    let (updated_new_word, updated_explanations) =
        iterate_over_tricks(any_tricks.clone(), new_word.to_string());
    new_word = updated_new_word;

    applied_tricks
        .as_mut()
        .unwrap()
        .extend(updated_explanations);

    if new_word == word && slur_trick_chars.contains(&first_char) {
        let trick_list = match_slur_trick_list(first_char);
        let (updated_new_word, updated_explanations) =
            iterate_over_tricks(trick_list.clone(), new_word.to_string());
        new_word = updated_new_word;
        applied_tricks
            .as_mut()
            .unwrap_or(&mut explanations)
            .extend(updated_explanations);
    }

    if new_word == word {
        (word, None)
    } else {
        (new_word, applied_tricks)
    }
}

fn iterate_over_tricks(trick_list: Vec<Trick>, mut word: String) -> (String, Vec<String>) {
    // word should be modified after each operation is applied.
    let mut explanations = Vec::new();

    for trick in trick_list.iter() {
        let (new_word, new_explanation) = match trick.operation {
            Operation::FlipFlop => flip_flop(trick.str_1, trick.str_2, &word),
            Operation::Flip => flip(trick.str_1, trick.str_2, &word),
            Operation::Internal => internal(trick.str_1, trick.str_2, &word),
            Operation::Slur => (word.clone(), String::new()), // Assuming Slur causes an exception
        };

        word = new_word;

        if !new_explanation.is_empty() {
            explanations.push(new_explanation);
        }
    }

    (word, explanations)
}
