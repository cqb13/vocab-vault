use std::{char, vec};

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

pub fn is_all_numbers(word: String) -> bool {
    word.chars().all(char::is_numeric)
}

pub fn contains_number(word: String) -> bool {
    word.chars().any(char::is_numeric)
}

pub fn remove_all_numbers(word: String) -> String {
    word.chars().filter(|c| !c.is_numeric()).collect()
}

pub fn contains_non_alphanumeric(word: String) -> bool {
    word.chars().any(|c| !c.is_alphanumeric())
}

pub fn remove_non_alphanumeric(word: String) -> String {
    word.chars().filter(|c| c.is_alphanumeric()).collect()
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

pub fn translate_number_to_roman_numeral(number: usize) -> String {
    let roman_numeral = match number {
        1 => "I",
        5 => "V",
        10 => "X",
        50 => "L",
        100 => "C",
        500 => "D",
        1000 => "M",
        _ => panic!("Invalid number: {}", number),
    };

    roman_numeral.to_string()
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

pub fn convert_number_to_roman_numeral(number: &str) -> String {
    let full_numeral = evaluate_full_numeral_from_number(number);
    let proper_numeral = simplify_full_numeral_to_proper_numeral(full_numeral);
    proper_numeral
}

fn simplify_full_numeral_to_proper_numeral(numeral: String) -> String {
    let mut new_numeral = String::new();

    let numeral_counts = [
        numeral.matches("M").count(),
        numeral.matches("C").count(),
        numeral.matches("X").count(),
        numeral.matches("I").count(),
    ];

    let numerals = ["M", "C", "X", "I"];
    let fives = ["", "D", "L", "V"];
    let tens = ["", "M", "C", "X"];

    for i in 0..4 {
        let count = numeral_counts[i];
        let numeral = numerals[i];
        let five = fives[i];
        let ten = tens[i];

        match count {
            1..=3 => new_numeral.push_str(&numeral.repeat(count)),
            4 => new_numeral.push_str(&format!("{}{}", numeral, five)),
            5 => new_numeral.push_str(five),
            6..=8 => new_numeral.push_str(&format!("{}{}", five, numeral.repeat(count - 5))),
            9 => new_numeral.push_str(&format!("{}{}", numeral, ten)),
            _ => (),
        }
    }

    new_numeral
}

fn evaluate_full_numeral_from_number(number: &str) -> String {
    let array_of_nums = split_number_by_places(number);
    let mut roman_numeral = String::new();

    for num in array_of_nums.iter() {
        let first_digit = num.to_string().chars().next().unwrap();
        let places = num.to_string().len();
        let iterations = first_digit.to_string().parse::<usize>().unwrap();
        let mut base = 1;

        let basic_number = 10u32.pow(places as u32);
        let basic_number = basic_number / 10;

        while base <= iterations {
            roman_numeral.push_str(
                translate_number_to_roman_numeral(
                    basic_number.to_string().parse::<usize>().unwrap(),
                )
                .as_str(),
            );
            base += 1;
        }
    }

    roman_numeral
}

fn split_number_by_places(number: &str) -> Vec<u32> {
    let split_number = number.split("").collect::<Vec<&str>>();
    // removes the empty string at the beginning and end
    let split_number = &split_number[1..split_number.len() - 1];

    let mut array_of_true_digits = Vec::new();

    for (index, digit) in split_number.iter().enumerate() {
        let digit = digit.parse::<u32>().unwrap();
        let place = split_number.len() - index - 1;
        let place = 10u32.pow(place as u32);

        let true_digit = digit * place;
        array_of_true_digits.push(true_digit);
    }

    array_of_true_digits
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
