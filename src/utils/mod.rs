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
    let mut word = word.trim().to_lowercase();

    // allows for translation of numbers to roman numerals
    if contains_number(&word) && !is_all_numbers(&word) {
        word = remove_all_numbers(&word);
    }

    if contains_non_alphanumeric(&word) {
        word = remove_non_alphanumeric(&word);
    }

    word
}

pub fn is_all_numbers(word: &str) -> bool {
    word.chars().all(char::is_numeric)
}

pub fn contains_number(word: &str) -> bool {
    word.chars().any(char::is_numeric)
}

pub fn remove_all_numbers(word: &str) -> String {
    word.chars().filter(|c| !c.is_numeric()).collect()
}

pub fn contains_non_alphanumeric(word: &str) -> bool {
    word.chars().any(|c| !c.is_alphanumeric())
}

pub fn remove_non_alphanumeric(word: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_word_lowercases() {
        assert_eq!(sanitize_word("AMOR"), "amor");
    }

    #[test]
    fn test_sanitize_word_trims() {
        assert_eq!(sanitize_word("  amor  "), "amor");
    }

    #[test]
    fn test_sanitize_word_removes_punctuation() {
        assert_eq!(sanitize_word("amor!"), "amor");
        assert_eq!(sanitize_word("amor?"), "amor");
        assert_eq!(sanitize_word("amor."), "amor");
    }

    #[test]
    fn test_sanitize_word_removes_numbers_mixed() {
        assert_eq!(sanitize_word("amor1"), "amor");
    }

    #[test]
    fn test_sanitize_word_preserves_all_numbers() {
        assert_eq!(sanitize_word("123"), "123");
    }

    #[test]
    fn test_number_with_ending() {
        assert_eq!(number_with_ending(1), "1st");
        assert_eq!(number_with_ending(2), "2nd");
        assert_eq!(number_with_ending(3), "3rd");
        assert_eq!(number_with_ending(4), "4th");
        assert_eq!(number_with_ending(11), "11th");
        assert_eq!(number_with_ending(12), "12th");
        assert_eq!(number_with_ending(13), "13th");
        assert_eq!(number_with_ending(21), "21st");
        assert_eq!(number_with_ending(22), "22nd");
        assert_eq!(number_with_ending(23), "23rd");
    }

    #[test]
    fn test_is_all_numbers() {
        assert!(is_all_numbers("123"));
        assert!(!is_all_numbers("abc"));
        assert!(!is_all_numbers("a1b2"));
    }

    #[test]
    fn test_evaluate_roman_numeral() {
        assert_eq!(evaluate_roman_numeral("I").unwrap(), 1);
        assert_eq!(evaluate_roman_numeral("IV").unwrap(), 4);
        assert_eq!(evaluate_roman_numeral("IX").unwrap(), 9);
        assert_eq!(evaluate_roman_numeral("XLII").unwrap(), 42);
        assert_eq!(evaluate_roman_numeral("XC").unwrap(), 90);
        assert_eq!(evaluate_roman_numeral("CD").unwrap(), 400);
        assert_eq!(evaluate_roman_numeral("MCMXCVIII").unwrap(), 1998);
    }

    #[test]
    fn test_is_roman_number() {
        assert!(is_roman_number("XIV"));
        assert!(is_roman_number("MCMXCVIII"));
        assert!(!is_roman_number("ABC"));
    }

    #[test]
    fn test_is_vowel() {
        assert!(is_vowel('a'));
        assert!(is_vowel('e'));
        assert!(is_vowel('i'));
        assert!(is_vowel('o'));
        assert!(is_vowel('u'));
        assert!(!is_vowel('b'));
        assert!(!is_vowel('y'));
    }

    #[test]
    fn test_contains_number() {
        assert!(contains_number("abc123"));
        assert!(!contains_number("abc"));
    }

    #[test]
    fn test_remove_non_alphanumeric() {
        assert_eq!(remove_non_alphanumeric("hello!world?"), "helloworld");
        assert_eq!(remove_non_alphanumeric("no punctuation"), "nopunctuation");
        assert_eq!(remove_non_alphanumeric("keep123"), "keep123");
    }
}
