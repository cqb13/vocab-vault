use crate::utils::data::{get_english_words, get_latin_dictionary, EnglishWordInfo, LatinWordInfo};

pub fn translate_to_latin(english_word: &str) -> Vec<EnglishWordInfo> {
    const MAX_RESPONSE_ITEMS: usize = 6;
    let mut output: Vec<EnglishWordInfo> = Vec::new();

    let english_words = get_english_words();
    for word in english_words {
        if word.orth == english_word.to_lowercase() {
            let word_info = EnglishWordInfo {
                orth: word.orth,
                wid: word.wid,
                pos: word.pos,
                frequency_type: word.frequency_type,
                true_frequency: calculate_true_frequency(
                    word.frequency,
                    word.compound,
                    word.semi,
                ),
                frequency: word.frequency,
                compound: word.compound,
                semi: word.semi,
                latin_entry: LatinWordInfo::new(),
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
    let latin_dictionary = get_latin_dictionary();

    for word_info in word_list.iter_mut() {
        for latin_word in latin_dictionary {
            if latin_word.id == word_info.wid {
                word_info.latin_entry = latin_word;
            }
        }
    }
}
