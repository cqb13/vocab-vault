use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::Modifier;
use crate::use_data::utils::word_fits_filters;
use rand::Rng;

pub fn parse_modifiers(
    modifiers: Vec<Modifier>,
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
) -> Vec<Modifier> {
    let mut modifier_list: Vec<Modifier> = Vec::new();

    if let Some(amount) = amount {
        if random {
            let mut rng = rand::thread_rng();
            while modifier_list.len() as i32 != amount {
                let random_index = rng.gen_range(0..modifiers.len());
                let modifier_at_index = modifiers[random_index].clone();
                if !word_fits_filters(
                    &modifier_at_index.orth,
                    &modifier_at_index.pos,
                    &pos_list,
                    &max,
                    &min,
                    &exact,
                ) {
                    continue;
                }
                modifier_list.push(modifier_at_index);
            }
        } else {
            for modifier in modifiers {
                if !word_fits_filters(
                    &modifier.orth,
                    &modifier.pos,
                    &pos_list,
                    &max,
                    &min,
                    &exact,
                ) {
                    continue;
                }

                modifier_list.push(modifier);
                if modifier_list.len() as i32 == amount {
                    break;
                }
            }
        }
    } else {
        for modifier in modifiers {
            if !word_fits_filters(
                &modifier.orth,
                &modifier.pos,
                &pos_list,
                &max,
                &min,
                &exact,
            ) {
                continue;
            }

            modifier_list.push(modifier);
        }
    }

    modifier_list
}
