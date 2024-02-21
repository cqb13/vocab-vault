use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::Inflection;
use crate::use_data::utils::word_fits_filters;
use crate::utils::data::get_latin_inflections;
use rand::Rng;

pub fn parse_latin_inflections(
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> Vec<Inflection> {
    let latin_inflections = get_latin_inflections();
    let mut inflection_list: Vec<Inflection> = Vec::new();

    if let Some(amount) = amount {
        if random {
            let mut rng = rand::thread_rng();
            while inflection_list.len() as i32 != amount {
                let random_index = rng.gen_range(0..latin_inflections.len());
                let inflection_at_index = latin_inflections[random_index].clone();
                if !word_fits_filters(
                    &inflection_at_index.ending,
                    &inflection_at_index.pos,
                    &pos_list,
                    &max,
                    &min,
                    &exact,
                ) {
                    continue;
                }
                inflection_list.push(inflection_at_index);
            }
        } else {
            for inflection in latin_inflections {
                if !word_fits_filters(
                    &inflection.ending,
                    &inflection.pos,
                    &pos_list,
                    &max,
                    &min,
                    &exact,
                ) {
                    continue;
                }

                inflection_list.push(inflection);
                if inflection_list.len() as i32 == amount {
                    break;
                }
            }
        }
    } else {
        for inflection in latin_inflections {
            if !word_fits_filters(
                &inflection.ending,
                &inflection.pos,
                &pos_list,
                &max,
                &min,
                &exact,
            ) {
                continue;
            }

            inflection_list.push(inflection);
        }
    }

    inflection_list
}
