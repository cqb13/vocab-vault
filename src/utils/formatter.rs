use regex::Regex;

use crate::latin_to_english::Word;
use crate::{Translation, Language};
use crate::utils::key_translator::translate_part_of_speech;
use crate::utils::data::Stem;

use super::data::{LatinWordInfo, WordInfo};
use super::key_translator::{translate_age, translate_area, translate_frequency, translate_source};

pub fn format_output(mut translation_output: Vec<Translation>, language: Language) -> Vec<Translation> {

    for mut translation in &translation_output {
        if language == Language::English {

        } else if language == Language::Latin {
            
        } else {
            panic!("Language not supported");
        }
    }

    translation_output
}

fn format_latin_word_info(latin_word_info: LatinWordInfo) -> LatinWordInfo {
    let mut clean_latin_word_info: LatinWordInfo = latin_word_info;

    clean_latin_word_info.pos = translate_part_of_speech(&clean_latin_word_info.pos[..]).to_string();
    clean_latin_word_info.info = format_word_info(clean_latin_word_info.info);

    clean_latin_word_info
}

fn format_word_info(word_info: WordInfo) -> WordInfo {
    let mut  clean_word_info: WordInfo = word_info;

    clean_word_info.age = translate_age(&clean_word_info.age[..]).to_string();
    clean_word_info.area = translate_area(&clean_word_info.area[..]).to_string();
    clean_word_info.geo = translate_area(&clean_word_info.geo[..]).to_string();
    clean_word_info.freq = translate_frequency(&clean_word_info.freq[..]).to_string();
    clean_word_info.source = translate_source(&clean_word_info.source[..]).to_string();

    clean_word_info
}

fn format_latin_stem(latin_stem: Stem) -> Stem {
    let mut clean_latin_stem: Stem = latin_stem;

    clean_latin_stem.pos = translate_part_of_speech(&clean_latin_stem.pos[..]).to_string();

    clean_latin_stem
}

fn format_latin_inflections() {

}

fn remove_inflections_without_endings() {

}

pub fn sanitize_word(word: &str) -> String {
    let mut word = word.to_owned();
    word = word.trim().to_lowercase();
    let re = Regex::new(r"[^a-z ]|\d|\s+").unwrap();
    word = re.replace_all(&word, " ").to_string();
    word
}