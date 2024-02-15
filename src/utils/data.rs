use crate::dictionary_structures::dictionary_values::{
    Attachment, EnglishWordInfo, Inflection, LatinWordInfo, Modifier, Stem, UniqueLatinWordInfo,
};
use std::include_bytes;

pub fn get_english_dictionary() -> Vec<EnglishWordInfo> {
    let english_words_json = include_bytes!("../dictionary/english_words.json");
    serde_json::from_slice(english_words_json).unwrap()
}

pub fn get_latin_dictionary() -> Vec<LatinWordInfo> {
    let latin_words_json = include_bytes!("../dictionary/latin_dictionary.json");
    serde_json::from_slice(latin_words_json).unwrap()
}

pub fn get_unique_latin_words() -> Vec<LatinWordInfo> {
    let unique_latin_words_json = include_bytes!("../dictionary/unique_latin_words.json");
    let unique_latin_words: Vec<UniqueLatinWordInfo> =
        serde_json::from_slice(unique_latin_words_json).unwrap();

    unique_latin_words
        .iter()
        .map(|word| {
            let mut latin_word_info = LatinWordInfo::new();
            latin_word_info.orth = word.orth.to_string();
            latin_word_info.senses = word.senses.to_vec();
            latin_word_info.pos = word.pos;
            latin_word_info.form = word.form.clone();
            latin_word_info.info = word.info.clone();
            latin_word_info.n = word.n.clone();
            latin_word_info
        })
        .collect()
}

pub fn get_latin_inflections() -> Vec<Inflection> {
    let latin_inflections_json = include_bytes!("../dictionary/latin_inflections.json");
    serde_json::from_slice(latin_inflections_json).unwrap()
}

pub fn get_latin_stems() -> Vec<Stem> {
    let latin_stems_json = include_bytes!("../dictionary/latin_stems.json");
    serde_json::from_slice(latin_stems_json).unwrap()
}

pub fn get_latin_prefixes() -> Vec<Modifier> {
    let latin_prefixes_json = include_bytes!("../dictionary/latin_prefixes.json");
    serde_json::from_slice(latin_prefixes_json).unwrap()
}

pub fn get_latin_suffixes() -> Vec<Modifier> {
    let latin_suffixes_json = include_bytes!("../dictionary/latin_suffixes.json");
    serde_json::from_slice(latin_suffixes_json).unwrap()
}

pub fn get_latin_packons() -> Vec<Attachment> {
    let latin_packons_json = include_bytes!("../dictionary/latin_packons.json");
    serde_json::from_slice(latin_packons_json).unwrap()
}

pub fn get_latin_not_packons() -> Vec<Attachment> {
    let latin_not_packons_json = include_bytes!("../dictionary/latin_not_packons.json");
    serde_json::from_slice(latin_not_packons_json).unwrap()
}

pub fn get_latin_tackons() -> Vec<Attachment> {
    let latin_tackons_json = include_bytes!("../dictionary/latin_tackons.json");
    serde_json::from_slice(latin_tackons_json).unwrap()
}

pub fn get_latin_tickons() -> Vec<Attachment> {
    let latin_tickons_json = include_bytes!("../dictionary/latin_tickons.json");
    serde_json::from_slice(latin_tickons_json).unwrap()
}
