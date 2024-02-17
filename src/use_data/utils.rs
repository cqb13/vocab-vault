use crate::dictionary_structures::dictionary_keys::PartOfSpeech;

pub fn word_fits_filters(
    word_orth: &str,
    word_pos: &PartOfSpeech,
    pos_list: &Option<Vec<PartOfSpeech>>,
    max: &Option<i32>,
    min: &Option<i32>,
    exact: &Option<i32>,
) -> bool {
    if let Some(pos_list) = pos_list {
        if !pos_list.contains(word_pos) {
            return false;
        }
    }

    if let Some(max) = max {
        if word_orth.len() > *max as usize {
            return false;
        }
    }

    if let Some(min) = min {
        if word_orth.len() < *min as usize {
            return false;
        }
    }

    if let Some(exact) = exact {
        if word_orth.len() != *exact as usize {
            return false;
        }
    }

    true
}
