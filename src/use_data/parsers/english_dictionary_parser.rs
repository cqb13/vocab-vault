use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::EnglishWordInfo;
use crate::use_data::utils::word_fits_filters;
use crate::utils::data::get_english_dictionary;
use rand::Rng;

pub fn parse_english_dictionary(
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> Vec<EnglishWordInfo> {
    let english_dictionary = get_english_dictionary();
    let mut english_word_info_list: Vec<EnglishWordInfo> = Vec::new();

    if let Some(amount) = amount {
        if random {
            let mut rng = rand::thread_rng();
            while english_word_info_list.len() as i32 != amount {
                let random_index = rng.gen_range(0..english_dictionary.len());
                let word_at_index = english_dictionary[random_index].clone();
                if !word_fits_filters(
                    &word_at_index.orth,
                    &word_at_index.pos,
                    &pos_list,
                    &max,
                    &min,
                    &exact,
                ) {
                    continue;
                }
                english_word_info_list.push(word_at_index);
            }
        } else {
            for word in english_dictionary {
                if !word_fits_filters(&word.orth, &word.pos, &pos_list, &max, &min, &exact) {
                    continue;
                }

                english_word_info_list.push(word);
                if english_word_info_list.len() as i32 == amount {
                    break;
                }
            }
        }
    } else {
        for word in english_dictionary {
            if !word_fits_filters(&word.orth, &word.pos, &pos_list, &max, &min, &exact) {
                continue;
            }

            english_word_info_list.push(word);
        }
    }

    english_word_info_list
}
