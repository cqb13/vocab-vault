use crate::dictionary_structures::dictionary_keys::PartOfSpeech;
use crate::dictionary_structures::dictionary_values::{LatinWordInfo, EnglishWordInfo};
use self::parsers::latin_dictionary_parser::parse_latin_dictionary;
use serde_json;

mod parsers {
    pub mod latin_dictionary_parser;
}

#[derive(Debug)]
pub enum WordType {
    English,
    Latin,
    Inflections,
    NotPackons,
    Packons,
    Prefixes,
    Stems,
    Suffixes,
    Tackons,
    Tickons,
    UniqueLatin,
}

impl WordType {
    pub fn from_str(s: &str) -> Result<WordType, String> {
        match s {
            "english" => Ok(WordType::English),
            "latin" => Ok(WordType::Latin),
            "inflections" => Ok(WordType::Inflections),
            "not_packons" => Ok(WordType::NotPackons),
            "packons" => Ok(WordType::Packons),
            "prefixes" => Ok(WordType::Prefixes),
            "stems" => Ok(WordType::Stems),
            "suffixes" => Ok(WordType::Suffixes),
            "tackons" => Ok(WordType::Tackons),
            "tickons" => Ok(WordType::Tickons),
            "unique_latin" => Ok(WordType::UniqueLatin),
            _ => Err(format!("Invalid word type: {}", s)),
        }
    }
}

pub enum OutputList {
    Latin(Vec<LatinWordInfo>),
    English(Vec<EnglishWordInfo>),
}

pub fn get_list(
    word_type: WordType,
    pos_list: Option<Vec<PartOfSpeech>>,
    max: Option<i32>,
    min: Option<i32>,
    exact: Option<i32>,
    amount: Option<i32>,
    random: bool,
    to: Option<String>,
) {
    let list: OutputList = match word_type {
        WordType::Latin => {
            let list = parse_latin_dictionary(pos_list, max, min, exact, amount, random);
            println!("{}", serde_json::to_string_pretty(&list).unwrap());
            OutputList::Latin(list)
        }
        _ => unimplemented!(),
    };
}
