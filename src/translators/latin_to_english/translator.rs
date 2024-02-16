use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::{Inflection, LatinWordInfo, Part, Stem};
use crate::translators::latin_to_english::utils::add_stem_to_word;
use crate::translators::latin_to_english::LatinTranslationInfo;
use crate::utils::data::get_latin_dictionary;
use std::collections::HashMap;

pub fn lookup_stems(
    stems: Vec<Stem>,
    inflections: Vec<Inflection>,
) -> Option<Vec<LatinTranslationInfo>> {
    let latin_dictionary = get_latin_dictionary();
    let mut output: Vec<LatinTranslationInfo> = Vec::new();

    let latin_words_map: HashMap<i32, &LatinWordInfo> = latin_dictionary
        .iter()
        .map(|word| (word.id, word))
        .collect();

    for stem in stems {
        let dict_word = latin_words_map.get(&stem.wid);

        if let Some(latin_word) = dict_word {
            let word_is_in_output = output
                .iter()
                .any(|x| x.word.id == latin_word.id || x.word.orth == latin_word.orth);

            // if the word is already in the output add the stem to it
            if word_is_in_output {
                let matching_word = output
                    .iter_mut()
                    .find(|x| x.word.id == latin_word.id || x.word.orth == latin_word.orth);

                if let Some(word) = matching_word {
                    add_stem_to_word(stem, Some(word));
                }
            } else {
                let new_inflections: Vec<Inflection>;

                if latin_word.pos == PartOfSpeech::Verb
                    || latin_word.pos == PartOfSpeech::Participle
                {
                    let fourth_part = latin_word.get_part(Part::Fourth);
                    if fourth_part.is_some() && fourth_part.unwrap() != stem.orth {
                        new_inflections = inflections
                            .iter()
                            .filter(|inflection| inflection.pos != PartOfSpeech::Participle)
                            .cloned()
                            .collect();
                    } else {
                        new_inflections = inflections
                            .iter()
                            .filter(|inflection| inflection.pos != PartOfSpeech::Verb)
                            .cloned()
                            .collect();
                    }
                } else {
                    new_inflections = inflections.clone();
                }

                let mut new_word = LatinTranslationInfo::new();

                let next_word = latin_words_map.get(&(latin_word.id + 1));

                if next_word.is_some() {
                    let next_senses = next_word.unwrap().senses.clone();

                    // senses starting with | also apply to word before
                    if next_senses.len() > 0 && next_senses[0].starts_with('|') {
                        new_word.word.set_extension_senses(next_senses);
                    }
                }

                new_word.word.set_word(&latin_word);
                new_word.stem = stem;
                new_word.inflections = Some(new_inflections);

                output.push(new_word);
            }
        }
    }

    if output.len() > 0 {
        Some(output)
    } else {
        None
    }
}
