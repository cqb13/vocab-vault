use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::LatinWordInfo;
use crate::use_data::utils::word_fits_filters;
use rand::Rng;

// need to generate principal parts before checking if the word fits the filter, to account for length filters
pub fn parse_latin_dictionary(
    dictionary: Vec<LatinWordInfo>,
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> Vec<LatinWordInfo> {
    let mut latin_word_info_list: Vec<LatinWordInfo> = Vec::new();

    if let Some(amount) = amount {
        if random {
            let mut rng = rand::thread_rng();
            while latin_word_info_list.len() as i32 != amount {
                let random_index = rng.gen_range(0..dictionary.len());
                let mut word_at_index = dictionary[random_index].clone();
                word_at_index.generate_principle_parts();
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
            for mut word in dictionary {
                word.generate_principle_parts();
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
        for mut word in dictionary {
            word.generate_principle_parts();
            if !word_fits_filters(&word.orth, &word.pos, &pos_list, &max, &min, &exact) {
                continue;
            }

            latin_word_info_list.push(word);
        }
    }

    latin_word_info_list
}
