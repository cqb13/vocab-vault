use crate::utils::is_vowel;

pub fn flip(str_to_replace: &str, replacement_str: &str, word: &str) -> (String, String) {
    let mut new_word = String::from(word);
    let mut explanation = String::from("");

    if new_word.len() >= str_to_replace.len() + 2
        && new_word.starts_with(str_to_replace)
        && !new_word[str_to_replace.len()..].starts_with(replacement_str)
    {
        new_word.replace_range(0..str_to_replace.len(), replacement_str);

        if new_word.len() >= replacement_str.len() + 2 && new_word.starts_with(replacement_str) {
            let canned_explanation = "' may have replaced usual '";

            explanation = if !canned_explanation.is_empty() {
                format!(
                    "An initial '{}{}{}'",
                    str_to_replace, canned_explanation, replacement_str
                )
            } else {
                String::from("")
            };

            return (new_word, explanation);
        }
    }

    (new_word, explanation)
}

// at the beginning of Input word, replaces str_to_replace by replacement_str - then replacement_str by str_to_replace
// only used when both str_to_replace and replacement_str start with the same letter
pub fn flip_flop(str_to_replace: &str, replacement_str: &str, word: &str) -> (String, String) {
    let explanation = String::from("");

    if word.len() >= str_to_replace.len() + 2 && word.starts_with(str_to_replace) {
        let mut new_word = String::new();
        new_word.push_str(replacement_str);
        new_word.push_str(&word[str_to_replace.len()..]);

        if new_word.len() >= replacement_str.len() + 2 && new_word.starts_with(replacement_str) {
            let explanation = format!(
                "An initial '{}' may be rendered by '{}'",
                str_to_replace, replacement_str
            );

            return (new_word, explanation);
        }
    }

    (word.to_string(), explanation)
}

// replace str_to_replace by replacement_str anywhere in the word, then tests for validity
pub fn internal(str_to_replace: &str, replacement_str: &str, word: &str) -> (String, String) {
    let explanation = String::from("");

    if word.contains(str_to_replace) {
        let new_word = word.replace(str_to_replace, replacement_str);

        if new_word.len() >= replacement_str.len() + 2 {
            let explanation = format!(
                "An internal '{}' may be rendered by '{}'",
                str_to_replace, replacement_str
            );

            return (new_word, explanation);
        }
    }

    (word.to_string(), explanation)
}

pub fn double_consonants(latin_word: &str) -> (String, String) {
    let mut doubled_word = String::new();
    let mut explanation = String::new();

    let split_word: Vec<char> = latin_word.chars().collect();

    for (i, letter) in split_word.iter().enumerate() {
        if is_vowel(letter.to_owned()) {
            doubled_word.push(*letter);
            continue;
        }

        if i == 0 && is_vowel(split_word[i + 1]) {
            doubled_word.push(*letter);
            continue;
        } else if i == split_word.len() - 1 && is_vowel(split_word[i - 1]) {
            doubled_word.push(*letter);
            doubled_word.push(*letter);
            continue;
        } else if is_vowel(split_word[i - 1]) && is_vowel(split_word[i + 1]) {
            doubled_word.push(*letter);
            doubled_word.push(*letter);
            continue;
        } else {
            doubled_word.push(*letter);
        }
    }

    if doubled_word.len() > latin_word.len() {
        explanation = format!("Consonants may be doubled in '{}'", latin_word);
    }

    (doubled_word, explanation)
}
