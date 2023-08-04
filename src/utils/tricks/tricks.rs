//TODO: add word mod, if word ends in e, try ae
use crate::utils::tricks::word_mods::{flip, flip_flop};

pub enum Operation {
    FlipFlop,
    Flip,
    Internal,
    Slur,
}

pub struct Trick {
    pub max_attempts: i32,
    pub operation: Operation,
    pub flip_flop1: &'static str,
    pub flip_flop2: &'static str,
    pub flip_flip3: &'static str,
    pub flip_flip4: &'static str,
    pub internal1: &'static str,
    pub internal2: &'static str,
    pub slur1: &'static str,
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

pub fn iterate_over_tricks(trick_list: &[Trick], max_attempts: i32, word: &str) -> bool {
    let mut is_finished = false;

    for trick in trick_list.iter() {
        match trick.operation {
            Operation::FlipFlop => flip_flop(trick.flip_flop1, trick.flip_flop2, word),
            Operation::Flip => flip(trick.flip_flip3, trick.flip_flip4, word),
            Operation::Internal => return true, //internal(trick.internal1, trick.internal2),
            Operation::Slur => return true, // Assuming Slur causes an exception
        };

        if max_attempts > trick.max_attempts {
            is_finished = true;
            return is_finished;
        }
    }

    is_finished
}

pub fn is_common_prefix(prefix: String) -> bool {
    let constant_prefixes = [
        "dis", "ex", "in", "per", "prae", "pro", "re", "si", "sub", "super", "trans",
    ];

    constant_prefixes.contains(&prefix.as_str())
}
