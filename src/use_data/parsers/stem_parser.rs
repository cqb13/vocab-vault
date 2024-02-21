use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::Stem;
use crate::use_data::utils::word_fits_filters;
use crate::utils::data::get_latin_stems;
use rand::Rng;

pub fn parse_latin_stems(
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> Vec<Stem> {
    let latin_stems = get_latin_stems();
    let mut stem_list: Vec<Stem> = Vec::new();

    if let Some(amount) = amount {
        if random {
            let mut rng = rand::thread_rng();
            while stem_list.len() as i32 != amount {
                let random_index = rng.gen_range(0..latin_stems.len());
                let stem_at_index = latin_stems[random_index].clone();
                if !word_fits_filters(
                    &stem_at_index.orth,
                    &stem_at_index.pos,
                    &pos_list,
                    &max,
                    &min,
                    &exact,
                ) {
                    continue;
                }
                stem_list.push(stem_at_index);
            }
        } else {
            for stem in latin_stems {
                if !word_fits_filters(&stem.orth, &stem.pos, &pos_list, &max, &min, &exact) {
                    continue;
                }

                stem_list.push(stem);
                if stem_list.len() as i32 == amount {
                    break;
                }
            }
        }
    } else {
        for stem in latin_stems {
            if !word_fits_filters(&stem.orth, &stem.pos, &pos_list, &max, &min, &exact) {
                continue;
            }

            stem_list.push(stem);
        }
    }

    stem_list
}
