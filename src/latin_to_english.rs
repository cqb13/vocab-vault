use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::data::{
    get_latin_dictionary, get_latin_inflections, get_latin_not_packons, get_latin_packons,
    get_latin_prefixes, get_latin_stems, get_latin_suffixes, get_latin_tackons,
    get_unique_latin_words, Attachment, Inflection, LatinWordInfo, Stem, UniqueLatinWordInfo,
};

use crate::utils::tricks::{evaluate_roman_numeral, is_roman_number, switch_first_i_or_j};

#[derive(Serialize, Deserialize, Debug)]
pub struct LatinTranslationInfo {
    pub word: Word,
    pub stems: Vec<StemMatch>,
    pub addon: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Word {
    LatinWordInfo(LatinWordInfo),
    UniqueLatinWordInfo(UniqueLatinWordInfo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StemMatch {
    pub stem: Stem,
    pub inflections: Vec<Inflection>,
}

pub fn translate_to_english(latin_word: &str) -> Vec<LatinTranslationInfo> {
    let mut output: Vec<LatinTranslationInfo> = Vec::new();

    let (unique_latin_word, found) = parse_unique_latin_words(latin_word);

    if found {
        output.push(LatinTranslationInfo {
            word: Word::UniqueLatinWordInfo(unique_latin_word),
            stems: Vec::new(),
            addon: "unique".to_string(),
        });
    }

    if output.len() == 0 {
        output = find_form(latin_word, false);
    }

    //instead of updating actual word, a copy is created that is switched, to not break splitEnclitic parsing.
    // Some words that start with i can also start with j
    // ex: iecit -> jecit
    // checking if return is word, because if word does not start with I or J, original word is returned, making the parsing not needed.
    if output.len() == 0 && switch_first_i_or_j(latin_word) != latin_word {
        output = find_form(&switch_first_i_or_j(latin_word), false);
    }

    // If nothing is found, try removing enclitics and try again
    // ex: clamaverunt -> clamare
    // doing this here instead of earlier should fix words like salve having the "ve" removed and returning wrong def
    if output.len() == 0 {
        let (split_word, enclitic_output) = split_enclitic(latin_word);
        output = enclitic_output;

        let (unique_latin_word, found) = parse_unique_latin_words(&split_word);
        if found {
            output.push(LatinTranslationInfo {
                word: Word::UniqueLatinWordInfo(unique_latin_word),
                stems: Vec::new(),
                addon: "".to_string(),
            });
        } else {
            output.append(&mut find_form(&split_word, false));

            if output.len() == 0 && switch_first_i_or_j(&split_word) != split_word {
                output.append(&mut find_form(&switch_first_i_or_j(&split_word), false));
            }
        }
    }

    if is_roman_number(latin_word) {
        let numeral_evaluation = evaluate_roman_numeral(latin_word);

        if numeral_evaluation > 0 {
            output.push(LatinTranslationInfo {
                word: Word::UniqueLatinWordInfo(UniqueLatinWordInfo {
                    orth: latin_word.to_string(),
                    senses: [numeral_evaluation.to_string()].to_vec(),
                    pos: "NUM".to_string(),
                    form: "NUM".to_string(),
                }),
                stems: Vec::new(),
                addon: "roman numeral".to_string(),
            });
        }
    }

    output
}

fn parse_unique_latin_words(latin_word: &str) -> (UniqueLatinWordInfo, bool) {
    let mut unique_latin_word = UniqueLatinWordInfo::new();
    let unique_words: Value = get_unique_latin_words();
    let mut found = false;
    for unique_word in unique_words.as_array().unwrap() {
        if unique_word["orth"]
            .as_str()
            .unwrap_or_default()
            .to_lowercase()
            == latin_word.to_lowercase()
        {
            unique_latin_word.orth = unique_word["orth"].as_str().unwrap_or_default().to_string();
            unique_latin_word.senses = unique_word["senses"]
                .as_array()
                .unwrap()
                .iter()
                .map(|sense| sense.as_str().unwrap_or_default().to_string())
                .collect();
            unique_latin_word.pos = unique_word["pos"].as_str().unwrap_or_default().to_string();
            unique_latin_word.form = unique_word["form"].as_str().unwrap_or_default().to_string();
            found = true;
            break;
        }
    }

    return (unique_latin_word, found);
}

fn find_form(latin_word: &str, reduced: bool) -> Vec<LatinTranslationInfo> {
    let mut latin_word_inflections: Vec<Inflection> = Vec::new();
    let latin_inflections: Value = get_latin_inflections();

    for inflection in latin_inflections.as_array().unwrap() {
        if latin_word.ends_with(inflection["ending"].as_str().unwrap_or_default()) {
            // if the longest inflection has been found, stop looking
            if latin_word_inflections.len() > 0
                && latin_word_inflections[0].ending.len()
                    > inflection["ending"].as_str().unwrap_or_default().len()
            {
                break;
            } else {
                latin_word_inflections.push(Inflection {
                    ending: inflection["ending"]
                        .as_str()
                        .unwrap_or_default()
                        .to_string(),
                    pos: inflection["pos"].as_str().unwrap_or_default().to_string(),
                    notes: inflection["notes"].as_str().unwrap_or_default().to_string(),
                    n: inflection["n"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|n| n.as_i64().unwrap_or_default() as i8)
                        .collect(),
                    form: inflection["form"].as_str().unwrap_or_default().to_string(),
                });
            }
        }
    }

    let mut matched_stems = check_stems(latin_word, &latin_word_inflections, true);
    // If no stems were found, try again without ensuring infls
    // allows for words that end with erunt and similar cases
    // ex: clamaverunt return null without this
    if matched_stems.len() == 0 {
        matched_stems = check_stems(latin_word, &latin_word_inflections, false);
    }

    let mut output = lookup_stems(matched_stems);

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
) -> Vec<StemMatch> {
    let latin_stems = get_latin_stems();
    let mut matched_stems: Vec<StemMatch> = Vec::new();

    for inflection in latin_word_inflections {
        let word_stem = latin_word.trim_end_matches(&inflection.ending);

        for stem in latin_stems.as_array().unwrap() {
            if word_stem == stem["orth"].as_str().unwrap_or_default() {
                if inflection.pos == stem["pos"].as_str().unwrap_or_default()
                    || (inflection.pos == "VPAR" && stem["pos"].as_str().unwrap_or_default() == "V")
                    || (inflection.pos == "V" && stem["pos"].as_str().unwrap_or_default() == "VPAR")
                {
                    if inflection.n[0] != stem["n"][0].as_i64().unwrap_or_default() as i8
                        && ensure_inflections
                    {
                        continue;
                    }

                    let mut is_matched_stem = false;

                    for matched_stem in &mut matched_stems {
                        if stem["form"] == matched_stem.stem.form {
                            is_matched_stem = true;
                            let mut in_stem_inflections = false;

                            for stem_inflection in &matched_stem.inflections {
                                if stem_inflection.form == inflection.form {
                                    in_stem_inflections = true;
                                    break;
                                }
                            }

                            if !in_stem_inflections {
                                matched_stem.inflections.push(inflection.clone());
                            }
                        }
                    }

                    if !is_matched_stem {
                        matched_stems.push(StemMatch {
                            stem: Stem {
                                pos: stem["pos"].as_str().unwrap_or_default().to_string(),
                                form: stem["form"].as_str().unwrap_or_default().to_string(),
                                orth: stem["orth"].as_str().unwrap_or_default().to_string(),
                                n: stem["n"]
                                    .as_array()
                                    .unwrap()
                                    .iter()
                                    .map(|n| n.as_i64().unwrap_or_default() as i8)
                                    .collect(),
                                wid: stem["wid"].as_i64().unwrap_or_default() as i32,
                            },
                            inflections: vec![inflection.clone()],
                        });
                    }
                }
            }
        }
    }

    matched_stems
}

fn lookup_stems(matched_stems: Vec<StemMatch>) -> Vec<LatinTranslationInfo> {
    let latin_dictionary = get_latin_dictionary();
    let mut output: Vec<LatinTranslationInfo> = Vec::new();

    for matched_stem in matched_stems {
        let dict_word = latin_dictionary
            .as_array()
            .unwrap()
            .iter()
            .find(|word| word["id"] == matched_stem.stem.wid);

        if let Some(latin_word) = dict_word {
            let word_is_in_out = output.iter().any(|w| match &w.word {
                Word::LatinWordInfo(latin_word_info) => {
                    latin_word_info.id == latin_word["id"].as_i64().unwrap_or_default() as i32
                        || latin_word_info.orth == latin_word["orth"].as_str().unwrap_or_default()
                }
                Word::UniqueLatinWordInfo(unique_latin_word_info) => {
                    unique_latin_word_info.orth == latin_word["orth"].as_str().unwrap_or_default()
                }
            });

            if word_is_in_out {
                let matching_word = output.iter_mut().find(|w| match &w.word {
                    Word::LatinWordInfo(latin_word_info) => {
                        latin_word_info.id == latin_word["id"].as_i64().unwrap_or_default() as i32
                            || latin_word_info.orth
                                == latin_word["orth"].as_str().unwrap_or_default()
                    }
                    Word::UniqueLatinWordInfo(unique_latin_word_info) => {
                        unique_latin_word_info.orth
                            == latin_word["orth"].as_str().unwrap_or_default()
                    }
                });

                if let Some(matching_word) = matching_word {
                    add_stem_to_word(matched_stem, Some(matching_word));
                }
            } else {
                let temp_stem;
                if latin_word["pos"] == "V" {
                    let fourth_part = latin_word["parts"][3].as_str().unwrap_or_default();
                    if fourth_part != matched_stem.stem.orth {
                        temp_stem = remove_extra_inflections(matched_stem, "VPAR");
                    } else {
                        temp_stem = remove_extra_inflections(matched_stem, "V");
                    }
                } else {
                    temp_stem = matched_stem;
                }

                let new_word = Word::LatinWordInfo(LatinWordInfo {
                    pos: latin_word["pos"].as_str().unwrap_or_default().to_string(),
                    n: latin_word["n"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|n| n.as_i64().unwrap_or_default() as i8)
                        .collect(),
                    parts: latin_word["parts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|part| part.as_str().unwrap_or_default().to_string())
                        .collect(),
                    senses: latin_word["senses"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|sense| sense.as_str().unwrap_or_default().to_string())
                        .collect(),
                    form: latin_word["form"].as_str().unwrap_or_default().to_string(),
                    orth: latin_word["orth"].as_str().unwrap_or_default().to_string(),
                    id: latin_word["id"].as_i64().unwrap_or_default() as i32,
                });

                output.push(LatinTranslationInfo {
                    word: new_word,
                    stems: vec![temp_stem],
                    addon: "".to_string(),
                });
            }
        }
    }

    output
}

fn add_stem_to_word(matched_stem: StemMatch, matching_word: Option<&mut LatinTranslationInfo>) {
    if let Some(word) = matching_word {
        let stem_to_add = &matched_stem.stem.orth;
        let word_stems = &mut word.stems;

        if !word_stems.iter().any(|st| st.stem.orth == *stem_to_add) {
            word_stems.push(matched_stem);
        }
    }
}

fn remove_extra_inflections(stem_match: StemMatch, pos_to_remove: &str) -> StemMatch {
    let inflections = stem_match
        .inflections
        .into_iter()
        .filter(|inf| inf.pos != pos_to_remove)
        .collect();
    StemMatch {
        stem: stem_match.stem,
        inflections,
    }
}

fn reduce(latin_word: &mut String) -> (Vec<LatinTranslationInfo>, bool) {
    let mut output: Vec<LatinTranslationInfo> = Vec::new();
    let latin_prefixes = get_latin_prefixes();
    let latin_suffixes = get_latin_suffixes();
    let mut found = false;

    for prefix in latin_prefixes {
        if latin_word.starts_with(&prefix.orth) {
            *latin_word = latin_word.replace(&prefix.orth, "");
            output.push({
                LatinTranslationInfo {
                    word: Word::UniqueLatinWordInfo(UniqueLatinWordInfo {
                        orth: prefix.orth,
                        senses: Vec::new(),
                        pos: prefix.pos,
                        form: prefix.form,
                    }),
                    stems: Vec::new(),
                    addon: "prefix".to_string(),
                }
            });
        }
    }

    for suffix in latin_suffixes {
        if latin_word.ends_with(&suffix.orth) {
            *latin_word = latin_word.replace(&suffix.orth, "");
            output.push({
                LatinTranslationInfo {
                    word: Word::UniqueLatinWordInfo(UniqueLatinWordInfo {
                        orth: suffix.orth,
                        senses: Vec::new(),
                        pos: suffix.pos,
                        form: suffix.form,
                    }),
                    stems: Vec::new(),
                    addon: "suffix".to_string(),
                }
            });
        }
    }

    output = find_form(latin_word, true);

    for word in &output {
        if word.stems.len() > 0 {
            found = true;
        }
    }
    if found == false {
        return (output, false);
    }

    (output, true)
}

fn split_enclitic(latin_word: &str) -> (String, Vec<LatinTranslationInfo>) {
    let latin_tackons = get_latin_tackons();
    let latin_packons = get_latin_packons();
    let latin_not_packons = get_latin_not_packons();
    let mut tackon = Attachment::new();
    let mut output: Vec<LatinTranslationInfo> = Vec::new();
    let mut split_word = latin_word.to_string();

    for enclitic in latin_tackons {
        if latin_word.ends_with(&enclitic.orth) {
            tackon = enclitic;
            break;
        }
    }

    if tackon != Attachment::new() {
        // Est exception
        if latin_word != "est" {
            output.push({
                LatinTranslationInfo {
                    word: Word::UniqueLatinWordInfo(UniqueLatinWordInfo {
                        orth: tackon.orth.clone(),
                        senses: tackon.senses,
                        pos: tackon.pos,
                        form: tackon.orth.clone(),
                    }),
                    stems: Vec::new(),
                    addon: "tackon".to_string(),
                }
            });
        }

        split_word.truncate(split_word.len() - tackon.orth.len());
    } else {
        if latin_word.starts_with("qu") {
            for packon in latin_packons {
                if latin_word.ends_with(&packon.orth) {
                    output.push({
                        LatinTranslationInfo {
                            word: Word::UniqueLatinWordInfo(UniqueLatinWordInfo {
                                orth: packon.orth.clone(),
                                senses: packon.senses,
                                pos: packon.pos,
                                form: "".to_string(),
                            }),
                            stems: Vec::new(),
                            addon: "packon".to_string(),
                        }
                    });
                }

                split_word.truncate(split_word.len() - packon.orth.len());
            }
        } else {
            for not_packon in latin_not_packons {
                if latin_word.ends_with(&not_packon.orth) {
                    output.push({
                        LatinTranslationInfo {
                            word: Word::UniqueLatinWordInfo(UniqueLatinWordInfo {
                                orth: not_packon.orth.clone(),
                                senses: not_packon.senses,
                                pos: not_packon.pos,
                                form: "".to_string(),
                            }),
                            stems: Vec::new(),
                            addon: "not packon".to_string(),
                        }
                    });
                }

                split_word.truncate(split_word.len() - not_packon.orth.len());
            }
        }
    }

    (split_word, output)
}
