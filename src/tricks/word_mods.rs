pub fn switch_first_i_or_j(word: &str) -> (String, Option<Vec<String>>) {
    let mut chars: Vec<char> = word.chars().collect();
    let mut explanation: Option<Vec<String>> = None;
    if let Some(first_char) = chars.first_mut() {
        if *first_char == 'i' {
            *first_char = 'j';
            explanation = Some(vec![String::from("An initial 'i' may be rendered by 'j'")]);
        } else if *first_char == 'j' {
            *first_char = 'i';
            explanation = Some(vec![String::from("An initial 'j' may be rendered by 'i'")]);
        }
    }

    (chars.into_iter().collect(), explanation)
}

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
                    "An initial '{}'{}{}'",
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

pub fn slur(str_to_slur: &str, word: &str) -> String {
    //let explanation = format!("{} was slurped", str_to_slur);

    let mut new_word = word.to_string();

    new_word
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
