use crate::utils::data::{get_english_words, get_latin_dictionary, EnglishWordInfo, LatinWordInfo};
use serde_json::Value;

pub fn translate_to_latin(english_word: &str) -> Vec<EnglishWordInfo> {
    const MAX_RESPONSE_ITEMS: usize = 6;
    let mut output: Vec<EnglishWordInfo> = Vec::new();

    let english_words: Value = get_english_words();
    for word in english_words.as_array().unwrap() {
        if word["orth"].as_str().unwrap_or_default().to_lowercase() == english_word.to_lowercase() {
            let word_info = EnglishWordInfo {
                orth: word["orth"].as_str().unwrap_or_default().to_string(),
                wid: word["wid"].as_i64().unwrap_or_default() as i32,
                pos: word["pos"].as_str().unwrap_or_default().to_string(),
                frequency_type: word["frequencyType"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                true_frequency: calculate_true_frequency(
                    word["frequency"].as_i64().unwrap_or_default() as i16,
                    word["compound"].as_i64().unwrap_or_default() as i16,
                    word["semi"].as_i64().unwrap_or_default() as i16,
                ),
                frequency: word["frequency"].as_i64().unwrap_or_default() as i16,
                compound: word["compound"].as_i64().unwrap_or_default() as i16,
                semi: word["semi"].as_i64().unwrap_or_default() as i16,
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

    output = remove_duplicates(output);

    // other words are probably rare/irrelevant or wrong
    if output.len() > MAX_RESPONSE_ITEMS {
        output.truncate(MAX_RESPONSE_ITEMS);
    }

    find_definition(&mut output);

    output
}

fn calculate_true_frequency(frequency: i16, compound: i16, semi: i16) -> i16 {
    frequency + compound - semi
}

fn weigh_words(word_list: Vec<EnglishWordInfo>) -> Vec<EnglishWordInfo> {
    let mut weighted_word_list = word_list;
    weighted_word_list.sort_by(|a, b| b.true_frequency.cmp(&a.true_frequency));
    weighted_word_list
}

fn remove_duplicates(word_list: Vec<EnglishWordInfo>) -> Vec<EnglishWordInfo> {
    let mut deduped_word_list: Vec<EnglishWordInfo> = Vec::new();
    let mut seen_wids: Vec<i32> = Vec::new();

    for word_info in word_list {
        if !seen_wids.contains(&word_info.wid) {
            seen_wids.push(word_info.wid);
            deduped_word_list.push(word_info);
        }
    }

    deduped_word_list
}

fn find_definition(word_list: &mut Vec<EnglishWordInfo>) {
    let latin_dictionary: Value = get_latin_dictionary();

    for word_info in word_list.iter_mut() {
        for latin_word in latin_dictionary.as_array().unwrap() {
            if latin_word["id"].as_i64().unwrap_or_default() as i32 == word_info.wid {
                word_info.latin_entry.pos =
                    latin_word["pos"].as_str().unwrap_or_default().to_string();

                if let Some(n) = latin_word["n"].as_array() {
                    word_info.latin_entry.n = n
                        .iter()
                        .map(|x| x.as_i64().unwrap_or_default() as i8)
                        .collect();
                }

                if let Some(parts) = latin_word["parts"].as_array() {
                    word_info.latin_entry.parts = parts
                        .iter()
                        .map(|x| x.as_str().unwrap_or_default().to_string())
                        .collect();
                }

                if let Some(senses) = latin_word["senses"].as_array() {
                    word_info.latin_entry.senses = senses
                        .iter()
                        .map(|x| x.as_str().unwrap_or_default().to_string())
                        .collect();
                }

                word_info.latin_entry.form =
                    latin_word["form"].as_str().unwrap_or_default().to_string();
                word_info.latin_entry.orth =
                    latin_word["orth"].as_str().unwrap_or_default().to_string();
                word_info.latin_entry.id = latin_word["id"].as_i64().unwrap_or_default() as i32;
            }
        }
    }
}
