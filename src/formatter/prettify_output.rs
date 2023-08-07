use crate::{Translation, TranslationType};
use crate::data::data::{Form, LongForm, Stem, Inflection, LatinWordInfo};
use crate::latin_to_english::{Word, LatinTranslationInfo};

pub struct PrettifiedOutput {
    pub searched_word: String,
    pub definitions: Vec<PrettifiedDefinition>,
}

impl PrettifiedOutput {
    pub fn new(searched_word: String, definitions: Vec<PrettifiedDefinition>) -> Self {
        Self {
            searched_word,
            definitions,
        }
    }
}

pub struct PrettifiedDefinition {
    pub orth_info: String,
    pub form_info: String,
    pub inflections: Vec<String>,
    pub senses: String,
}

pub fn prettify_output(translation: Translation, search_word: String) -> PrettifiedOutput {
    let mut prettified_output = PrettifiedOutput::new(search_word, vec![]);

    match translation.definitions {
        TranslationType::Latin(definitions) => {
            for definition in definitions {
                let prettified_definition = create_pretty_latin_definition(definition);
                prettified_output.definitions.push(prettified_definition);
            }
        }
        TranslationType::English(definitions) => {
            for definition in definitions {
                let prettified_definition = create_pretty_english_definition(definition.translation);
                prettified_output.definitions.push(prettified_definition);
            }
        }
    }

    prettified_output
}

fn create_pretty_english_definition(latin_word_info: LatinWordInfo) -> PrettifiedDefinition {
    let mut output = PrettifiedDefinition {
        orth_info: "".to_string(),
        form_info: "".to_string(),
        inflections: vec![],
        senses: "".to_string(),
    };

    let parts_info = latin_word_info.parts.iter().map(ToString::to_string).collect::<Vec<String>>().join(" ");
    let senses_info = latin_word_info.senses.iter().map(ToString::to_string).collect::<Vec<String>>().join(" | ");

    output.orth_info = parts_info;
    output.form_info = convert_form_to_string(latin_word_info.form);
    output.senses = senses_info;

    output
}

fn create_pretty_latin_definition(latin_translation_info: LatinTranslationInfo) -> PrettifiedDefinition {
    let mut output = PrettifiedDefinition {
        orth_info: "".to_string(),
        form_info: "".to_string(),
        inflections: vec![],
        senses: "".to_string(),
    };

    let latin_word_info = latin_translation_info.word;

    match latin_word_info {
        Word::LatinWordInfo(latin_word_info) => {
            let parts_info = latin_word_info.parts.iter().map(ToString::to_string).collect::<Vec<String>>().join(" ");
            let senses_info = latin_word_info.senses.iter().map(ToString::to_string).collect::<Vec<String>>().join(" | ");
    
            output.orth_info = parts_info;
            output.form_info = convert_form_to_string(latin_word_info.form);
            output.senses = senses_info;
        }
        Word::UniqueLatinWordInfo(unique_latin_word_info) => {
            output.orth_info = unique_latin_word_info.orth;
            output.form_info = convert_form_to_string(unique_latin_word_info.form);
            output.senses = unique_latin_word_info.senses.iter().map(ToString::to_string).collect::<Vec<String>>().join(" | ");
        }
    }
    

    output.inflections = make_inflection_string(latin_translation_info.stem, latin_translation_info.inflections);

    output
}

fn convert_form_to_string(form: Form) -> String {
    match form {
        Form::StrForm(form) => form,
        Form::LongForm(form) => LongForm::long_form_to_string(form),
    }
}

fn make_inflection_string(stem: Stem, inflections: Vec<Inflection>) -> Vec<String> {
    let mut string_inflection: Vec<String> = vec![];
    let stem = stem.orth.to_string();

    for inflection in inflections {
        let mut inflection_string = stem.clone() + "." + inflection.ending.as_str();
        inflection_string = inflection_string + " | " + convert_form_to_string(inflection.form).as_str();
        inflection_string = inflection_string + " | " + inflection.pos.as_str();
        string_inflection.push(inflection_string);
    }

    string_inflection
}