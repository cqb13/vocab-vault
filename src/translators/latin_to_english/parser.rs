use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::{Inflection, LatinWordInfo, NValue, Stem};
use crate::translators::latin_to_english::translator::lookup_stems;
use crate::translators::latin_to_english::tricks::{try_medieval_tricks, TrickResult};
use crate::translators::latin_to_english::utils::reduce;
use crate::translators::latin_to_english::LatinTranslationInfo;
use crate::utils::data::{get_latin_inflections, get_latin_stems, get_unique_latin_words};
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

fn get_stem_map() -> &'static HashMap<String, Vec<&'static Stem>> {
    static MAP: OnceLock<HashMap<String, Vec<&'static Stem>>> = OnceLock::new();
    MAP.get_or_init(|| {
        let mut map: HashMap<String, Vec<&'static Stem>> = HashMap::new();
        for stem in get_latin_stems().iter() {
            map.entry(stem.orth.clone()).or_default().push(stem);
        }
        map
    })
}

pub fn parse(latin_word: &str, reduced: bool) -> Option<Vec<LatinTranslationInfo>> {
    match find_form(latin_word, reduced) {
        Some(form) => return Some(form),
        None => match parse_unique_latin_words(latin_word) {
            Some(unique_word) => {
                let mut translation = LatinTranslationInfo::new();
                translation.word = unique_word;
                return Some(vec![translation]);
            }
            None => None,
        },
    }
}

fn parse_unique_latin_words(latin_word: &str) -> Option<LatinWordInfo> {
    let unique_words = get_unique_latin_words();

    let latin_word_lower = latin_word.to_lowercase();
    unique_words
        .iter()
        .find(|unique_word| unique_word.orth.to_lowercase() == latin_word_lower)
        .cloned()
}

pub fn find_form(latin_word: &str, reduced: bool) -> Option<Vec<LatinTranslationInfo>> {
    let latin_inflections = get_latin_inflections();
    let mut latin_word_inflections: Vec<Inflection> = Vec::new();

    for inflection in latin_inflections.iter() {
        if latin_word.ends_with(inflection.ending.as_str()) {
            // if the longest inflection has been found, stop looking
            if latin_word_inflections.len() > 0
                && latin_word_inflections[0].ending.len() > inflection.ending.len()
            {
                break;
            }
            latin_word_inflections.push(inflection.clone());
        }
    }

    let (stems, inflections) = check_stems(latin_word, &latin_word_inflections, false);
    let mut output = lookup_stems(stems, inflections);

    if output.is_none() && !reduced {
        output = reduce(latin_word);
    }

    //curebantur -> currebantur (needs to work on stem or word: cureb -> curreb)
    if output.is_none() {
        let (stems, inflections) = check_stems(latin_word, &latin_word_inflections, true);
        output = lookup_stems(stems, inflections);
    }

    output
}

fn check_stems(
    latin_word: &str,
    latin_word_inflections: &[Inflection],
    tricks: bool,
) -> (Vec<Stem>, Vec<Inflection>) {
    let stem_map = get_stem_map();
    let mut matched_stems: Vec<Stem> = Vec::new();
    let mut inflections: Vec<Inflection> = Vec::new();
    let mut found_inflection_forms: HashSet<String> = HashSet::new();

    for inflection in latin_word_inflections {
        let word_stem = latin_word.trim_end_matches(&inflection.ending);

        let word_stem = if tricks {
            let tricked = try_medieval_tricks(word_stem);
            match tricked {
                TrickResult::Found(word, _) => word,
                TrickResult::NotFound => word_stem.to_string(),
            }
        } else {
            word_stem.to_string()
        };

        let Some(stems) = stem_map.get(&word_stem) else {
            continue;
        };

        for stem in stems {
            if inflection.pos != stem.pos
                && !(inflection.pos == PartOfSpeech::Participle && stem.pos == PartOfSpeech::Verb)
                && !(inflection.pos == PartOfSpeech::Verb && stem.pos == PartOfSpeech::Participle)
            {
                continue;
            }

            let n_from_inflection = inflection.n.as_ref().expect("Inflection has no n value");
            let n_from_stem = stem.n.as_ref().expect("Stem has no n value");

            if n_from_stem.len() == 1 && n_from_stem[0] != n_from_inflection[0] {
                continue;
            }

            if n_from_stem.len() >= 2 {
                if n_from_inflection[0] != n_from_stem[0]
                    && n_from_inflection[0] != NValue::Integer(0)
                {
                    continue;
                }

                if n_from_inflection[1] != n_from_stem[1]
                    && n_from_inflection[1] != NValue::Integer(0)
                {
                    continue;
                }
            }

            let form_key = inflection.form.as_str();
            if !found_inflection_forms.insert(form_key) {
                continue;
            }

            for stem_inflection in &inflections {
                if stem_inflection.pos == inflection.pos
                    || (stem_inflection.pos == PartOfSpeech::Participle
                        && inflection.pos == PartOfSpeech::Verb)
                    || (stem_inflection.pos == PartOfSpeech::Verb
                        && inflection.pos == PartOfSpeech::Participle)
                {
                    break;
                }
            }
            matched_stems.push(Stem::clone(stem));
            inflections.push(inflection.clone());
        }
    }

    (matched_stems, inflections)
}
