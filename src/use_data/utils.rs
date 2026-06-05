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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_fits_no_filters() {
        assert!(word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &None,
            &None,
            &None,
            &None
        ));
    }

    #[test]
    fn test_word_fits_pos_filter() {
        assert!(word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &Some(vec![PartOfSpeech::Noun]),
            &None,
            &None,
            &None
        ));
        assert!(!word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &Some(vec![PartOfSpeech::Verb]),
            &None,
            &None,
            &None
        ));
    }

    #[test]
    fn test_word_fits_length_filters() {
        assert!(word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &None,
            &Some(6),
            &None,
            &None
        ));
        assert!(!word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &None,
            &Some(3),
            &None,
            &None
        ));
        assert!(word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &None,
            &None,
            &Some(3),
            &None
        ));
        assert!(!word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &None,
            &None,
            &Some(6),
            &None
        ));
        assert!(word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &None,
            &None,
            &None,
            &Some(4)
        ));
        assert!(!word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &None,
            &None,
            &None,
            &Some(5)
        ));
    }

    #[test]
    fn test_word_fits_combined_filters() {
        assert!(word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &Some(vec![PartOfSpeech::Noun]),
            &Some(6),
            &Some(2),
            &None
        ));
        assert!(!word_fits_filters(
            "amor",
            &PartOfSpeech::Noun,
            &Some(vec![PartOfSpeech::Verb]),
            &Some(6),
            &Some(2),
            &None
        ));
    }
}
