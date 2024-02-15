use crate::dictionary_structures::dictionary_values::{Modifier, ModifierType, Stem};
use crate::translators::latin_to_english::parser::find_form;
use crate::translators::latin_to_english::LatinTranslationInfo;
use crate::utils::data::{
    get_latin_not_packons, get_latin_packons, get_latin_prefixes, get_latin_suffixes,
    get_latin_tackons,
};

pub fn add_stem_to_word(matched_stem: Stem, matching_word: Option<&mut LatinTranslationInfo>) {
    if let Some(word) = matching_word {
        let stem_to_add = &matched_stem.orth;
        let word_stems = &mut word.stem;

        if word_stems.orth != *stem_to_add {
            word_stems.orth = stem_to_add.to_string();
        }
    }
}

pub fn reduce(latin_word: &str) -> Option<Vec<LatinTranslationInfo>> {
    let mut modifiers: Vec<Modifier> = Vec::new();
    let latin_prefixes = get_latin_prefixes();
    let latin_suffixes = get_latin_suffixes();

    let mut stripped_latin_word = latin_word.to_string();
    latin_prefixes.iter().for_each(|prefix| {
        if stripped_latin_word.starts_with(prefix.orth.as_str()) {
            stripped_latin_word = stripped_latin_word
                .trim_start_matches(prefix.orth.as_str())
                .to_string();

            let mut modifier = Modifier::new();
            modifier.set_orth(&prefix.orth);
            modifier.set_pos(prefix.pos);
            modifier.set_senses(&prefix.senses);
            modifier.set_modifier(ModifierType::Prefix);

            modifiers.push(modifier);
        }
    });

    latin_suffixes.iter().for_each(|suffix| {
        if stripped_latin_word.ends_with(suffix.orth.as_str()) {
            stripped_latin_word = stripped_latin_word
                .trim_end_matches(suffix.orth.as_str())
                .to_string();

            let mut modifier = Modifier::new();
            modifier.set_orth(&suffix.orth);
            modifier.set_pos(suffix.pos);
            modifier.set_senses(&suffix.senses);
            modifier.set_modifier(ModifierType::Suffix);

            modifiers.push(modifier);
        }
    });

    if stripped_latin_word == latin_word || stripped_latin_word.len() == 0 || modifiers.len() == 0 {
        return None;
    }

    let mut output = find_form(&stripped_latin_word, true);

    if output.is_some() {
        for word in output.as_mut().unwrap() {
            word.word.set_modifiers(modifiers.clone());
        }

        return output;
    } else {
        return None;
    }
}

pub fn split_enclitic(latin_word: &str) -> (String, Vec<Modifier>) {
    let mut modifiers: Vec<Modifier> = Vec::new();
    let latin_not_packons = get_latin_not_packons();
    let mut split_word = latin_word.to_string();
    let latin_tackons = get_latin_tackons();
    let latin_packons = get_latin_packons();

    let tackon = latin_tackons
        .iter()
        .find(|tackon| split_word.ends_with(tackon.orth.as_str()));

    if tackon.is_some() {
        let tackon = tackon.unwrap();

        // Est exception
        if latin_word != "est" {
            let mut modifier = Modifier::new();
            modifier.set_orth(&tackon.orth);
            modifier.set_pos(tackon.pos);
            modifier.set_senses(&tackon.senses);
            modifier.set_modifier(ModifierType::Tackon);

            split_word.truncate(split_word.len() - tackon.orth.len());
            modifiers.push(modifier);
        }
    } else {
        if latin_word.starts_with("qu") {
            for packon in latin_packons {
                if split_word.ends_with(packon.orth.as_str()) {
                    let mut modifier = Modifier::new();
                    modifier.set_orth(&packon.orth);
                    modifier.set_pos(packon.pos);
                    modifier.set_senses(&packon.senses);
                    modifier.set_modifier(ModifierType::Packon);

                    split_word.truncate(split_word.len() - packon.orth.len());
                    modifiers.push(modifier);
                }
            }
        } else {
            for packon in latin_not_packons {
                if split_word.ends_with(packon.orth.as_str()) {
                    let mut modifier = Modifier::new();
                    modifier.set_orth(&packon.orth);
                    modifier.set_pos(packon.pos);
                    modifier.set_senses(&packon.senses);
                    modifier.set_modifier(ModifierType::Packon);

                    split_word.truncate(split_word.len() - packon.orth.len());
                    modifiers.push(modifier);
                }
            }
        }
    }

    (split_word, modifiers)
}
