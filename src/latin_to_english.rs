use serde::{Deserialize, Serialize, Serializer};

use crate::data::data::{
    get_latin_dictionary, get_latin_inflections, get_latin_not_packons, get_latin_packons,
    get_latin_prefixes, get_latin_stems, get_latin_suffixes, get_latin_tackons,
    get_unique_latin_words, Attachment, Form, Inflection, LatinWordInfo, Modifier, Stem,
    UniqueLatinWordInfo, WordInfo,
};

use crate::tricks::tricks::{evaluate_roman_numeral, is_roman_number, try_tricks};
use crate::tricks::word_mods::try_syncopes;

#[derive(Serialize, Deserialize, Debug)]
pub struct LatinTranslationInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tricks: Option<Vec<String>>,
    #[serde(serialize_with = "serialize_word")]
    pub word: Word,
    pub stem: Stem,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inflections: Option<Vec<Inflection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addon: Option<String>,
}

impl Clone for LatinTranslationInfo {
    fn clone(&self) -> Self {
        LatinTranslationInfo {
            tricks: self.tricks.clone(),
            word: self.word.clone(),
            stem: self.stem.clone(),
            inflections: self.inflections.clone(),
            addon: self.addon.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Word {
    #[serde(rename = "LatinWordInfo")]
    LatinWordInfo(LatinWordInfo),
    #[serde(rename = "UniqueLatinWordInfo")]
    UniqueLatinWordInfo(UniqueLatinWordInfo),
}

impl Clone for Word {
    fn clone(&self) -> Self {
        match self {
            Word::LatinWordInfo(latin_word_info) => Word::LatinWordInfo(latin_word_info.clone()),
            Word::UniqueLatinWordInfo(unique_latin_word_info) => {
                Word::UniqueLatinWordInfo(unique_latin_word_info.clone())
            }
        }
    }
}

fn serialize_word<S>(word: &Word, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match word {
        Word::UniqueLatinWordInfo(info) => info.serialize(serializer),
        Word::LatinWordInfo(info) => info.serialize(serializer),
    }
}

pub fn translate_to_english(latin_word: String, tricks: bool) -> Vec<LatinTranslationInfo> {
    let mut output = parse(&latin_word, false);
    if tricks {
        let (mut modified_latin_word, mut trick_explanations) = try_tricks(latin_word.clone());

        let (syncopated_word, trick_explanation) = try_syncopes(modified_latin_word.clone());
        if syncopated_word != modified_latin_word {
            modified_latin_word = syncopated_word;
            let mut trick_explanation_list = trick_explanations.unwrap_or(Vec::new());
            trick_explanation_list.push(trick_explanation);
            trick_explanations = Some(trick_explanation_list);
        }

        let mut output_from_trick = parse(&modified_latin_word, false);

        if modified_latin_word != latin_word {
            if output_from_trick.len() > 0 {
                for entry in &mut output_from_trick {
                    entry.tricks = trick_explanations.clone();
                }
            }

            output.append(&mut output_from_trick);
        }
    }

    // most words should be found by now

    // If nothing is found, try removing enclitics and try again
    // ex: clamaverunt -> clamare
    // doing this here instead of earlier should fix words like salve having the "ve" removed and returning wrong def
    if output.len() == 0 {
        let (split_word, modifier) = split_enclitic(&latin_word);
        for entry in &mut output {
            if let Word::LatinWordInfo(latin_word_info) = &mut entry.word {
                latin_word_info.modifiers = Some(modifier.clone());
            }
        }
        output = parse(&split_word, false);
    }

    if is_roman_number(&latin_word) {
        let numeral_evaluation = evaluate_roman_numeral(&latin_word);

        if numeral_evaluation > 0 {
            output.push(LatinTranslationInfo {
                tricks: None,
                word: Word::UniqueLatinWordInfo(UniqueLatinWordInfo {
                    orth: latin_word.to_uppercase(),
                    senses: [numeral_evaluation.to_string()].to_vec(),
                    pos: "NUM".to_string(),
                    form: Form::StrForm("NUM".to_string()),
                    n: None,
                    info: WordInfo::new_set(
                        "X".to_string(),
                        "T".to_string(),
                        "I".to_string(),
                        "C".to_string(),
                        "X".to_string(),
                    ),
                }),
                stem: Stem::new(),
                inflections: None,
                addon: Some("roman numeral".to_string()),
            });
        }
    }

    output
}

fn parse(latin_word: &str, reduced: bool) -> Vec<LatinTranslationInfo> {
    let mut output: Vec<LatinTranslationInfo> = Vec::new();

    let (unique_latin_word, found) = parse_unique_latin_words(&latin_word);

    if found {
        output.push(LatinTranslationInfo {
            tricks: None,
            word: Word::UniqueLatinWordInfo(unique_latin_word),
            stem: Stem::new(),
            inflections: None,
            addon: Some("unique".to_string()),
        });
    }

    if output.len() == 0 {
        output = find_form(&latin_word, reduced);
    }

    output
}

fn parse_unique_latin_words(latin_word: &str) -> (UniqueLatinWordInfo, bool) {
    let mut unique_latin_word = UniqueLatinWordInfo::new();
    let unique_words = get_unique_latin_words();
    let mut found = false;
    for unique_word in unique_words {
        if unique_word.orth.to_lowercase() == latin_word {
            unique_latin_word = unique_word.clone();
            found = true;
            break;
        }
    }

    return (unique_latin_word, found);
}

fn find_form(latin_word: &str, reduced: bool) -> Vec<LatinTranslationInfo> {
    let mut latin_word_inflections: Vec<Inflection> = Vec::new();
    let latin_inflections: Vec<Inflection> = get_latin_inflections();

    for inflection in latin_inflections {
        if latin_word.ends_with(inflection.ending.as_str()) {
            // if the longest inflection has been found, stop looking
            if latin_word_inflections.len() > 0
                && latin_word_inflections[0].ending.len() > inflection.ending.len()
            {
                break;
            } else {
                latin_word_inflections.push(inflection.clone());
            }
        }
    }

    let (mut stems, mut inflections) = check_stems(latin_word, &latin_word_inflections, true);
    // If no stems were found, try again without ensuring infls
    // allows for words that end with erunt and similar cases
    // ex: clamaverunt return null without this
    let (more_stems, more_inflections) = check_stems(latin_word, &latin_word_inflections, false);
    stems.extend(more_stems);
    inflections.extend(more_inflections);

    // remove duplicates
    //TODO: test if this is needed
    stems.sort_by(|a, b| a.pos.cmp(&b.pos));
    stems.dedup_by(|a, b| {
        a.pos == b.pos && a.form == b.form && a.orth == b.orth && a.n == b.n && a.wid == b.wid
    });

    inflections.sort_by(|a, b| a.ending.len().cmp(&b.ending.len()));
    inflections.dedup_by(|a, b| {
        a.ending == b.ending && a.pos == b.pos && a.n == b.n && a.form == b.form && a.note == b.note
    });

    let mut output = lookup_stems(stems, inflections);

    if output.len() == 0 && !reduced {
        let (mut reduced_output, found) = reduce(&mut latin_word.to_string());
        if found {
            output.append(&mut reduced_output);
        }
    }

    let mut filtered_output = Vec::new();

    // remove inflections from output, that don't match n with the word
    for mut definition in output.clone() {
        let unfiltered_inflections = definition.inflections.clone();
        let n = match &definition.word {
            Word::LatinWordInfo(word) => word.n.clone(),
            Word::UniqueLatinWordInfo(word) => word.n.clone(),
        };

        let mut filtered_inflections: Vec<Inflection> = Vec::new();

        match n {
            Some(n) => {
                for inflection in unfiltered_inflections.unwrap_or(Vec::new()) {
                    if inflection.n[0] != n[0] {
                        continue;
                    }

                    filtered_inflections.push(inflection);
                }
            }
            None => {
                filtered_inflections = unfiltered_inflections.unwrap_or(Vec::new());
            }
        }

        let mut unique_inflections: Vec<Inflection> = Vec::new();

        for inflection in filtered_inflections.clone() {
            let mut is_unique = true;
            for unique_inflection in unique_inflections.clone() {
                if inflection.form == unique_inflection.form {
                    is_unique = false;
                    continue;
                }
            }
            if is_unique {
                unique_inflections.push(inflection);
            }
        }

        definition.inflections = Some(unique_inflections);
        filtered_output.push(definition);
    }

    filtered_output
}

fn check_stems(
    latin_word: &str,
    latin_word_inflections: &Vec<Inflection>,
    ensure_inflections: bool,
) -> (Vec<Stem>, Vec<Inflection>) {
    let latin_stems = get_latin_stems();
    let mut matched_stems: Vec<Stem> = Vec::new();
    let mut inflections: Vec<Inflection> = Vec::new();

    for inflection in latin_word_inflections {
        let word_stem = latin_word.trim_end_matches(&inflection.ending);

        for stem in &latin_stems {
            if word_stem == stem.orth {
                if inflection.pos == stem.pos
                    || (inflection.pos == "VPAR" && stem.pos == "V")
                    || (inflection.pos == "V" && stem.pos == "VPAR")
                {
                    if inflection.n != stem.n && ensure_inflections {
                        continue;
                    }
                    let mut in_stem_inflections = false;
                    for stem_inflection in &inflections {
                        if stem_inflection.pos == inflection.pos
                            || (stem_inflection.pos == "VPAR" && inflection.pos == "V")
                            || (stem_inflection.pos == "V" && inflection.pos == "VPAR")
                        {
                            in_stem_inflections = true;
                            break;
                        }
                    }
                    if !in_stem_inflections {
                        // !!!: this is important, but needs to be filtered somehow (cursus | whitakers words)
                        //println!("added infl form {:?}", inflection.form);
                        inflections.push(inflection.clone());
                    }
                    matched_stems.push(stem.clone());
                    inflections.push(inflection.clone());
                }
            }
        }
    }

    (matched_stems, inflections)
}

fn lookup_stems(stems: Vec<Stem>, inflections: Vec<Inflection>) -> Vec<LatinTranslationInfo> {
    let latin_dictionary = get_latin_dictionary();
    let mut output: Vec<LatinTranslationInfo> = Vec::new();

    for stem in stems {
        let dict_word = latin_dictionary.iter().find(|word| word.id == stem.wid);

        if let Some(latin_word) = dict_word {
            let word_is_in_out = output.iter().any(|w| match &w.word {
                Word::LatinWordInfo(latin_word_info) => {
                    latin_word_info.id == latin_word.id || latin_word_info.orth == latin_word.orth
                }
                Word::UniqueLatinWordInfo(unique_latin_word_info) => {
                    unique_latin_word_info.orth == latin_word.orth
                }
            });

            // if the word is already in the output add the stem to it
            if word_is_in_out {
                let matching_word = output.iter_mut().find(|w| match &w.word {
                    Word::LatinWordInfo(latin_word_info) => {
                        latin_word_info.id == latin_word.id
                            || latin_word_info.orth == latin_word.orth
                    }
                    Word::UniqueLatinWordInfo(unique_latin_word_info) => {
                        unique_latin_word_info.orth == latin_word.orth
                    }
                });

                if let Some(matching_word) = matching_word {
                    add_stem_to_word(stem, Some(matching_word));
                }
            } else {
                let new_inflections: Vec<Inflection>;
                if latin_word.pos == "V" || latin_word.pos == "VPAR" {
                    let fourth_part = latin_word.parts[3].as_str();
                    if fourth_part != stem.orth {
                        new_inflections = remove_extra_inflections(inflections.clone(), "VPAR");
                    } else {
                        new_inflections = remove_extra_inflections(inflections.clone(), "V");
                    }
                } else {
                    new_inflections = inflections.clone();
                }

                let mut new_word = Word::LatinWordInfo((*latin_word).clone());

                let next_word = latin_dictionary
                    .iter()
                    .find(|word| word.id == latin_word.id + 1);
                let next_word_senses = if let Some(next_word) = next_word {
                    next_word.senses.clone()
                } else {
                    Vec::new()
                };

                // senses starting with | also apply to word before
                if next_word_senses.len() > 0 && next_word_senses[0].trim().starts_with("|") {
                    let first_sense = next_word_senses[0].trim_start_matches("|").to_string();
                    let mut extension_senses = next_word_senses;
                    extension_senses[0] = first_sense;
                    if let Word::LatinWordInfo(latin_word_info) = &mut new_word {
                        latin_word_info.extension_senses = Some(extension_senses);
                    }
                }

                output.push(LatinTranslationInfo {
                    tricks: None,
                    word: new_word,
                    stem,
                    inflections: Some(new_inflections),
                    addon: None,
                });
            }
        }
    }

    output
}

fn add_stem_to_word(matched_stem: Stem, matching_word: Option<&mut LatinTranslationInfo>) {
    if let Some(word) = matching_word {
        let stem_to_add = &matched_stem.orth;
        let word_stems = &mut word.stem;

        if word_stems.orth != *stem_to_add {
            word_stems.orth = stem_to_add.to_string();
        }
    }
}

fn remove_extra_inflections(inflections: Vec<Inflection>, pos_to_remove: &str) -> Vec<Inflection> {
    let inflections = inflections
        .into_iter()
        .filter(|inflection| inflection.pos != pos_to_remove)
        .collect();

    inflections
}

fn reduce(latin_word: &mut String) -> (Vec<LatinTranslationInfo>, bool) {
    let mut modifiers: Vec<Modifier> = Vec::new();
    let latin_prefixes = get_latin_prefixes();
    let latin_suffixes = get_latin_suffixes();
    let mut found = false;

    // cutting length instead of replace to avoid removing letters if 1 letter prefix/suffix
    for prefix in latin_prefixes {
        if latin_word.starts_with(&prefix.orth) {
            *latin_word = latin_word[prefix.orth.len()..].to_string();
            modifiers.push({
                Modifier {
                    orth: prefix.orth,
                    senses: prefix.senses,
                    pos: prefix.pos,
                    form: None,
                    modifier: Some("prefix".to_string()),
                }
            });
        }
    }

    for suffix in latin_suffixes {
        if latin_word.ends_with(&suffix.orth) {
            *latin_word = latin_word[..latin_word.len() - suffix.orth.len()].to_string();
            modifiers.push({
                Modifier {
                    orth: suffix.orth,
                    senses: suffix.senses,
                    pos: suffix.pos,
                    form: None,
                    modifier: Some("suffix".to_string()),
                }
            });
        }
    }

    let mut output = find_form(latin_word, true);

    for word in &mut output {
        if let Word::LatinWordInfo(latin_word_info) = &mut word.word {
            latin_word_info.modifiers = Some(modifiers.clone());
        }
    }

    for word in &output {
        if word.stem != Stem::new() {
            found = true;
        }
    }
    if found == false {
        return (output, false);
    }

    (output, true)
}

fn split_enclitic(latin_word: &str) -> (String, Vec<Modifier>) {
    // using vector to make life easier
    let mut modifier: Vec<Modifier> = Vec::new();
    let latin_not_packons = get_latin_not_packons();
    let mut split_word = latin_word.to_string();
    let latin_tackons = get_latin_tackons();
    let latin_packons = get_latin_packons();
    let mut tackon = Attachment::new();

    for enclitic in latin_tackons {
        if latin_word.ends_with(&enclitic.orth) {
            tackon = enclitic;
            break;
        }
    }

    if tackon != Attachment::new() {
        // Est exception
        if latin_word != "est" {
            modifier.push(Modifier {
                orth: tackon.orth.clone(),
                senses: tackon.senses,
                pos: tackon.pos,
                form: None,
                modifier: Some("enclitic tackon".to_string()),
            });
        }

        split_word.truncate(split_word.len() - tackon.orth.len());
    } else {
        if latin_word.starts_with("qu") {
            for packon in latin_packons {
                if latin_word.ends_with(&packon.orth) {
                    modifier.push(Modifier {
                        orth: packon.orth.clone(),
                        senses: packon.senses,
                        pos: packon.pos,
                        form: None,
                        modifier: Some("enclitic packon".to_string()),
                    });

                    split_word.truncate(split_word.len() - packon.orth.len());
                }
            }
        } else {
            for not_packon in latin_not_packons {
                if latin_word.ends_with(&not_packon.orth) {
                    modifier.push(Modifier {
                        orth: not_packon.orth.clone(),
                        senses: not_packon.senses,
                        pos: not_packon.pos,
                        form: None,
                        modifier: Some("enclitic not packon".to_string()),
                    });

                    split_word.truncate(split_word.len() - not_packon.orth.len());
                }
            }
        }
    }
    (split_word, modifier)
}
