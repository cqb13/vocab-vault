use serde::{Deserialize, Serialize, Serializer};

use crate::data::data::{
    get_latin_dictionary, get_latin_inflections, get_latin_not_packons, get_latin_packons,
    get_latin_prefixes, get_latin_stems, get_latin_suffixes, get_latin_tackons,
    get_unique_latin_words, Attachment, Form, Inflection, LatinWordInfo, Modifier, Stem,
    UniqueLatinWordInfo, WordInfo,
};

use crate::tricks::tricks::{evaluate_roman_numeral, is_roman_number, try_tricks};
use crate::tricks::word_mods::switch_first_i_or_j;

#[derive(Serialize, Deserialize, Debug)]
pub struct LatinTranslationInfo {
    pub tricks: Option<Vec<String>>,
    #[serde(serialize_with = "serialize_word")]
    pub word: Word,
    pub stem: Stem,
    pub inflections: Vec<Inflection>,
    pub addon: String,
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

    if output.len() == 0 && tricks {
        let (modified_latin_word, trick_explanations) = try_tricks(latin_word.clone());
        output = parse(&modified_latin_word, false);
        if output.len() > 0 {
            for entry in &mut output {
                entry.tricks = trick_explanations.clone();
            }
        }
    }

    // most words should be found by now

    //instead of updating actual word, a copy is created that is switched, to not break splitEnclitic parsing.
    // Some words that start with i can also start with j
    // ex: iecit -> jecit
    // checking if return is word, because if word does not start with I or J, original word is returned, making the parsing not needed.
    if output.len() == 0 {
        let (switched_word, trick_explanation) = switch_first_i_or_j(&latin_word);
        if switched_word != latin_word {
            output = parse(&switched_word, false);
            if output.len() > 0 {
                for entry in &mut output {
                    entry.tricks = trick_explanation.clone();
                }
            }
        }
    }

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
                    orth: latin_word.to_string(),
                    senses: [numeral_evaluation.to_string()].to_vec(),
                    pos: "NUM".to_string(),
                    form: Form::StrForm("NUM".to_string()),
                    n: Vec::new(),
                    info: WordInfo::new_set(
                        "X".to_string(),
                        "T".to_string(),
                        "I".to_string(),
                        "C".to_string(),
                        "X".to_string(),
                    ),
                }),
                stem: Stem::new(),
                inflections: Vec::new(),
                addon: "roman numeral".to_string(),
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
            inflections: Vec::new(),
            addon: "unique".to_string(),
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
    if stems.len() == 0 {
        (stems, inflections) = check_stems(latin_word, &latin_word_inflections, false);
    }

    let mut output = lookup_stems(stems, inflections);

    if output.len() == 0 && !reduced {
        let (mut reduced_output, found) = reduce(&mut latin_word.to_string());
        if found {
            output.append(&mut reduced_output);
        }
    }

    output
}

/**
 * For each inflection that was a match, remove the inflection from
 * the end of the word string and then check the resulting stem
 * against the list of stems from stemList.ts
 */
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
                    if inflection.n[0] != stem.n[0] && ensure_inflections {
                        continue;
                    }

                    let mut is_matched_stem = false;

                    for matched_stem in &matched_stems {
                        if stem.form == matched_stem.form {
                            is_matched_stem = true;
                            let mut in_stem_inflections = false;
                            for stem_inflection in &inflections {
                                if stem_inflection.form == inflection.form {
                                    in_stem_inflections = true;
                                    break;
                                }
                            }
                            if !in_stem_inflections {
                                inflections.push(inflection.clone());
                            }
                        }
                    }

                    for stem_inflection in &inflections {
                        if stem_inflection.form == inflection.form {
                            is_matched_stem = true;
                            break;
                        }
                    }

                    if !is_matched_stem {
                        matched_stems.push(stem.clone());
                        inflections.push(inflection.clone());
                    }
                }
            }
        }
    }

    (matched_stems, inflections)
}

//TODO: a new prop needs to be added to LatinWordInfo:
/*
senses and extension senses. extension senses are senses that start with a "|", they are found in the next word in the dictionary.
when adding a word, check the next word, if the senses start with a "|", assign that sense to the extension_senses prop.
*/
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
                if latin_word.pos == "V" {
                    let fourth_part = latin_word.parts[3].as_str();
                    if fourth_part != stem.orth {
                        let inflections_clone = inflections.clone();
                        new_inflections = remove_extra_inflections(inflections_clone, "VPAR");
                    } else {
                        new_inflections = remove_extra_inflections(inflections.clone(), "V");
                    }
                } else {
                    new_inflections = inflections.clone();
                }

                let new_word = Word::LatinWordInfo((*latin_word).clone());

                output.push(LatinTranslationInfo {
                    tricks: None,
                    word: new_word,
                    stem,
                    inflections: new_inflections,
                    addon: "".to_string(),
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
                    form: prefix.form,
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
                    form: suffix.form,
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

    //TODO: add a way to tell what kind of enclitic it is
    if tackon != Attachment::new() {
        // Est exception
        if latin_word != "est" {
            modifier.push(Modifier {
                orth: tackon.orth.clone(),
                senses: tackon.senses,
                pos: tackon.pos,
                form: Form::StrForm(tackon.orth.clone()),
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
                        form: Form::StrForm("".to_string()),
                        modifier: Some("enclitic packon".to_string()),
                    });
                }

                split_word.truncate(split_word.len() - packon.orth.len());
            }
        } else {
            for not_packon in latin_not_packons {
                if latin_word.ends_with(&not_packon.orth) {
                    modifier.push(Modifier {
                        orth: not_packon.orth.clone(),
                        senses: not_packon.senses,
                        pos: not_packon.pos,
                        form: Form::StrForm("".to_string()),
                        modifier: Some("enclitic not packon".to_string()),
                    });
                }

                split_word.truncate(split_word.len() - not_packon.orth.len());
            }
        }
    }
    (split_word, modifier)
}
