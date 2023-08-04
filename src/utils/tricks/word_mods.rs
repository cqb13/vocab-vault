pub fn switch_first_i_or_j(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    if let Some(first_char) = chars.first_mut() {
        if *first_char == 'i' {
            *first_char = 'j';
        } else if *first_char == 'j' {
            *first_char = 'i';
        }
    }
    chars.into_iter().collect()
}

// replace the beginning part of a word with another string
pub fn flip(str_to_replace: &str, replacement_str: &str, word: &str) -> String {
    //let explanation = format!("{} was replaced by {}", str_to_replace, replacement_str);

    let mut new_word = word.to_string();
    if let Some(replaced) = new_word.strip_prefix(str_to_replace) {
        new_word = format!("{}{}", replacement_str, replaced);
    }

    new_word
}

// at the beginning of Input word, replaces str_to_replace by replacement_str - then replacement_str by str_to_replace
// only used when both str_to_replace and replacement_str start with the same letter
pub fn flip_flop(str_to_replace: &str, replacement_str: &str, word: &str) -> String {
    //let explanation = format!("{} may be written as {}", str_to_replace, replacement_str);

    let mut new_word = word.to_string();


    new_word
}
