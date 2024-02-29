pub mod dictionary_structures;
pub mod translators;
pub mod use_data;
pub mod utils;

use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};
use dictionary_structures::dictionary_keys::PartOfSpeech;
use serde::Deserialize;
use translators::english_to_latin::translate_english_to_latin;
use translators::latin_to_english::translate_latin_to_english;
use translators::{Language, Translation, TranslationType};
use use_data::{get_list, WordType};
use utils::data::{get_english_dictionary, get_latin_dictionary};
use utils::sanitize_word;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://192.168.4.149:5500")
            .allowed_origin("https://learninglatin.net")
            .allowed_origin("https://www.learninglatin.net")
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(query_latin_to_english)
            .service(query_english_to_latin)
            .service(query_get_list)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

#[derive(Deserialize, Debug)]
struct LatinToEnglishQuery {
    latin_text: String,
    max: Option<usize>,
    tricks: bool,
    sort: bool,
}

#[get("/latin_to_english")]
async fn query_latin_to_english(query: web::Query<LatinToEnglishQuery>) -> impl Responder {
    let latin_words: Vec<&str> = query.latin_text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    for word in latin_words {
        let mut definitions = translate_latin_to_english(&sanitize_word(word), query.tricks);
        definitions.truncate(query.max.unwrap_or(6));
        let mut translation =
            Translation::new(word.to_string(), TranslationType::Latin(definitions));

        translation.post_process(Language::Latin, query.sort);
        translations.push(translation);
    }

    HttpResponse::Ok().body(format!(
        "{}",
        serde_json::to_string_pretty(&translations).unwrap()
    ))
}

#[derive(Deserialize, Debug)]
struct EnglishToLatinQuery {
    english_text: String,
    max: Option<usize>,
    sort: bool,
}

#[get("/english_to_latin")]
async fn query_english_to_latin(query: web::Query<EnglishToLatinQuery>) -> impl Responder {
    let english_words: Vec<&str> = query.english_text.split(" ").collect();
    let mut translations: Vec<Translation> = Vec::new();

    let latin_dictionary = get_latin_dictionary();
    let english_dictionary = get_english_dictionary();

    for word in english_words {
        let definitions = translate_english_to_latin(
            &english_dictionary,
            &latin_dictionary,
            &sanitize_word(word),
            query.max.unwrap_or(6),
            query.sort,
        );
        let mut translation =
            Translation::new(word.to_string(), TranslationType::English(definitions));
        translation.post_process(Language::English, query.sort);
        translations.push(translation);
    }

    HttpResponse::Ok().body(format!(
        "{}",
        serde_json::to_string_pretty(&translations).unwrap()
    ))
}

#[derive(Deserialize, Debug)]
struct GetListQuery {
    type_of_words: String,
    pos_list: String, // separated by commas
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
}

#[get("/get_list")]
async fn query_get_list(query: web::Query<GetListQuery>) -> impl Responder {
    if !WordType::is_valid_word_type(&query.type_of_words) {
        return HttpResponse::BadRequest().body("Invalid type of words.");
    }

    let word_type = WordType::from_str(query.type_of_words.as_str()).unwrap_or_else(|e| e);

    if word_type.is_unknown() {
        return HttpResponse::BadRequest().body("Invalid type of words.");
    }

    let pos_list: Option<Vec<PartOfSpeech>> = if query.pos_list.is_empty() {
        None
    } else {
        let pos_list: Vec<PartOfSpeech> = query
            .pos_list
            .split(",")
            .map(|pos| PartOfSpeech::dict_key_to_part_of_speech(pos))
            .collect();
        Some(pos_list)
    };

    if pos_list.is_some() && pos_list.as_ref().unwrap().contains(&PartOfSpeech::Unknown) {
        return HttpResponse::BadRequest().body("Invalid part of speech.");
    }

    let list = get_list(
        word_type,
        pos_list,
        query.max,
        query.min,
        query.exact,
        query.amount,
        query.random,
    );

    HttpResponse::Ok().body(list)
}
