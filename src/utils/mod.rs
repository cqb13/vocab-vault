pub mod data;
pub mod principle_part_generator;
pub mod type_translator;

/**
 * Returns the number with the appropriate ending
 * Ex: 1 -> 1st, 2 -> 2nd, 3 -> 3rd, 4 -> 4th, 5 -> 5th, 6 -> 6th, 7 -> 7th, 8 -> 8th, 9 -> 9th, 10 -> 10th
 */
pub fn number_with_ending(number: i8) -> String {
    let last_digit = number % 10;
    let last_two_digits = number % 100;
    if last_two_digits >= 11 && last_two_digits <= 13 {
        return format!("{}th", number);
    }
    match last_digit {
        1 => format!("{}st", number),
        2 => format!("{}nd", number),
        3 => format!("{}rd", number),
        _ => format!("{}th", number),
    }
}

/**
 * Removes all non-alphanumeric characters from a string
 */
pub fn sanitize_word(word: &str) -> String {
    let mut word = word.to_owned();
    word = word.trim().to_lowercase();

    // allows for translation of numbers to roman numerals
    if contains_number(word.clone()) && !is_all_numbers(&word) {
        word = remove_all_numbers(word.clone());
    }

    if contains_non_alphanumeric(word.clone()) {
        word = remove_non_alphanumeric(word.clone());
    }

    word
}

pub fn is_all_numbers(word: &str) -> bool {
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

pub fn translate_roman_digit_to_number(c: char) -> Result<i32, String> {
    match c.to_ascii_uppercase() {
        'I' => Ok(1),
        'V' => Ok(5),
        'X' => Ok(10),
        'L' => Ok(50),
        'C' => Ok(100),
        'D' => Ok(500),
        'M' => Ok(1000),
        _ => return Err(format!("{} is an invalid roman numeral digit", c)),
    }
}

pub fn translate_number_to_roman_numeral(number: usize) -> Result<String, String> {
    let roman_numeral = match number {
        1 => "I",
        5 => "V",
        10 => "X",
        50 => "L",
        100 => "C",
        500 => "D",
        1000 => "M",
        _ => return Err(format!("{} is an invalid number", number)),
    };

    Ok(roman_numeral.to_string())
}

pub fn evaluate_roman_numeral(roman_numeral: &str) -> Result<i32, String> {
    let mut result = 0;
    let mut last_digit = 0;
    for c in roman_numeral.chars().rev() {
        let digit = translate_roman_digit_to_number(c)?;
        if digit < last_digit {
            result -= digit;
        } else {
            result += digit;
        }
        last_digit = digit;
    }
    Ok(result)
}

pub fn convert_number_to_roman_numeral(number: &str) -> Result<String, String> {
    let full_numeral = evaluate_full_numeral_from_number(number)?;
    let proper_numeral = simplify_full_numeral_to_proper_numeral(full_numeral);
    Ok(proper_numeral)
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

fn evaluate_full_numeral_from_number(number: &str) -> Result<String, String> {
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
                )?
                .as_str(),
            );
            base += 1;
        }
    }

    Ok(roman_numeral)
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
