use crate::data::data::{get_english_words, get_latin_dictionary, EnglishWordInfo, LatinWordInfo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EnglishTranslationInfo {
    pub word: EnglishWordInfo,
    pub translation: LatinWordInfo,
}

impl Clone for EnglishTranslationInfo {
    fn clone(&self) -> Self {
        EnglishTranslationInfo {
            word: self.word.clone(),
            translation: self.translation.clone(),
        }
    }
}

pub fn translate_to_latin(english_word: &str, max: usize, sort: bool) -> Vec<EnglishTranslationInfo> {
    let mut output: Vec<EnglishTranslationInfo> = Vec::new();

    let english_words = get_english_words();
    for word in english_words {
        if word.orth == english_word.to_lowercase() {
            let mut english_word_info = EnglishTranslationInfo {
                word: word.clone(),
                translation: LatinWordInfo::new(),
            };
            english_word_info.word.true_frequency = Some(calculate_true_frequency(
                word.frequency,
                word.compound,
                word.semi,
            ));
            output.push(english_word_info);
        }
    }

    if sort {
        output = weigh_words(output);
    }

    output = remove_duplicates(output);

    // other words are probably rare/irrelevant or wrong
    if output.len() > max {
        output.truncate(max);
    }

    find_definition(&mut output);

    output
}

fn calculate_true_frequency(frequency: i16, compound: i16, semi: i16) -> i16 {
    frequency + compound - semi
}

fn weigh_words(word_list: Vec<EnglishTranslationInfo>) -> Vec<EnglishTranslationInfo> {
    let mut weighted_word_list = word_list;
    weighted_word_list.sort_by(|a, b| b.word.true_frequency.cmp(&a.word.true_frequency));
    weighted_word_list
}

fn remove_duplicates(word_list: Vec<EnglishTranslationInfo>) -> Vec<EnglishTranslationInfo> {
    let mut deduped_word_list: Vec<EnglishTranslationInfo> = Vec::new();
    let mut seen_wids: Vec<i32> = Vec::new();

    for word_info in word_list {
        if !seen_wids.contains(&word_info.word.wid) {
            seen_wids.push(word_info.word.wid);
            deduped_word_list.push(word_info);
        }
    }

    deduped_word_list
}

fn find_definition(word_list: &mut Vec<EnglishTranslationInfo>) {
    let latin_dictionary = get_latin_dictionary();

    for word_info in word_list.iter_mut() {
        for latin_word in &latin_dictionary {
            if latin_word.id == word_info.word.wid {
                word_info.translation = latin_word.clone();
            }
        }
    }
}
