mod utils;

use self::utils::{remove_duplicates, weigh_words};
use crate::dictionary_structures::dictionary_keys::{
    Age, Area, Frequency, Geography, PartOfSpeech, Source,
};
use crate::dictionary_structures::dictionary_values::{
    EnglishWordInfo, Form, LatinWordInfo, LongForm,
};
use crate::utils::data::{get_english_dictionary, get_latin_dictionary};
use crate::utils::{convert_number_to_roman_numeral, is_all_numbers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct EnglishTranslationInfo {
    pub word: EnglishWordInfo,
    pub translation: LatinWordInfo,
}

impl EnglishTranslationInfo {
    pub fn new(word: EnglishWordInfo, translation: LatinWordInfo) -> EnglishTranslationInfo {
        EnglishTranslationInfo { word, translation }
    }
}

pub fn translate_english_to_latin(
    english_word: &str,
    max: usize,
    sort: bool,
) -> Vec<EnglishTranslationInfo> {
    let mut output: Vec<EnglishTranslationInfo> = Vec::new();

    if is_all_numbers(english_word) {
        match convert_number_to_roman_numeral(english_word) {
            Ok(roman_numeral) => {
                let mut translation =
                    EnglishTranslationInfo::new(EnglishWordInfo::new(), LatinWordInfo::new());

                translation.word.set_orth(english_word);
                translation.word.set_pos(PartOfSpeech::Numeral);
                translation.word.set_frequency_type(Frequency::Common);

                translation.translation.set_orth(&roman_numeral);
                translation.translation.set_senses(vec![format!(
                    "{}, Roman numeral for {}",
                    roman_numeral, english_word
                )]);

                translation.translation.set_pos(PartOfSpeech::Numeral);
                translation.translation.set_form(Form::LongForm(
                    LongForm::new().set_part_of_speech(PartOfSpeech::Numeral),
                ));
                translation
                    .translation
                    .info
                    .set_age(Age::UsedThroughoutAges);
                translation.translation.info.set_area(Area::Technical);
                translation.translation.info.set_freq(Frequency::Common);
                translation.translation.info.set_geo(Geography::AllOrNone);
                translation.translation.info.set_source(Source::General);

                return vec![translation];
            }
            Err(e) => {
                println!("Error converting number to roman numeral: {}", e);
                return output;
            }
        }
    }

    let english_words = get_english_dictionary();
    let latin_words = get_latin_dictionary();

    let latin_words_map: HashMap<i32, &LatinWordInfo> =
        latin_words.iter().map(|word| (word.id, word)).collect();

    for word in english_words {
        if word.orth.to_lowercase() == english_word.to_lowercase() {
            let mut translation =
                EnglishTranslationInfo::new(EnglishWordInfo::new(), LatinWordInfo::new());

            if let Some(latin_word) = latin_words_map.get(&word.wid) {
                translation.translation.set_word(latin_word);
            }

            translation.word.set_word(word);
            output.push(translation);
        }
    }

    output = remove_duplicates(output);

    if sort {
        output = weigh_words(output);
    }

    // other words are probably rare or wrong (default 6)
    if output.len() > max {
        output.truncate(max);
    }

    output
}
