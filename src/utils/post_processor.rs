use crate::data::data::{Form, LatinWordInfo};
use crate::formatter::formatter::format_output;
use crate::latin_to_english::Word;
use crate::utils::filter::{entry_is_vague, filter_inflections};
use crate::utils::principle_part_generator::{
    generate_for_adjectives, generate_for_nouns, generate_for_numerals, generate_for_pronouns,
    generate_for_verbs, Comparison, NumeralType, VerbType,
};
use crate::{Language, Translation, TranslationType};

use super::sorter::sort_output;

pub fn post_process(
    translations: Vec<Translation>,
    language: Language,
    max: usize,
    formatted_output: bool,
    clean: bool,
    sort: bool,
    filter_uncommon: bool,
) -> Vec<Translation> {
    let mut translations = match language {
        Language::Latin => {
            let sorted_translations = if sort {
                sort_output(translations)
            } else {
                translations
            };
            latin_translation_output_post_processing(
                sorted_translations,
                clean,
                filter_uncommon,
                max,
            )
        }
        Language::English => english_translation_output_post_processing(translations),
    };

    if formatted_output {
        translations = format_output(translations, language, clean);
    }


    translations
}

fn latin_translation_output_post_processing(
    mut translations: Vec<Translation>,
    clean: bool,
    mut filter_uncommon: bool,
    max: usize,
) -> Vec<Translation> {
    let mut translation_length = translations.len();

    translations
        .iter_mut()
        .filter_map(|translation| {
            if let TranslationType::Latin(definitions) = &mut translation.definitions {
                let mut modified_definitions = Vec::new();

                if max > 0 && definitions.len() > max {
                    definitions.truncate(max);
                }

                if definitions.len() <= 1 {
                    // prevents accidental filtering of definitions with no alternatives
                    filter_uncommon = false;
                }

                for definition in definitions.iter_mut() {
                    if let Word::LatinWordInfo(latin_word_info) = &mut definition.word {
                        let vague = entry_is_vague(latin_word_info.clone(), clean, filter_uncommon);
                        let pos = latin_word_info.pos.clone();
                        let word_with_parts = add_principle_parts(latin_word_info.clone());
                        definition.word = Word::LatinWordInfo(word_with_parts);

                        let inflections = &definition.inflections;
                        if inflections.is_some() {
                            let inflections = inflections.clone().unwrap();
                            let filtered_inflections = filter_inflections(inflections, pos, clean);

                            definition.inflections = Some(filtered_inflections);
                        }

                        // !!!: issue here, keep track of length of output, if output length drops bellow 0, dont allow
                        if vague && (clean || filter_uncommon) && translation_length - 1 > 0 {
                            translation_length -= 1;
                            continue;
                        } else {
                            modified_definitions.push(definition.clone());
                        }
                    } else {
                        // Unique words
                        modified_definitions.push(definition.clone());
                    }
                }

                if !modified_definitions.is_empty() {
                    return Some(Translation {
                        definitions: TranslationType::Latin(modified_definitions),
                        ..translation.clone()
                    });
                }
            }

            None
        })
        .collect()
}

fn english_translation_output_post_processing(
    mut translations: Vec<Translation>,
) -> Vec<Translation> {
    for translation in translations.iter_mut() {
        if let TranslationType::English(definitions) = &mut translation.definitions {
            for definition in definitions.iter_mut() {
                let latin_word_info = &mut definition.translation;
                if latin_word_info.parts.is_empty() {
                    continue;
                }
                let word_with_parts = add_principle_parts(latin_word_info.clone());
                definition.translation = word_with_parts;
            }
        }
    }

    translations
}

fn add_principle_parts(mut latin_word_info: LatinWordInfo) -> LatinWordInfo {
    let pos = latin_word_info.pos.clone();
    let number_type = latin_word_info.n.clone();
    let principle_parts = latin_word_info.parts.clone();
    let gender = get_gender(latin_word_info.form.clone());
    let form = latin_word_info.form.clone();
    let word_type = match &form {
        Form::LongForm(_form) => "cqb13",
        Form::StrForm(form) => {
            let form_array = form.split_whitespace().collect::<Vec<&str>>();
            if form_array.len() < 2 {
                "cqb13"
            } else {
                form_array[2]
            }
        }
    };

    let principle_parts: Vec<String> = match pos.as_str() {
        "N" => generate_for_nouns(number_type.clone().unwrap(), gender, principle_parts),
        "V" => generate_for_verbs(
            number_type.clone().unwrap(),
            principle_parts,
            VerbType::from_str(word_type),
        ),
        "ADJ" => generate_for_adjectives(
            number_type.clone().unwrap(),
            principle_parts,
            Comparison::from_str(word_type),
        ),
        "PRON" => generate_for_pronouns(number_type.clone().unwrap(), principle_parts),
        "NUM" => generate_for_numerals(
            number_type.clone().unwrap(),
            principle_parts,
            NumeralType::from_str(word_type),
        ),
        _ => principle_parts,
    };

    latin_word_info.parts = principle_parts.clone();
    latin_word_info.orth = principle_parts.get(0).cloned().unwrap_or_default();

    latin_word_info
}

fn get_gender(form: Form) -> String {
    let form = match form {
        Form::LongForm(_form) => return "cqb13".to_string(),
        Form::StrForm(form) => form,
    };

    let form_array = form.split_whitespace().collect::<Vec<&str>>();

    if form_array.len() < 2 {
        return "cqb13".to_string();
    }

    form_array[2].to_string()
}
