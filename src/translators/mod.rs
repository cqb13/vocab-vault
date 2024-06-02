pub mod english_to_latin;
pub mod latin_to_english;

use crate::dictionary_structures::dictionary_values::Form;
use crate::translators::english_to_latin::EnglishTranslationInfo;
use crate::translators::latin_to_english::LatinTranslationInfo;
use serde::{Deserialize, Serialize, Serializer};
use std::mem::take;

pub enum DisplayType {
    Pretty(bool), // true if detailed
    Json,
}

pub enum Language {
    Latin,
    English,
}

impl Language {
    pub fn as_str(&self) -> &str {
        match self {
            Language::Latin => "Latin",
            Language::English => "English",
        }
    }
}

pub enum Structure {
    LatinWordInfo,
    Inflection,
    Stem,
    Modifier,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    pub word: String,
    #[serde(serialize_with = "serialize_translation")]
    pub definitions: TranslationType,
}

fn serialize_translation<S>(def: &TranslationType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match def {
        TranslationType::Latin(info) => info.serialize(serializer),
        TranslationType::English(info) => info.serialize(serializer),
    }
}

impl Translation {
    pub fn new(word: String, definitions: TranslationType) -> Translation {
        Translation { word, definitions }
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /**
     * Makes output more readable.
     * Should only be called after all parsing is done.
     */
    pub fn post_process(&mut self, language: Language, sort: bool) {
        let processed_translation = self;

        match language {
            Language::Latin => {
                if sort {
                    processed_translation.sort();
                }

                processed_translation.definitions = match &mut processed_translation.definitions {
                    TranslationType::Latin(definitions) => {
                        let new_definitions: Vec<_> = definitions
                            .drain(..)
                            .map(|mut definition| {
                                definition.word.form.str_form_to_long_form(
                                    definition.word.pos,
                                    Structure::LatinWordInfo,
                                );

                                definition.word.generate_principle_parts();

                                definition
                                    .stem
                                    .form
                                    .str_form_to_long_form(definition.word.pos, Structure::Stem);

                                if let Some(mut inflections) = take(&mut definition.inflections) {
                                    definition.remove_inflections_with_wrong_pos();
                                    let part_of_speech = definition.word.pos;

                                    inflections.iter_mut().for_each(|inflection| {
                                        inflection.form.str_form_to_long_form(
                                            part_of_speech,
                                            Structure::Inflection,
                                        );
                                    });

                                    definition.inflections = Some(inflections);
                                }
                                definition
                            })
                            .collect();
                        TranslationType::Latin(new_definitions)
                    }
                    _ => {
                        println!("Expected Latin translation type");
                        std::process::exit(0);
                    }
                };
            }
            Language::English => {
                processed_translation.definitions = match &mut processed_translation.definitions {
                    TranslationType::English(definitions) => {
                        let new_definitions: Vec<_> = definitions
                            .drain(..)
                            .map(|mut definition| {
                                definition.translation.form.str_form_to_long_form(
                                    definition.word.pos,
                                    Structure::LatinWordInfo,
                                );

                                definition.translation.generate_principle_parts();
                                definition
                            })
                            .collect();
                        TranslationType::English(new_definitions)
                    }
                    _ => {
                        println!("Expected English translation type");
                        std::process::exit(0);
                    }
                };
            }
        }
    }

    fn sort(&mut self) {
        let sorted_translation = self;

        match &mut sorted_translation.definitions {
            &mut TranslationType::Latin(ref mut info) => {
                info.sort_by(|a, b| {
                    a.word
                        .info
                        .freq
                        .as_number()
                        .cmp(&b.word.info.freq.as_number())
                });
            }
            _ => {}
        }
    }

    pub fn display(&self, display_type: DisplayType) {
        println!("{}", self.word);
        match display_type {
            DisplayType::Pretty(detailed) => match &self.definitions {
                TranslationType::Latin(definitions) => {
                    if definitions.is_empty() {
                        println!("No definitions found");
                        return;
                    }
                    for definition in definitions {
                        if definition.tricks.is_some() {
                            for trick in definition.tricks.as_ref().unwrap() {
                                println!("{}", trick);
                            }
                        }
                        if &definition.word.parts.len() > &0 {
                            println!();
                            for part in &definition.word.parts {
                                print!("{} ", part);
                            }
                            println!();
                        }
                        println!("{}", definition.word.pos.as_str());
                        match definition.word.form {
                            Form::StrForm(ref form) => {
                                println!("{}", form);
                            }
                            Form::LongForm(ref form) => {
                                println!("{}", form.as_clean_str());
                            }
                        }
                        if definition.inflections.is_some() {
                            let stem_orth = &definition.stem.orth;
                            for inflection in definition.inflections.as_ref().unwrap() {
                                if inflection.ending.is_empty() {
                                    continue;
                                }
                                let form_string = match &inflection.form {
                                    Form::StrForm(form) => {
                                        let form_string = form.as_str();
                                        if form_string.is_empty() {
                                            continue;
                                        }
                                        form_string.to_string()
                                    }
                                    Form::LongForm(form) => {
                                        let form_string = form.as_clean_str();
                                        if form_string.is_empty() {
                                            continue;
                                        }
                                        form_string
                                    }
                                };
                                let inflection_line = format!(
                                    "{}.{} | {}",
                                    stem_orth, inflection.ending, form_string
                                );
                                println!("{}", inflection_line);
                            }
                        }

                        if detailed {
                            println!("{}", definition.word.info.as_str());
                        }
                        for sense in &definition.word.senses {
                            print!("{} ", sense);
                        }

                        if definition.word.modifiers.is_some() {
                            println!();
                            for modifier in definition.word.modifiers.as_ref().unwrap() {
                                println!("\n{}: {}", modifier.modifier.as_str(), modifier.orth);
                                println!("{} ", modifier.pos.as_str());
                                for sense in &modifier.senses {
                                    print!("{} ", sense);
                                }
                            }
                        }

                        println!("\n");
                    }
                }
                TranslationType::English(definitions) => {
                    if definitions.is_empty() {
                        println!("No definitions found");
                        return;
                    }
                    for definition in definitions {
                        println!();
                        for part in &definition.translation.parts {
                            print!("{} ", part);
                        }
                        println!();
                        println!("{}", definition.word.pos.as_str());
                        match definition.translation.form {
                            Form::StrForm(ref form) => {
                                println!("{}", form);
                            }
                            Form::LongForm(ref form) => {
                                println!("{}", form.as_clean_str());
                            }
                        }
                        if detailed {
                            println!("{}", definition.translation.info.as_str());
                        }
                        for sense in &definition.translation.senses {
                            print!("{} ", sense);
                        }
                        println!();
                    }
                }
            },
            DisplayType::Json => {
                println!("{}", serde_json::to_string_pretty(&self).unwrap());
            }
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum TranslationType {
    #[serde(rename = "Latin")]
    Latin(Vec<LatinTranslationInfo>),
    #[serde(rename = "English")]
    English(Vec<EnglishTranslationInfo>),
}
