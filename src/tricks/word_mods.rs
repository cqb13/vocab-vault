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

pub fn try_syncopes(word: String) -> (String, String) {
    let mut new_word = word;
    let mut explanation = String::new();

    if new_word.len() >= 3 && new_word.ends_with("ivi") {
        new_word.replace_range(new_word.len() - 3..new_word.len(), "ii");
        explanation =
            String::from("Syncopated perfect 'ivi' can drop 'v' without contracting vowel");
    } else if new_word.len() >= 4 && new_word.ends_with("iver") {
        new_word.replace_range(new_word.len() - 4..new_word.len(), "ier");
        explanation =
            String::from("Syncopated perfect 'iver' can drop 'v' without contracting vowel");
    }

    (new_word, explanation)
}
