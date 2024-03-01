use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::{Inflection, LatinWordInfo, NValue, Stem};
use crate::translators::latin_to_english::translator::lookup_stems;
use crate::translators::latin_to_english::utils::reduce;
use crate::translators::latin_to_english::LatinTranslationInfo;
use crate::utils::data::{get_latin_inflections, get_latin_stems, get_unique_latin_words};

pub fn parse(latin_word: &str, reduced: bool) -> Option<Vec<LatinTranslationInfo>> {
    match parse_unique_latin_words(latin_word) {
        Some(unique_word) => {
            let mut translation = LatinTranslationInfo::new();
            translation.word = unique_word;
            return Some(vec![translation]);
        }
        None => (),
    }

    find_form(latin_word, reduced)
}

fn parse_unique_latin_words(latin_word: &str) -> Option<LatinWordInfo> {
    let unique_words = get_unique_latin_words();

    let latin_word_lower = latin_word.to_lowercase();
    unique_words
        .into_iter()
        .find(|unique_word| unique_word.orth.to_lowercase() == latin_word_lower)
}

pub fn find_form(latin_word: &str, reduced: bool) -> Option<Vec<LatinTranslationInfo>> {
    let latin_inflections: Vec<Inflection> = get_latin_inflections();
    let mut latin_word_inflections: Vec<Inflection> = Vec::new();

    for inflection in latin_inflections {
        if latin_word.ends_with(inflection.ending.as_str()) {
            // if the longest inflection has been found, stop looking
            if latin_word_inflections.len() > 0
                && latin_word_inflections[0].ending.len() > inflection.ending.len()
            {
                break;
            }
            latin_word_inflections.push(inflection);
        }
    }

    //TODO: curebantur -> currebantur (needs to work on stem or word: curo -> curro)
    let (stems, inflections) = check_stems(latin_word, &latin_word_inflections);
    let mut output = lookup_stems(stems, inflections);

    if output.is_none() && !reduced {
        output = reduce(latin_word);
    }

    output
}

//??? weird issue, when word searched alone, 1 result but when with others, different result ("cur" vs "cur sum hic")
fn check_stems(
    latin_word: &str,
    latin_word_inflections: &Vec<Inflection>,
) -> (Vec<Stem>, Vec<Inflection>) {
    let latin_stems = get_latin_stems();
    let mut matched_stems: Vec<Stem> = Vec::new();
    let mut inflections: Vec<Inflection> = Vec::new();
    let mut found_inflection_forms: Vec<String> = Vec::new();

    for inflection in latin_word_inflections {
        let word_stem = latin_word.trim_end_matches(&inflection.ending);

        for stem in &latin_stems {
            if word_stem == stem.orth {
                if inflection.pos == stem.pos
                    || (inflection.pos == PartOfSpeech::Participle
                        && stem.pos == PartOfSpeech::Verb)
                    || (inflection.pos == PartOfSpeech::Verb
                        && stem.pos == PartOfSpeech::Participle)
                {
                    let n_from_inflection = match &inflection.n {
                        Some(n) => n,
                        None => {
                            println!("Inflection has no n value");
                            std::process::exit(0);
                        }
                    };
                    let n_from_stem = match &stem.n {
                        Some(n) => n,
                        None => {
                            println!("Stem has no n value");
                            std::process::exit(0);
                        }
                    };

                    //???: Weird issue here where some words get inflections by should not (cur)
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

                    if found_inflection_forms.contains(&inflection.form.as_str()) {
                        continue;
                    }

                    found_inflection_forms.push(inflection.form.as_str());

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
                    matched_stems.push(stem.clone());
                    inflections.push(inflection.clone());
                }
            }
        }
    }

    (matched_stems, inflections)
}
