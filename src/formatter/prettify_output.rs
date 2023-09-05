use crate::data::data::{Form, Inflection, LatinWordInfo, LongForm, Stem, WordInfo};
use crate::latin_to_english::{LatinTranslationInfo, Word};
use crate::{Translation, TranslationType};

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

impl PartialEq for PrettifiedOutput {
    fn eq(&self, other: &Self) -> bool {
        self.searched_word == other.searched_word && self.definitions == other.definitions
    }
}

pub struct PrettifiedDefinition {
    pub tricks: Option<Vec<String>>,
    pub orth_info: String,
    pub form_info: String,
    pub inflections: Vec<String>,
    pub details: String,
    pub senses: String,
    pub modifiers: Vec<ModifierAttachment>,
}

impl PartialEq for PrettifiedDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.tricks == other.tricks
            && self.orth_info == other.orth_info
            && self.form_info == other.form_info
            && self.inflections == other.inflections
            && self.details == other.details
            && self.senses == other.senses
            && self.modifiers == other.modifiers
    }
}

pub struct ModifierAttachment {
    pub pos: String,
    pub senses: String,
    pub orth: String,
    pub modifier: String,
}

impl PartialEq for ModifierAttachment {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
            && self.senses == other.senses
            && self.orth == other.orth
            && self.modifier == other.modifier
    }
}

pub fn prettify_output(translation: Translation, search_word: String) -> PrettifiedOutput {
    let mut prettified_output = PrettifiedOutput::new(search_word, vec![]);

    match translation.definitions {
        TranslationType::Latin(definitions) => {
            for definition in definitions {
                let copied_definition = definition.clone();
                let mut prettified_definition = create_pretty_latin_definition(definition);
                let tricks = copied_definition.tricks;
                prettified_definition.tricks = tricks;
                prettified_output.definitions.push(prettified_definition);
            }
        }
        TranslationType::English(definitions) => {
            for definition in definitions {
                let prettified_definition =
                    create_pretty_english_definition(definition.translation);
                prettified_output.definitions.push(prettified_definition);
            }
        }
    }

    prettified_output
}

fn create_pretty_english_definition(latin_word_info: LatinWordInfo) -> PrettifiedDefinition {
    let mut output = PrettifiedDefinition {
        tricks: None,
        orth_info: "".to_string(),
        form_info: "".to_string(),
        inflections: vec![],
        details: "".to_string(),
        senses: "".to_string(),
        modifiers: vec![],
    };

    let parts_info = latin_word_info
        .parts
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(" ");
    let senses_info = latin_word_info
        .senses
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>()
        .join(" | ");

    output.orth_info = parts_info;
    output.form_info = convert_form_to_string(latin_word_info.form);
    output.details = WordInfo::info_to_string(latin_word_info.info);
    output.senses = senses_info;

    output
}

fn create_pretty_latin_definition(
    latin_translation_info: LatinTranslationInfo,
) -> PrettifiedDefinition {
    let mut output = PrettifiedDefinition {
        tricks: None,
        orth_info: "".to_string(),
        form_info: "".to_string(),
        inflections: vec![],
        details: "".to_string(),
        senses: "".to_string(),
        modifiers: vec![],
    };

    let latin_word_info = latin_translation_info.word;

    match latin_word_info {
        Word::LatinWordInfo(latin_word_info) => {
            let parts_info = latin_word_info
                .parts
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ");

            let senses = latin_word_info.senses.clone();
            let extension_senses = latin_word_info.extension_senses;

            let senses = if extension_senses.is_some() {
                let mut senses = senses;
                let extension_senses = extension_senses.unwrap();
                senses.extend(extension_senses);
                senses
            } else {
                senses
            };

            let senses_info = senses
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" | ");

            let modifiers = if latin_word_info.modifiers.is_some() {
                latin_word_info.modifiers.unwrap()
            } else {
                vec![]
            };

            if modifiers.len() >= 1 {
                for modifier in modifiers {
                    let mut modifier_attachment = ModifierAttachment {
                        pos: "".to_string(),
                        senses: "".to_string(),
                        orth: "".to_string(),
                        modifier: "".to_string(),
                    };

                    modifier_attachment.pos = modifier.pos;
                    modifier_attachment.senses = modifier
                        .senses
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>()
                        .join(" | ");
                    modifier_attachment.orth = modifier.orth;
                    modifier_attachment.modifier = modifier.modifier.unwrap_or("".to_string());

                    output.modifiers.push(modifier_attachment);
                }
            }

            output.orth_info = parts_info;
            output.form_info = convert_form_to_string(latin_word_info.form);
            output.details = WordInfo::info_to_string(latin_word_info.info);
            output.senses = senses_info;
        }
        Word::UniqueLatinWordInfo(unique_latin_word_info) => {
            output.orth_info = unique_latin_word_info.orth;
            output.form_info = convert_form_to_string(unique_latin_word_info.form);
            output.details = WordInfo::info_to_string(unique_latin_word_info.info);
            output.senses = unique_latin_word_info
                .senses
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" | ");
        }
    }

    output.inflections = make_inflection_string(
        latin_translation_info.stem,
        latin_translation_info.inflections,
    );

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
        inflection_string =
            inflection_string + " | " + convert_form_to_string(inflection.form).as_str();
        inflection_string = inflection_string + " | " + inflection.pos.as_str();
        string_inflection.push(inflection_string);
    }

    string_inflection
}
