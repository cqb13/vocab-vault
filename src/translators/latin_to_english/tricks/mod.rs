pub mod trick_lists;
pub mod word_mods;

use crate::translators::latin_to_english::tricks::trick_lists::{
    get_any_tricks, match_slur_trick_list, match_tricks_list, Trick,
};
use crate::translators::latin_to_english::tricks::word_mods::{flip, flip_flop, internal};

pub enum Operation {
    FlipFlop,
    Flip,
    Internal,
}

pub enum TrickResult {
    Found(String, Vec<String>),
    NotFound,
}

impl TrickResult {
    pub fn is_found(&self) -> bool {
        match self {
            TrickResult::Found(_, _) => true,
            TrickResult::NotFound => false,
        }
    }

    pub fn get_word(&self) -> String {
        match self {
            TrickResult::Found(word, _) => word.to_string(),
            TrickResult::NotFound => String::new(),
        }
    }

    pub fn get_explanations(&self) -> Vec<String> {
        match self {
            TrickResult::Found(_, explanations) => explanations.to_vec(),
            TrickResult::NotFound => Vec::new(),
        }
    }
}

pub fn try_tricks(word: &str) -> TrickResult {
    let trick_chars = [
        'a', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 's', 't', 'u', 'y',
        'z',
    ];
    let slur_trick_chars = ['a', 'c', 'i', 'n', 'o', 'q', 's'];
    let first_char = word.chars().next().unwrap();

    if trick_chars.contains(&first_char) {
        let trick_list = match_tricks_list(first_char);
        let (new_word, explanations) = iterate_over_tricks(&trick_list, word);
        if new_word != word {
            return TrickResult::Found(new_word, explanations);
        }
    } else {
        let any_tricks = get_any_tricks();
        let (new_word, explanations) = iterate_over_tricks(&any_tricks, word);

        if new_word == word && slur_trick_chars.contains(&first_char) {
            let trick_list = match_slur_trick_list(first_char);
            let (updated_new_word, updated_explanations) = iterate_over_tricks(&trick_list, word);

            if updated_new_word != word {
                return TrickResult::Found(updated_new_word, updated_explanations);
            }
        } else {
            if new_word != word {
                return TrickResult::Found(new_word, explanations);
            }
        }
    }

    TrickResult::NotFound
}

pub fn try_syncopes(word: &str) -> TrickResult {
    let mut new_word = word.to_string();
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

    if new_word != word {
        return TrickResult::Found(new_word.to_string(), vec![explanation]);
    }

    TrickResult::NotFound
}

fn iterate_over_tricks(trick_list: &Vec<Trick>, word: &str) -> (String, Vec<String>) {
    // word should be modified after each operation is applied.
    let mut explanations = Vec::new();
    let mut modified_word = word.to_string();

    for trick in trick_list.iter() {
        let (word, new_explanation) = match trick.operation {
            Operation::FlipFlop => flip_flop(trick.str_1, trick.str_2, &word),
            Operation::Flip => flip(trick.str_1, trick.str_2, &word),
            Operation::Internal => internal(trick.str_1, trick.str_2, &word),
        };

        if word != modified_word {
            modified_word = word;
        }

        if !new_explanation.is_empty() {
            explanations.push(new_explanation);
        }
    }

    (modified_word, explanations)
}
