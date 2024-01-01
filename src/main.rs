use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use english_to_latin::EnglishTranslationInfo;
use latin_to_english::LatinTranslationInfo;
use serde::{Deserialize, Serialize, Serializer};

mod english_to_latin;
mod latin_to_english;

pub mod utils {
    pub mod filter;
    pub mod post_processor;
    pub mod principle_part_generator;
    pub mod sorter;
}

pub mod data {
    pub mod data;
}

pub mod formatter {
    pub mod formatter;
    pub mod key_translator;
    pub mod type_translator;
}

pub mod tricks {
    pub mod trick_list;
    pub mod tricks;
    pub mod word_mods;
}

use crate::formatter::formatter::sanitize_word;
use crate::utils::post_processor::post_process;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub struct Translation {
    word: String,
    #[serde(serialize_with = "serialize_translation")]
    pub definitions: TranslationType,
}

impl Clone for Translation {
    fn clone(&self) -> Self {
        Translation {
            word: self.word.clone(),
            definitions: self.definitions.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TranslationType {
    #[serde(rename = "Latin")]
    Latin(Vec<LatinTranslationInfo>),
    #[serde(rename = "English")]
    English(Vec<EnglishTranslationInfo>),
}

impl Clone for TranslationType {
    fn clone(&self) -> Self {
        match self {
            TranslationType::Latin(info) => TranslationType::Latin(info.clone()),
            TranslationType::English(info) => TranslationType::English(info.clone()),
        }
    }
}

pub enum Language {
    Latin,
    English,
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Language::Latin, Language::Latin) => true,
            (Language::English, Language::English) => true,
            _ => false,
        }
    }
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://192.168.4.149:5500")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new().wrap(cors).service(query_latin_to_english)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[derive(Deserialize, Debug)]
struct Query {
    text: String,
    max_definitions: usize,
    use_tricks: bool,
    format_output: bool,
    clean_output: bool,
    sort_output: bool,
    filter_uncommon_translations: bool,
}

#[get("/latin_to_english")]
async fn query_latin_to_english(query: web::Query<Query>) -> impl Responder {
    let text = &query.text;
    let max_definitions = query.max_definitions;
    let use_tricks = query.use_tricks;
    let format_output = query.format_output;
    let clean_output = query.clean_output;
    let sort_output = query.sort_output;
    let filter_uncommon_translations = query.filter_uncommon_translations;

    let latin_words: Vec<&str> = text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    for latin_word in latin_words {
        let output = latin_to_english::translate_to_english(sanitize_word(latin_word), use_tricks);
        if output.len() > 0 {
            translations.push(Translation {
                word: latin_word.to_string(),
                definitions: TranslationType::Latin(output),
            });
        }
    }

    let output = post_process(
        translations,
        Language::Latin,
        max_definitions,
        format_output,
        clean_output,
        sort_output,
        filter_uncommon_translations,
    );

    HttpResponse::Ok().body(format!("{}", output))
}

#[get("/english_to_latin")]
async fn query_english_to_latin(query: web::Query<Query>) -> impl Responder {
    let text = &query.text;
    let max_definitions = query.max_definitions;
    let format_output = query.format_output;
    let clean_output = query.clean_output;
    let sort_output = query.sort_output;

    let english_words: Vec<&str> = text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    for word in english_words {
        let output = english_to_latin::translate_to_latin(
            &sanitize_word(word),
            max_definitions,
            sort_output,
        );
        if output.len() > 0 {
            translations.push(Translation {
                word: word.to_string(),
                definitions: TranslationType::English(output),
            });
        }
    }

    let output = post_process(
        translations,
        Language::English,
        max_definitions,
        format_output,
        clean_output,
        false,
        false,
    );

    HttpResponse::Ok().body(format!("{}", output))
}
