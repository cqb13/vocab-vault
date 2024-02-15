pub mod parser;
pub mod translator;
pub mod tricks;
pub mod utils;

use self::tricks::{try_syncopes, try_tricks};
use crate::dictionary_structures::dictionary_keys::{
    Age, Area, Frequency, Geography, PartOfSpeech, Source,
};
use crate::dictionary_structures::dictionary_values::{
    Form, Inflection, LatinWordInfo, LongForm, Stem,
};
use crate::utils::{evaluate_roman_numeral, is_roman_number};
use serde::{Deserialize, Serialize};

use self::parser::parse;
use self::utils::split_enclitic;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LatinTranslationInfo {
    pub tricks: Option<Vec<String>>,
    pub word: LatinWordInfo,
    pub stem: Stem,
    pub inflections: Option<Vec<Inflection>>,
}

impl LatinTranslationInfo {
    pub fn new() -> LatinTranslationInfo {
        LatinTranslationInfo {
            tricks: None,
            word: LatinWordInfo::new(),
            stem: Stem::new(),
            inflections: None,
        }
    }

    pub fn remove_inflections_with_wrong_pos(&mut self) {
        if self.inflections.is_some() {
            let mut new_inflections = Vec::new();
            for inflection in self.inflections.as_ref().unwrap() {
                if inflection.pos == self.word.pos {
                    new_inflections.push(inflection.clone());
                }
            }
            self.inflections = Some(new_inflections);
        }
    }

    pub fn set_word(&mut self, word: &LatinWordInfo) {
        self.word = word.clone();
    }

    pub fn set_stem(&mut self, stem: &Stem) {
        self.stem = stem.clone();
    }

    pub fn set_inflections(&mut self, inflections: Vec<Inflection>) {
        self.inflections = Some(inflections);
    }

    pub fn set_tricks(&mut self, tricks: &Vec<String>) {
        self.tricks = Some(tricks.to_vec());
    }
}

pub fn translate_latin_to_english(latin_word: &str, tricks: bool) -> Vec<LatinTranslationInfo> {
    if is_roman_number(&latin_word) {
        match evaluate_roman_numeral(&latin_word) {
            Ok(number) => {
                if number > 0 {
                    let mut translation = LatinTranslationInfo::new();
                    translation.word.set_orth(&latin_word);
                    translation
                        .word
                        .set_senses(vec![format!("Number for the Roman Numeral {}", number)]);
                    translation.word.set_pos(PartOfSpeech::Numeral);
                    translation.word.set_form(Form::LongForm(
                        LongForm::new().set_part_of_speech(PartOfSpeech::Numeral),
                    ));
                    translation.word.info.set_age(Age::UsedThroughoutAges);
                    translation.word.info.set_area(Area::Technical);
                    translation.word.info.set_freq(Frequency::Common);
                    translation.word.info.set_geo(Geography::AllOrNone);
                    translation.word.info.set_source(Source::General);
                    return vec![translation];
                }
            }
            Err(e) => {
                println!("Error evaluating roman numeral: {}", e);
                return Vec::new();
            }
        }
    }

    let mut output = parse(&latin_word, false);

    if tricks {
        let trick_results = try_tricks(&latin_word);

        let mut modified_word = if trick_results.is_found() {
            trick_results.get_word()
        } else {
            latin_word.to_string()
        };
        let mut explanations = trick_results.get_explanations();

        let syncope_results = try_syncopes(&modified_word);

        if syncope_results.get_word() != modified_word && syncope_results.is_found() {
            modified_word = syncope_results.get_word();
            explanations.extend(syncope_results.get_explanations());
        }

        if modified_word != latin_word && modified_word != String::new() {
            let mut new_output = parse(&modified_word, false);

            if new_output.is_some() {
                for word in new_output.as_mut().unwrap() {
                    word.set_tricks(&explanations);
                }

                if output.is_some() {
                    output.as_mut().unwrap().extend(new_output.unwrap());
                } else {
                    output = new_output;
                }
            }
        }
    }

    // most words should be found by now

    // If nothing is found, try removing enclitics and try again
    // ex: clamaverunt -> clamare
    // doing this here instead of earlier should fix words like salve having the "ve" removed and returning wrong def
    if output.is_none() {
        let (word_without_ecliptics, modifiers) = split_enclitic(&latin_word);
        output = parse(&word_without_ecliptics, false);

        if output.is_some() {
            for word in output.as_mut().unwrap() {
                word.word.set_modifiers(modifiers.clone());
            }
        }
    }

    output.unwrap_or(Vec::new())
}
