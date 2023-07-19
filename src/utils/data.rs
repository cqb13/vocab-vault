use serde_json::Value;
use std::fs;

pub fn get_english_words() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/english_words.json").unwrap()).unwrap()
}

pub fn get_latin_dictionary() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_dictionary.json").unwrap()).unwrap()
}

pub fn get_unique_latin_words() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/unique_latin_words.json").unwrap()).unwrap()
}

pub fn get_latin_stems() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_stems.json").unwrap()).unwrap()
}

pub fn get_latin_inflections() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_inflections.json").unwrap()).unwrap()
}

pub fn get_latin_addons() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_addons.json").unwrap()).unwrap()
}

pub fn get_latin_prefixes() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_prefixes.json").unwrap()).unwrap()
}

pub fn get_latin_suffixes() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_suffixes.json").unwrap()).unwrap()
}

pub fn get_latin_packons() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_packons.json").unwrap()).unwrap()
}

pub fn get_latin_not_packons() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_not_packons.json").unwrap()).unwrap()
}

pub fn get_latin_tackons() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_tackons.json").unwrap()).unwrap()
}

pub fn get_latin_tickons() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_tickons.json").unwrap()).unwrap()
}
