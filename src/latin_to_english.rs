use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct LatinTranslationInfo {
    pub word: Word,
}

#[derive(Serialize, Deserialize)]
pub enum Word {
    LatinWordInfo(LatinWordInfo),
    UniqueLatinWordInfo(UniqueLatinWordInfo),
}

#[derive(Serialize, Deserialize)]
pub struct LatinWordInfo {
    pub pos: String,
    pub n: Vec<i8>,
    pub parts: Vec<String>,
    pub senses: Vec<String>,
    pub form: String,
    pub orth: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UniqueLatinWordInfo {
    pub orth: String,
    pub senses: Vec<String>,
    pub pos: String,
    pub form: String,
}

impl LatinWordInfo {
    pub fn new() -> LatinWordInfo {
        LatinWordInfo {
            pos: "".to_string(),
            n: Vec::new(),
            parts: Vec::new(),
            senses: Vec::new(),
            form: "".to_string(),
            orth: "".to_string(),
            id: 0,
        }
    }
}

impl UniqueLatinWordInfo {
    pub fn new() -> UniqueLatinWordInfo {
        UniqueLatinWordInfo {
            orth: "".to_string(),
            senses: Vec::new(),
            pos: "".to_string(),
            form: "".to_string(),
        }
    }
}

pub fn translate_to_english(latin_word: &str) -> Word {
    let LATIN_DICTIONARY: Value = get_latin_dictionary();
    let mut is_unique: bool = false;
    let mut output: UniqueLatinWordInfo = UniqueLatinWordInfo::new();

    output = parse_unique_latin_words(latin_word);

    return Word::UniqueLatinWordInfo(output);
}

fn parse_unique_latin_words(latin_word: &str) -> UniqueLatinWordInfo{
    let mut unique_latin_word = UniqueLatinWordInfo::new();
    let unique_words: Value =
        serde_json::from_str(&fs::read_to_string("src/data/unique_latin_words.json").unwrap())
            .unwrap();

    for object in unique_words.as_array().unwrap() {
        if object["orth"].as_str().unwrap_or_default().to_lowercase() == latin_word.to_lowercase() {
            unique_latin_word = UniqueLatinWordInfo {
                orth: object["orth"].as_str().unwrap_or_default().to_string(),
                senses: object["senses"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|sense| sense.as_str().unwrap_or_default().to_string())
                    .collect(),
                pos: object["pos"].as_str().unwrap_or_default().to_string(),
                form: object["form"].as_str().unwrap_or_default().to_string(),
            };
        }
    }

    return unique_latin_word;
}

pub fn get_latin_dictionary() -> Value {
    serde_json::from_str(&fs::read_to_string("src/data/latin_dictionary.json").unwrap()).unwrap()
}
