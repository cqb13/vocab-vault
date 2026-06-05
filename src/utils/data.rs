use crate::dictionary_structures::dictionary_values::{
    Attachment, EnglishWordInfo, Inflection, LatinWordInfo, Modifier, Stem, UniqueLatinWordInfo,
};
use std::sync::OnceLock;

macro_rules! cached_json {
    ($path:expr, $cache:expr) => {
        $cache.get_or_init(|| {
            serde_json::from_slice(include_bytes!($path)).expect(concat!("Failed to parse ", $path))
        })
    };
}

pub fn get_english_dictionary() -> &'static Vec<EnglishWordInfo> {
    static CACHE: OnceLock<Vec<EnglishWordInfo>> = OnceLock::new();
    cached_json!("../dictionary/english_words.json", CACHE)
}

pub fn get_latin_dictionary() -> &'static Vec<LatinWordInfo> {
    static CACHE: OnceLock<Vec<LatinWordInfo>> = OnceLock::new();
    cached_json!("../dictionary/latin_dictionary.json", CACHE)
}

pub fn get_unique_latin_words() -> &'static Vec<LatinWordInfo> {
    static CACHE: OnceLock<Vec<LatinWordInfo>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let unique_words: Vec<UniqueLatinWordInfo> =
            serde_json::from_slice(include_bytes!("../dictionary/unique_latin_words.json"))
                .expect("Failed to parse unique_latin_words.json");

        unique_words
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
    })
}

pub fn get_latin_inflections() -> &'static Vec<Inflection> {
    static CACHE: OnceLock<Vec<Inflection>> = OnceLock::new();
    cached_json!("../dictionary/latin_inflections.json", CACHE)
}

pub fn get_latin_stems() -> &'static Vec<Stem> {
    static CACHE: OnceLock<Vec<Stem>> = OnceLock::new();
    cached_json!("../dictionary/latin_stems.json", CACHE)
}

pub fn get_latin_prefixes() -> &'static Vec<Modifier> {
    static CACHE: OnceLock<Vec<Modifier>> = OnceLock::new();
    cached_json!("../dictionary/latin_prefixes.json", CACHE)
}

pub fn get_latin_suffixes() -> &'static Vec<Modifier> {
    static CACHE: OnceLock<Vec<Modifier>> = OnceLock::new();
    cached_json!("../dictionary/latin_suffixes.json", CACHE)
}

pub fn get_latin_packons() -> &'static Vec<Attachment> {
    static CACHE: OnceLock<Vec<Attachment>> = OnceLock::new();
    cached_json!("../dictionary/latin_packons.json", CACHE)
}

pub fn get_latin_not_packons() -> &'static Vec<Attachment> {
    static CACHE: OnceLock<Vec<Attachment>> = OnceLock::new();
    cached_json!("../dictionary/latin_not_packons.json", CACHE)
}

pub fn get_latin_tackons() -> &'static Vec<Attachment> {
    static CACHE: OnceLock<Vec<Attachment>> = OnceLock::new();
    cached_json!("../dictionary/latin_tackons.json", CACHE)
}

pub fn get_latin_tickons() -> &'static Vec<Attachment> {
    static CACHE: OnceLock<Vec<Attachment>> = OnceLock::new();
    cached_json!("../dictionary/latin_tickons.json", CACHE)
}
