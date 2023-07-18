use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::Ordering;
use std::fs;

use crate::latin_to_english::LatinWordInfo;

#[derive(Serialize, Deserialize)]
pub struct WordInfo {
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

impl From<WordInfo> for Vec<String> {
    fn from(word_info: WordInfo) -> Self {
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

pub fn translate_to_latin(english_word: &str) -> Vec<WordInfo> {
    const RESPONSE_LIMIT: usize = 6;
    let mut output: Vec<WordInfo> = Vec::new();

    let english_words: Value =
        serde_json::from_str(&fs::read_to_string("src/data/english_words.json").unwrap()).unwrap();
    for object in english_words.as_array().unwrap() {
        if object["orth"].as_str().unwrap_or_default().to_lowercase() == english_word.to_lowercase()
        {
            let word_info = WordInfo {
                orth: object["orth"].as_str().unwrap_or_default().to_string(),
                wid: object["wid"].as_i64().unwrap_or_default() as i32,
                pos: object["pos"].as_str().unwrap_or_default().to_string(),
                frequency_type: object["frequencyType"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                true_frequency: calculate_true_frequency(
                    object["frequency"].as_i64().unwrap_or_default() as i16,
                    object["compound"].as_i64().unwrap_or_default() as i16,
                    object["semi"].as_i64().unwrap_or_default() as i16,
                ),
                frequency: object["frequency"].as_i64().unwrap_or_default() as i16,
                compound: object["compound"].as_i64().unwrap_or_default() as i16,
                semi: object["semi"].as_i64().unwrap_or_default() as i16,
                latin_entry: LatinWordInfo {
                    pos: "".to_string(),
                    n: Vec::new(),
                    parts: Vec::new(),
                    senses: Vec::new(),
                    form: "".to_string(),
                    orth: "".to_string(),
                    id: 0,
                },
            };
            output.push(word_info.into());
        }
    }

    output = weigh_words(output);

    //output = remove_duplicates(output);

    // other words are probably rare/irrelevant or wrong
    if output.len() > RESPONSE_LIMIT {
        output.truncate(RESPONSE_LIMIT);
    }

    output = find_definition(output);

    output
}

fn calculate_true_frequency(frequency: i16, compound: i16, semi: i16) -> i16 {
    let true_frequency = frequency + compound - semi;
    return true_frequency;
}

fn weigh_words(word_list: Vec<WordInfo>) -> Vec<WordInfo> {
    let mut weighted_word_list = word_list;
    weighted_word_list.sort_by(|a, b| {
        if a.true_frequency > b.true_frequency {
            Ordering::Less
        } else if a.true_frequency < b.true_frequency {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    weighted_word_list
}

fn find_definition(word_list: Vec<WordInfo>) -> Vec<WordInfo> {
    let latin_dictionary: Value =
        serde_json::from_str(&fs::read_to_string("src/data/latin_dictionary.json").unwrap())
            .unwrap();

    let mut updated_word_list = word_list;

    for word_info in &mut updated_word_list {
        for object in latin_dictionary.as_array().unwrap() {
            if object["id"].as_i64().unwrap_or_default() as i32 == word_info.wid {
                word_info.latin_entry.pos = object["pos"].as_str().unwrap_or_default().to_string();

                if let Some(n) = object["n"].as_array() {
                    word_info.latin_entry.n = n
                        .iter()
                        .map(|x| x.as_i64().unwrap_or_default() as i8)
                        .collect();
                }

                if let Some(parts) = object["parts"].as_array() {
                    word_info.latin_entry.parts = parts
                        .iter()
                        .map(|x| x.as_str().unwrap_or_default().to_string())
                        .collect();
                }

                if let Some(senses) = object["senses"].as_array() {
                    word_info.latin_entry.senses = senses
                        .iter()
                        .map(|x| x.as_str().unwrap_or_default().to_string())
                        .collect();
                }

                word_info.latin_entry.form =
                    object["form"].as_str().unwrap_or_default().to_string();
                word_info.latin_entry.orth =
                    object["orth"].as_str().unwrap_or_default().to_string();
                word_info.latin_entry.id = object["id"].as_i64().unwrap_or_default() as i32;
            }
        }
    }

    updated_word_list
}
