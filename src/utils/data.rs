use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

//TODO: account for words that have a string as n
#[derive(Serialize, Deserialize, Debug)]
pub struct LatinWordInfo {
    pub pos: String,
    pub n: Vec<i8>,
    pub parts: Vec<String>,
    pub senses: Vec<String>,
    pub form: String,
    pub orth: String,
    pub id: i32,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueLatinWordInfo {
    pub orth: String,
    pub senses: Vec<String>,
    pub pos: String,
    pub form: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Inflection {
    pub ending: String,
    pub pos: String,
    pub notes: String,
    pub n: Vec<i8>,
    pub form: String,
}

impl Clone for Inflection {
    fn clone(&self) -> Inflection {
        Inflection {
            ending: self.ending.clone(),
            pos: self.pos.clone(),
            notes: self.notes.clone(),
            n: self.n.clone(),
            form: self.form.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stem {
    pub pos: String,
    pub form: String,
    pub orth: String,
    pub n: Vec<i8>,
    pub wid: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modifier {
    pub pos: String,
    pub form: String,
    pub senses: Vec<String>,
    pub orth: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachments {
    pub pos: String,
    pub senses: Vec<String>,
    pub orth: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnglishWordInfo {
    pub orth: String,
    pub wid: i32,
    pub pos: String,
    pub frequency_type: String,
    pub true_frequency: i16,
    pub frequency: i16,
    pub compound: i16,
    pub semi: i16,
    pub latin_entry: LatinWordInfo,
}

impl From<EnglishWordInfo> for Vec<String> {
    fn from(word_info: EnglishWordInfo) -> Self {
        vec![
            word_info.orth,
            word_info.wid.to_string(),
            word_info.pos,
            word_info.frequency_type,
            word_info.true_frequency.to_string(),
            word_info.frequency.to_string(),
            word_info.compound.to_string(),
            word_info.semi.to_string(),
        ]
    }
}

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

pub fn get_latin_prefixes() -> Vec<Modifier> {
    serde_json::from_str(&fs::read_to_string("src/data/latin_prefixes.json").unwrap()).unwrap()
}

pub fn get_latin_suffixes() -> Vec<Modifier> {
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
