use crate::data::data::{Form, LatinWordInfo};
use crate::formatter::formatter::format_output;
use crate::formatter::prettify_output::{prettify_output, PrettifiedOutput};
use crate::latin_to_english::Word;
use crate::utils::principle_part_generator::{generate_for_nouns, generate_for_verbs};
use crate::{Language, Translation, TranslationType};

//TODO: add sorting by freq here

pub fn post_process(
    translations: Vec<Translation>,
    language: Language,
    formatted_output: bool,
    clean: bool,
    pretty_output: bool,
) {
    let translations = match language {
        Language::Latin => {
            latin_translation_output_post_processing(translations, formatted_output, clean)
        }
        Language::English => {
            english_translation_output_post_processing(translations, formatted_output, clean)
        }
    };

    if pretty_output {
        print_pretty_output(translations);
    } else {
        print_output(translations);
    }
}

fn latin_translation_output_post_processing(
    mut translations: Vec<Translation>,
    formatted_output: bool,
    clean: bool,
) -> Vec<Translation> {
    for translation in translations.iter_mut() {
        if let TranslationType::Latin(definitions) = &mut translation.definitions {
            for definition in definitions.iter_mut() {
                if let Word::LatinWordInfo(latin_word_info) = &mut definition.word {
                    let word_with_parts = add_principle_parts(latin_word_info.clone());
                    definition.word = Word::LatinWordInfo(word_with_parts);
                }
            }
        }
    }

    if formatted_output {
        translations = format_output(translations, Language::Latin, clean);
    }

    translations
}

fn english_translation_output_post_processing(
    mut translations: Vec<Translation>,
    formatted_output: bool,
    clean: bool,
) -> Vec<Translation> {
    for translation in translations.iter_mut() {
        if let TranslationType::English(definitions) = &mut translation.definitions {
            for definition in definitions.iter_mut() {
                let latin_word_info = &mut definition.translation;
                let word_with_parts = add_principle_parts(latin_word_info.clone());
                definition.translation = word_with_parts;
            }
        }
    }

    if formatted_output {
        translations = format_output(translations, Language::English, clean);
    }

    translations
}

fn add_principle_parts(mut latin_word_info: LatinWordInfo) -> LatinWordInfo {
    let pos = latin_word_info.pos.clone();
    let number_type = latin_word_info.n.clone();
    let principle_parts = latin_word_info.parts.clone();
    let gender = get_gender(latin_word_info.form.clone());

    let principle_parts: Vec<String> = match pos.as_str() {
        "N" => generate_for_nouns(number_type, gender, principle_parts),
        "V" => generate_for_verbs(number_type, principle_parts),
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

fn print_output(translations: Vec<Translation>) {
    let json_output = serde_json::to_string_pretty(&translations).unwrap();
    println!("{}", json_output);
}

fn print_pretty_output(translations: Vec<Translation>) {
    let pretty_output: Vec<PrettifiedOutput> = translations
        .into_iter()
        .map(|t| prettify_output(t.clone(), t.word.clone()))
        .collect();

    for output in &pretty_output {
        println!("{}\n", output.searched_word);
        for definition in &output.definitions {
            println!("{}", definition.orth_info);
            println!("{}", definition.form_info);

            if !definition.inflections.is_empty() {
                for inflection in &definition.inflections {
                    println!("{}", inflection);
                }
            }

            println!("{}", definition.senses);
            println!("---------------------------------");
        }
    }
}
