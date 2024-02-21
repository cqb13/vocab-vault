use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::LatinWordInfo;
use crate::use_data::utils::word_fits_filters;
use crate::utils::data::get_unique_latin_words;
use rand::Rng;

pub fn parse_unique_latin_words(
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> Vec<LatinWordInfo> {
    let unique_latin_words = get_unique_latin_words();
    let mut latin_word_info_list: Vec<LatinWordInfo> = Vec::new();

    if let Some(amount) = amount {
        if random {
            let mut rng = rand::thread_rng();
            while latin_word_info_list.len() as i32 != amount {
                let random_index = rng.gen_range(0..unique_latin_words.len());
                let word_at_index = unique_latin_words[random_index].clone();
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
                latin_word_info_list.push(word_at_index);
            }
        } else {
            for word in unique_latin_words {
                if !word_fits_filters(&word.orth, &word.pos, &pos_list, &max, &min, &exact) {
                    continue;
                }

                latin_word_info_list.push(word);
                if latin_word_info_list.len() as i32 == amount {
                    break;
                }
            }
        }
    } else {
        for word in unique_latin_words {
            if !word_fits_filters(&word.orth, &word.pos, &pos_list, &max, &min, &exact) {
                continue;
            }

            latin_word_info_list.push(word);
        }
    }

    latin_word_info_list
}
