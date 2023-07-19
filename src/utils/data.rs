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
