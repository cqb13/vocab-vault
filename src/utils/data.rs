use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::include_bytes;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attachment {
    pub pos: String,
    pub senses: Vec<String>,
    pub orth: String,
}

impl Attachment {
    pub fn new() -> Attachment {
        Attachment {
            pos: "".to_string(),
            senses: Vec::new(),
            orth: "".to_string(),
        }
    }
}

impl PartialEq for Attachment {
    fn eq(&self, other: &Self) -> bool {
        self.orth == other.orth
    }
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
    let english_words_json = include_bytes!("../data/english_words.json");
    serde_json::from_slice(english_words_json).unwrap()
}

pub fn get_latin_dictionary() -> Value {
    let latin_dictionary_json = include_bytes!("../data/latin_dictionary.json");
    serde_json::from_slice(latin_dictionary_json).unwrap()
}

pub fn get_unique_latin_words() -> Value {
    let unique_latin_words_json = include_bytes!("../data/unique_latin_words.json");
    serde_json::from_slice(unique_latin_words_json).unwrap()
}

pub fn get_latin_stems() -> Value {
    let latin_stems_json = include_bytes!("../data/latin_stems.json");
    serde_json::from_slice(latin_stems_json).unwrap()
}

pub fn get_latin_inflections() -> Value {
    let latin_inflections_json = include_bytes!("../data/latin_inflections.json");
    serde_json::from_slice(latin_inflections_json).unwrap()
}

pub fn get_latin_addons() -> Value {
    let latin_addons_json = include_bytes!("../data/latin_addons.json");
    serde_json::from_slice(latin_addons_json).unwrap()
}

pub fn get_latin_prefixes() -> Vec<Modifier> {
    let latin_prefixes_json = include_bytes!("../data/latin_prefixes.json");
    serde_json::from_slice(latin_prefixes_json).unwrap()
}

pub fn get_latin_suffixes() -> Vec<Modifier> {
    let latin_suffixes_json = include_bytes!("../data/latin_suffixes.json");
    serde_json::from_slice(latin_suffixes_json).unwrap()
}

pub fn get_latin_packons() -> Vec<Attachment> {
    let latin_packons_json = include_bytes!("../data/latin_packons.json");
    serde_json::from_slice(latin_packons_json).unwrap()
}

pub fn get_latin_not_packons() -> Vec<Attachment> {
    let latin_not_packons_json = include_bytes!("../data/latin_not_packons.json");
    serde_json::from_slice(latin_not_packons_json).unwrap()
}

pub fn get_latin_tackons() -> Vec<Attachment> {
    let latin_tackons_json = include_bytes!("../data/latin_tackons.json");
    serde_json::from_slice(latin_tackons_json).unwrap()
}

pub fn get_latin_tickons() -> Vec<Attachment> {
    let latin_tickons_json = include_bytes!("../data/latin_tickons.json");
    serde_json::from_slice(latin_tickons_json).unwrap()
}
