use serde::{Deserialize, Serialize};
use std::include_bytes;

//TODO: account for words that have a string as n
#[derive(Serialize, Deserialize, Debug)]
pub struct LatinWordInfo {
    pub pos: String,
    pub n: Vec<NValue>,
    pub parts: Vec<String>,
    pub senses: Vec<String>,
    pub form: String,
    pub orth: String,
    pub id: i32,
}

impl Clone for LatinWordInfo {
    fn clone(&self) -> LatinWordInfo {
        LatinWordInfo {
            pos: self.pos.clone(),
            n: self.n.clone(),
            parts: self.parts.clone(),
            senses: self.senses.clone(),
            form: self.form.clone(),
            orth: self.orth.clone(),
            id: self.id.clone(),
        }
    }
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

impl Clone for UniqueLatinWordInfo {
    fn clone(&self) -> UniqueLatinWordInfo {
        UniqueLatinWordInfo {
            orth: self.orth.clone(),
            senses: self.senses.clone(),
            pos: self.pos.clone(),
            form: self.form.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inflection {
    pub ending: String,
    pub pos: String,
    pub notes: Option<String>,
    pub n: Vec<NValue>,
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
    pub n: Vec<NValue>,
    pub wid: i32,
}

impl Stem {
    pub fn new() -> Stem {
        Stem {
            pos: "".to_string(),
            form: "".to_string(),
            orth: "".to_string(),
            n: Vec::new(),
            wid: 0,
        }
    }
}

impl PartialEq for Stem {
    fn eq(&self, other: &Self) -> bool {
        self.orth == other.orth
    }
}

impl Clone for Stem {
    fn clone(&self) -> Stem {
        Stem {
            pos: self.pos.clone(),
            form: self.form.clone(),
            orth: self.orth.clone(),
            n: self.n.clone(),
            wid: self.wid.clone(),
        }
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EnglishWordInfo {
    pub orth: String,
    pub wid: i32,
    pub pos: String,
    pub frequency_type: String,
    pub true_frequency: Option<i16>,
    pub frequency: i16,
    pub compound: i16,
    pub semi: i16,
    pub latin_entry: Option<LatinWordInfo>,
}
impl Clone for EnglishWordInfo {
    fn clone(&self) -> EnglishWordInfo {
        EnglishWordInfo {
            orth: self.orth.clone(),
            wid: self.wid.clone(),
            pos: self.pos.clone(),
            frequency_type: self.frequency_type.clone(),
            true_frequency: self.true_frequency.clone(),
            frequency: self.frequency.clone(),
            compound: self.compound.clone(),
            semi: self.semi.clone(),
            latin_entry: self.latin_entry.clone(),
        }
    }
}

impl From<EnglishWordInfo> for Vec<String> {
    fn from(word_info: EnglishWordInfo) -> Self {
        vec![
            word_info.orth,
            word_info.wid.to_string(),
            word_info.pos,
            word_info.frequency_type,
            word_info
                .true_frequency
                .map_or_else(|| "None".to_string(), |freq| freq.to_string()),
            word_info.frequency.to_string(),
            word_info.compound.to_string(),
            word_info.semi.to_string(),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NValue {
    StrIntInt(String, i8, i8),
    IntInt(i8, i8),
    Integer(i8),
    String(String),
}

impl Clone for NValue {
    fn clone(&self) -> NValue {
        match self {
            NValue::StrIntInt(s, i1, i2) => NValue::StrIntInt(s.clone(), i1.clone(), i2.clone()),
            NValue::IntInt(i1, i2) => NValue::IntInt(i1.clone(), i2.clone()),
            NValue::Integer(i) => NValue::Integer(i.clone()),
            NValue::String(s) => NValue::String(s.clone()),
        }
    }
}

impl PartialEq for NValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NValue::StrIntInt(s1, i1, i2), NValue::StrIntInt(s2, i3, i4)) => {
                s1 == s2 && i1 == i3 && i2 == i4
            }
            (NValue::IntInt(i1, i2), NValue::IntInt(i3, i4)) => i1 == i3 && i2 == i4,
            (NValue::Integer(i1), NValue::Integer(i2)) => i1 == i2,
            (NValue::String(s1), NValue::String(s2)) => s1 == s2,
            _ => false,
        }
    }
}

pub fn get_english_words() -> Vec<EnglishWordInfo> {
    let english_words_json = include_bytes!("../data/english_words.json");
    serde_json::from_slice(english_words_json).unwrap()
}

pub fn get_latin_dictionary() -> Vec<LatinWordInfo> {
    let latin_dictionary_json = include_bytes!("../data/latin_dictionary.json");
    serde_json::from_slice(latin_dictionary_json).unwrap()
}

pub fn get_unique_latin_words() -> Vec<UniqueLatinWordInfo> {
    let unique_latin_words_json = include_bytes!("../data/unique_latin_words.json");
    serde_json::from_slice(unique_latin_words_json).unwrap()
}

pub fn get_latin_stems() -> Vec<Stem> {
    let latin_stems_json = include_bytes!("../data/latin_stems.json");
    serde_json::from_slice(latin_stems_json).unwrap()
}

pub fn get_latin_inflections() -> Vec<Inflection> {
    let latin_inflections_json = include_bytes!("../data/latin_inflections.json");
    serde_json::from_slice(latin_inflections_json).unwrap()
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
