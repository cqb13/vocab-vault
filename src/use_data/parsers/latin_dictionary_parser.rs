use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::LatinWordInfo;
use crate::utils::data::get_latin_dictionary;

pub fn parse_latin_dictionary(
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> Vec<LatinWordInfo> {
    let latin_dictionary = get_latin_dictionary();
    let mut latin_word_info_list: Vec<LatinWordInfo> = Vec::new();

    for word in latin_dictionary {
        if let Some(pos_list) = &pos_list {
            if !pos_list.contains(&word.pos) {
                continue;
            }
        }

        if let Some(max) = max {
            if word.orth.len() > max as usize {
                continue;
            }
        }

        if let Some(min) = min {
            if word.orth.len() < min as usize {
                continue;
            }
        }

        if let Some(exact) = exact {
            if word.orth.len() != exact as usize {
                continue;
            }
        }

        latin_word_info_list.push(word);
    }

    if let Some(amount) = amount {
        latin_word_info_list.truncate(amount as usize);
    }

    latin_word_info_list
}
