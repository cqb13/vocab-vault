use crate::translators::english_to_latin::EnglishTranslationInfo;

pub fn weigh_words(word_list: Vec<EnglishTranslationInfo>) -> Vec<EnglishTranslationInfo> {
    let mut weighted_word_list = word_list;
    weighted_word_list.sort_by(|a, b| b.word.true_frequency.cmp(&a.word.true_frequency));
    weighted_word_list
}

pub fn remove_duplicates(word_list: Vec<EnglishTranslationInfo>) -> Vec<EnglishTranslationInfo> {
    let mut deduped_word_list: Vec<EnglishTranslationInfo> = Vec::new();
    let mut seen_wids: Vec<i32> = Vec::new();

    for word_info in word_list {
        if !seen_wids.contains(&word_info.word.wid) {
            seen_wids.push(word_info.word.wid);
            deduped_word_list.push(word_info);
        }
    }

    deduped_word_list
}
