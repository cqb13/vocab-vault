use serde::{Deserialize, Deserializer, Serialize};

use crate::dictionary_structures::dictionary_keys::{
    Age, Area, Comparison, Declension, Frequency, Gender, Geography, Mood, Noun, Number, Numeral,
    PartOfSpeech, Pronoun, Source, Tense, Verb, Voice,
};
use crate::translators::Structure;
use crate::utils::number_with_ending;
use crate::utils::principle_part_generator::{generate_principle_parts, Generator};
use crate::utils::type_translator::translate_type;
// We know that the dictionary data, and that all the values are the types they should be, so we can unwrap without worry.

pub enum Part {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone)]
pub struct LatinWordInfo {
    pub orth: String,
    pub parts: Vec<String>,
    pub senses: Vec<String>,
    pub pos: PartOfSpeech,
    pub form: Form,
    pub info: WordInfo,
    pub n: Option<Vec<NValue>>,
    pub modifiers: Option<Vec<Modifier>>,
    pub id: i32,
    pub extension_senses: Option<Vec<String>>,
}

impl LatinWordInfo {
    pub fn new() -> LatinWordInfo {
        LatinWordInfo {
            orth: String::new(),
            parts: Vec::new(),
            senses: Vec::new(),
            pos: PartOfSpeech::Noun,
            form: Form::StrForm(String::new()),
            info: WordInfo {
                age: Age::Unknown,
                area: Area::AllOrNone,
                geo: Geography::AllOrNone,
                freq: Frequency::Unknown,
                source: Source::Unknown,
            },
            n: None,
            modifiers: None,
            id: 0,
            extension_senses: None,
        }
    }

    pub fn generate_principle_parts(&mut self) {
        let parts = &self.parts;

        let n = match &self.n {
            Some(n) => n,
            None => return,
        };

        if parts.is_empty() || n.len() < 2 {
            return;
        }

        let n_value_1 = match &self.n {
            Some(n) => n.get(0).unwrap().get_n_value_1(),
            None => 0,
        };

        let n_value_2 = match &self.n {
            Some(n) => n.get(1).unwrap().get_n_value_2(),
            None => 0,
        };

        match self.pos {
            PartOfSpeech::Noun => {
                let gender = match &self.form {
                    Form::LongForm(form) => form.gender.unwrap_or(Gender::Unknown),
                    Form::StrForm(form) => {
                        let form_array = form.split_whitespace().collect::<Vec<&str>>();
                        if form_array.len() < 2 {
                            Gender::Unknown
                        } else {
                            Gender::dict_key_to_gender(form_array[2])
                        }
                    }
                };
                let new_parts = generate_principle_parts(
                    Generator::Noun,
                    n_value_1,
                    n_value_2,
                    parts.to_vec(),
                    Some(gender),
                    None,
                    None,
                    None,
                );
                self.orth = new_parts[0].to_string();
                self.parts = new_parts;
            }
            PartOfSpeech::Verb => {
                let verb_type = match &self.form {
                    Form::LongForm(form) => form.verb.unwrap_or(Verb::Unknown),
                    Form::StrForm(form) => {
                        let form_array = form.split_whitespace().collect::<Vec<&str>>();
                        if form_array.len() < 2 {
                            Verb::Unknown
                        } else {
                            Verb::dict_key_to_verb(form_array[2])
                        }
                    }
                };
                let new_parts = generate_principle_parts(
                    Generator::Verb,
                    n_value_1,
                    n_value_2,
                    parts.to_vec(),
                    None,
                    None,
                    Some(verb_type),
                    None,
                );
                self.orth = new_parts[0].to_string();
                self.parts = new_parts;
            }
            PartOfSpeech::Adjective => {
                let comparison = match &self.form {
                    Form::LongForm(form) => form.comparison.unwrap_or(Comparison::Unknown),
                    Form::StrForm(form) => {
                        let form_array = form.split_whitespace().collect::<Vec<&str>>();
                        if form_array.len() < 2 {
                            Comparison::Unknown
                        } else {
                            Comparison::dict_key_to_comparison(form_array[2])
                        }
                    }
                };
                let new_parts = generate_principle_parts(
                    Generator::Adjective,
                    n_value_1,
                    n_value_2,
                    parts.to_vec(),
                    None,
                    Some(comparison),
                    None,
                    None,
                );
                self.orth = new_parts[0].to_string();
                self.parts = new_parts;
            }
            PartOfSpeech::Pronoun => {
                let new_parts = generate_principle_parts(
                    Generator::Pronoun,
                    n_value_1,
                    n_value_2,
                    parts.to_vec(),
                    None,
                    None,
                    None,
                    None,
                );
                self.orth = new_parts[0].to_string();
                self.parts = new_parts;
            }
            PartOfSpeech::Numeral => {
                let numeral_type = match &self.form {
                    Form::LongForm(form) => form.numeral.unwrap_or(Numeral::Unknown),
                    Form::StrForm(form) => {
                        let form_array = form.split_whitespace().collect::<Vec<&str>>();
                        if form_array.len() < 2 {
                            Numeral::Unknown
                        } else {
                            Numeral::dict_key_to_numeral(form_array[2])
                        }
                    }
                };
                let new_parts = generate_principle_parts(
                    Generator::Numeral,
                    n_value_1,
                    n_value_2,
                    parts.to_vec(),
                    None,
                    None,
                    None,
                    Some(numeral_type),
                );
                self.orth = new_parts[0].to_string();
                self.parts = new_parts;
            }
            _ => {}
        }
    }

    pub fn set_word(&mut self, latin_word_info: &LatinWordInfo) {
        self.orth = latin_word_info.orth.to_string();
        self.parts = latin_word_info.parts.to_vec();
        self.senses = latin_word_info.senses.to_vec();
        self.pos = latin_word_info.pos;
        self.form = latin_word_info.form.clone();
        self.info = latin_word_info.info.clone();
        self.n = latin_word_info.n.clone();
        self.modifiers = latin_word_info.modifiers.clone();
        self.id = latin_word_info.id;
        self.extension_senses = latin_word_info.extension_senses.to_owned();
    }

    pub fn get_part(&self, part: Part) -> Option<String> {
        match part {
            Part::First => self.parts.get(0).map(|s| s.to_string()),
            Part::Second => self.parts.get(1).map(|s| s.to_string()),
            Part::Third => self.parts.get(2).map(|s| s.to_string()),
            Part::Fourth => self.parts.get(3).map(|s| s.to_string()),
        }
    }

    pub fn set_orth(&mut self, orth: &str) {
        self.orth = orth.to_string();
    }

    pub fn set_parts(&mut self, parts: Vec<String>) {
        self.parts = parts;
    }

    pub fn set_senses(&mut self, senses: Vec<String>) {
        self.senses = senses;
    }

    pub fn set_pos(&mut self, pos: PartOfSpeech) {
        self.pos = pos;
    }

    pub fn set_form(&mut self, form: Form) {
        self.form = form
    }

    pub fn set_info(&mut self, info: WordInfo) {
        self.info = info
    }

    pub fn set_n(&mut self, n: Vec<NValue>) {
        self.n = Some(n)
    }

    pub fn set_modifiers(&mut self, modifiers: Vec<Modifier>) {
        self.modifiers = Some(modifiers)
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id
    }

    pub fn set_extension_senses(&mut self, extension_senses: Vec<String>) {
        self.extension_senses = Some(extension_senses)
    }
}

impl Serialize for LatinWordInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "orth".to_string(),
            serde_json::Value::String(self.orth.to_string()),
        );
        map.insert(
            "parts".to_string(),
            serde_json::Value::Array(
                self.parts
                    .iter()
                    .map(|p| serde_json::Value::String(p.to_string()))
                    .collect(),
            ),
        );
        map.insert(
            "senses".to_string(),
            serde_json::Value::Array(
                self.senses
                    .iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            ),
        );
        map.insert(
            "pos".to_string(),
            serde_json::Value::String(self.pos.as_str().to_string()),
        );
        map.insert(
            "form".to_string(),
            match &self.form {
                Form::StrForm(form) => serde_json::Value::String(form.to_string()),
                Form::LongForm(long_form) => serde_json::Value::Object(
                    serde_json::to_value(long_form)
                        .unwrap()
                        .as_object()
                        .unwrap()
                        .clone(),
                ),
            },
        );
        map.insert(
            "info".to_string(),
            serde_json::Value::Object(
                serde_json::to_value(&self.info)
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .clone(),
            ),
        );
        map.insert(
            "n".to_string(),
            match &self.n {
                Some(n) => serde_json::to_value(n).unwrap(),
                None => serde_json::Value::Null,
            },
        );
        map.insert(
            "id".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.id)),
        );
        map.insert(
            "extension_senses".to_string(),
            match &self.extension_senses {
                Some(extension_senses) => serde_json::Value::Array(
                    extension_senses
                        .iter()
                        .map(|s| serde_json::Value::String(s.to_string()))
                        .collect(),
                ),
                None => serde_json::Value::Null,
            },
        );
        map.insert(
            "modifiers".to_string(),
            match &self.modifiers {
                Some(modifiers) => serde_json::Value::Array(
                    modifiers
                        .iter()
                        .map(|m| serde_json::to_value(m).unwrap())
                        .collect(),
                ),
                None => serde_json::Value::Null,
            },
        );
        serde_json::Value::Object(map).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for LatinWordInfo {
    fn deserialize<D>(deserializer: D) -> Result<LatinWordInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let orth = serde_json::from_value(map.remove("orth").unwrap())
            .expect("Failed to deserialize orth");
        let parts = serde_json::from_value(map.remove("parts").unwrap())
            .expect("Failed to deserialize parts");
        let senses: Vec<String> = serde_json::from_value(map.remove("senses").unwrap())
            .expect("Failed to deserialize senses");
        let pos = map.remove("pos").unwrap().to_string();
        let form = serde_json::from_value(map.remove("form").unwrap())
            .expect("Failed to deserialize form");
        let info = map.remove("info").unwrap().to_string();
        // n is a list of values, so it can be a string, an integer, or a list of integers

        let n_value =
            serde_json::from_value(map.remove("n").unwrap()).expect("Failed to deserialize n");
        let n = match n_value {
            serde_json::Value::Array(n) => Some(
                n.iter()
                    .map(|n| {
                        if n.is_array() {
                            NValue::IntInt(
                                n[0].to_string().parse().unwrap(),
                                n[1].to_string().parse().unwrap(),
                            )
                        } else if n.is_string() {
                            NValue::String(serde_json::from_value(n.to_owned()).unwrap())
                        } else {
                            NValue::Integer(n.to_string().parse().unwrap())
                        }
                    })
                    .collect(),
            ),
            _ => None,
        };

        let id = map.remove("id").unwrap().to_string().parse().unwrap();

        Ok(LatinWordInfo {
            orth,
            parts,
            senses,
            pos: PartOfSpeech::dict_key_to_part_of_speech(&pos),
            form: Form::StrForm(form),
            info: serde_json::from_str(&info).unwrap(),
            n,
            modifiers: None,
            id,
            extension_senses: None,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct UniqueLatinWordInfo {
    pub orth: String,
    pub senses: Vec<String>,
    pub pos: PartOfSpeech,
    pub form: Form,
    pub n: Option<Vec<NValue>>,
    pub info: WordInfo,
}

impl<'de> Deserialize<'de> for UniqueLatinWordInfo {
    fn deserialize<D>(deserializer: D) -> Result<UniqueLatinWordInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let orth = serde_json::from_value(map.remove("orth").unwrap())
            .expect("Failed to deserialize orth");
        let senses: Vec<String> = serde_json::from_value(map.remove("senses").unwrap())
            .expect("Failed to deserialize senses");
        let pos = map.remove("pos").unwrap().to_string();
        let form = serde_json::from_value(map.remove("form").unwrap())
            .expect("Failed to deserialize form");
        let info = map.remove("info").unwrap().to_string();
        let n = match map.remove("n") {
            Some(n) => {
                let n_value = serde_json::from_value(n).expect("Failed to deserialize n");
                match n_value {
                    serde_json::Value::Array(n) => Some(
                        n.iter()
                            .map(|n| {
                                if n.is_array() {
                                    NValue::IntInt(
                                        n[0].to_string().parse().unwrap(),
                                        n[1].to_string().parse().unwrap(),
                                    )
                                } else if n.is_string() {
                                    NValue::String(serde_json::from_value(n.to_owned()).unwrap())
                                } else {
                                    NValue::Integer(n.to_string().parse().unwrap())
                                }
                            })
                            .collect(),
                    ),
                    _ => None,
                }
            }
            None => None,
        };

        Ok(UniqueLatinWordInfo {
            orth,
            senses,
            pos: PartOfSpeech::dict_key_to_part_of_speech(&pos),
            form: Form::StrForm(form),
            n,
            info: serde_json::from_str(&info).unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct EnglishWordInfo {
    pub orth: String,
    pub wid: i32,
    pub pos: PartOfSpeech,
    pub frequency_type: Frequency,
    pub true_frequency: Option<i16>,
    pub frequency: i16,
    pub compound: i16,
    pub semi: i16,
}

impl EnglishWordInfo {
    pub fn new() -> EnglishWordInfo {
        EnglishWordInfo {
            orth: String::new(),
            wid: 0,
            pos: PartOfSpeech::Noun,
            frequency_type: Frequency::Unknown,
            true_frequency: Some(0),
            frequency: 0,
            compound: 0,
            semi: 0,
        }
    }

    pub fn set_word(&mut self, english_word_info: EnglishWordInfo) {
        self.orth = english_word_info.orth;
        self.wid = english_word_info.wid;
        self.pos = english_word_info.pos;
        self.frequency_type = english_word_info.frequency_type;
        self.true_frequency = english_word_info.true_frequency;
        self.frequency = english_word_info.frequency;
        self.compound = english_word_info.compound;
        self.semi = english_word_info.semi;
    }

    pub fn set_orth(&mut self, orth: &str) {
        self.orth = orth.to_string();
    }

    pub fn set_wid(&mut self, wid: i32) {
        self.wid = wid;
    }

    pub fn set_pos(&mut self, pos: PartOfSpeech) {
        self.pos = pos;
    }

    pub fn set_frequency_type(&mut self, frequency_type: Frequency) {
        self.frequency_type = frequency_type;
    }

    pub fn set_true_frequency(&mut self, true_frequency: Option<i16>) {
        self.true_frequency = true_frequency;
    }

    pub fn set_frequency(&mut self, frequency: i16) {
        self.frequency = frequency;
    }

    pub fn set_compound(&mut self, compound: i16) {
        self.compound = compound;
    }

    pub fn set_semi(&mut self, semi: i16) {
        self.semi = semi;
    }
}

impl Serialize for EnglishWordInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "orth".to_string(),
            serde_json::Value::String(self.orth.to_string()),
        );
        map.insert(
            "wid".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.wid)),
        );
        map.insert(
            "pos".to_string(),
            serde_json::Value::String(self.pos.as_str().to_string()),
        );
        map.insert(
            "frequency_type".to_string(),
            serde_json::Value::String(self.frequency_type.as_str().to_string()),
        );
        map.insert(
            "true_frequency".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.true_frequency.unwrap_or(0))),
        );
        map.insert(
            "frequency".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.frequency)),
        );
        map.insert(
            "compound".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.compound)),
        );
        map.insert(
            "semi".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.semi)),
        );
        serde_json::Value::Object(map).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EnglishWordInfo {
    fn deserialize<D>(deserializer: D) -> Result<EnglishWordInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let orth = map.remove("orth").unwrap();
        let wid = map.remove("wid").unwrap();
        let pos = map.remove("pos").unwrap().to_string();
        let frequency_type = map.remove("frequency_type").unwrap().to_string();
        let frequency = map
            .remove("frequency")
            .unwrap()
            .to_string()
            .parse()
            .unwrap();
        let compound = map.remove("compound").unwrap().to_string().parse().unwrap();
        let semi = map.remove("semi").unwrap().to_string().parse().unwrap();
        let true_frequency = Some(frequency + compound - semi);

        Ok(EnglishWordInfo {
            orth: orth.to_string().trim_matches('"').to_string(),
            wid: wid.to_string().parse().unwrap(),
            pos: PartOfSpeech::dict_key_to_part_of_speech(&pos),
            frequency_type: Frequency::dict_key_to_frequency(&frequency_type),
            true_frequency,
            frequency,
            compound,
            semi,
        })
    }
}

#[derive(Debug, Clone)]
pub struct WordInfo {
    pub age: Age,
    pub area: Area,
    pub geo: Geography,
    pub freq: Frequency,
    pub source: Source,
}

impl WordInfo {
    pub fn new() -> WordInfo {
        WordInfo {
            age: Age::Unknown,
            area: Area::AllOrNone,
            geo: Geography::AllOrNone,
            freq: Frequency::Unknown,
            source: Source::Unknown,
        }
    }

    pub fn as_str(&self) -> String {
        format!(
            "age: {}, area: {}, origin: {}, freq: {}, source: {}",
            self.age.as_str(),
            self.area.as_str(),
            self.geo.as_str(),
            self.freq.as_str(),
            self.source.as_str()
        )
    }

    pub fn set_age(&mut self, age: Age) {
        self.age = age;
    }

    pub fn set_area(&mut self, area: Area) {
        self.area = area;
    }

    pub fn set_geo(&mut self, geo: Geography) {
        self.geo = geo;
    }

    pub fn set_freq(&mut self, freq: Frequency) {
        self.freq = freq;
    }

    pub fn set_source(&mut self, source: Source) {
        self.source = source;
    }
}

impl Serialize for WordInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "age".to_string(),
            serde_json::Value::String(self.age.as_str().to_string()),
        );
        map.insert(
            "area".to_string(),
            serde_json::Value::String(self.area.as_str().to_string()),
        );
        map.insert(
            "geo".to_string(),
            serde_json::Value::String(self.geo.as_str().to_string()),
        );
        map.insert(
            "freq".to_string(),
            serde_json::Value::String(self.freq.as_str().to_string()),
        );
        map.insert(
            "source".to_string(),
            serde_json::Value::String(self.source.as_str().to_string()),
        );
        serde_json::Value::Object(map).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WordInfo {
    fn deserialize<D>(deserializer: D) -> Result<WordInfo, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let age = map.remove("age").unwrap().to_string();
        let area = map.remove("area").unwrap().to_string();
        let geo = map.remove("geo").unwrap().to_string();
        let freq = map.remove("freq").unwrap().to_string();
        let source = map.remove("source").unwrap().to_string();

        Ok(WordInfo {
            age: Age::dict_key_to_age(&age),
            area: Area::dict_key_to_area(&area),
            geo: Geography::dict_key_to_geography(&geo),
            freq: Frequency::dict_key_to_frequency(&freq),
            source: Source::dict_key_to_source(&source),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Inflection {
    pub ending: String,
    pub pos: PartOfSpeech,
    pub note: Option<String>,
    pub n: Option<Vec<NValue>>,
    pub form: Form,
}

impl Inflection {
    pub fn new() -> Inflection {
        Inflection {
            ending: String::new(),
            pos: PartOfSpeech::Unknown,
            note: None,
            n: None,
            form: Form::StrForm(String::new()),
        }
    }

    pub fn set_ending(&mut self, ending: &str) {
        self.ending = ending.to_string();
    }

    pub fn set_pos(&mut self, pos: PartOfSpeech) {
        self.pos = pos;
    }

    pub fn set_note(&mut self, note: String) {
        self.note = Some(note);
    }

    pub fn set_n(&mut self, n: Vec<NValue>) {
        self.n = Some(n);
    }

    pub fn set_form(&mut self, form: Form) {
        self.form = form;
    }
}

impl<'de> Deserialize<'de> for Inflection {
    fn deserialize<D>(deserializer: D) -> Result<Inflection, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let ending: String = serde_json::from_value(map.remove("ending").unwrap())
            .expect("Failed to deserialize ending");
        let pos: String =
            serde_json::from_value(map.remove("pos").unwrap()).expect("Failed to deserialize pos");
        let note = serde_json::from_value(map.remove("note").unwrap())
            .expect("Failed to deserialize note");
        let n = match map.remove("n") {
            Some(n) => {
                let n_value = serde_json::from_value(n).expect("Failed to deserialize n");
                match n_value {
                    serde_json::Value::Array(n) => Some(
                        n.iter()
                            .map(|n| {
                                if n.is_array() {
                                    NValue::IntInt(
                                        n[0].to_string().parse().unwrap(),
                                        n[1].to_string().parse().unwrap(),
                                    )
                                } else if n.is_string() {
                                    NValue::String(serde_json::from_value(n.to_owned()).unwrap())
                                } else {
                                    NValue::Integer(n.to_string().parse().unwrap())
                                }
                            })
                            .collect(),
                    ),
                    _ => None,
                }
            }
            None => None,
        };
        let form = serde_json::from_value(map.remove("form").unwrap())
            .expect("Failed to deserialize form");

        Ok(Inflection {
            ending: ending.to_string().trim_matches('"').to_string(),
            pos: PartOfSpeech::dict_key_to_part_of_speech(&pos),
            note,
            n,
            form: Form::StrForm(form),
        })
    }
}

impl Serialize for Inflection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "ending".to_string(),
            serde_json::Value::String(self.ending.to_string()),
        );
        map.insert(
            "pos".to_string(),
            serde_json::Value::String(self.pos.as_str().to_string()),
        );
        map.insert(
            "note".to_string(),
            match &self.note {
                Some(note) => serde_json::Value::String(note.to_string()),
                None => serde_json::Value::Null,
            },
        );
        map.insert(
            "n".to_string(),
            match &self.n {
                Some(n) => serde_json::Value::Array(
                    n.iter().map(|n| serde_json::to_value(n).unwrap()).collect(),
                ),
                None => serde_json::Value::Null,
            },
        );
        map.insert(
            "form".to_string(),
            match &self.form {
                Form::StrForm(form) => serde_json::Value::String(form.to_string()),
                Form::LongForm(long_form) => serde_json::Value::Object(
                    serde_json::to_value(long_form)
                        .unwrap()
                        .as_object()
                        .unwrap()
                        .clone(),
                ),
            },
        );
        serde_json::Value::Object(map).serialize(serializer)
    }
}

#[derive(Debug, Clone)]
pub struct Stem {
    pub pos: PartOfSpeech,
    pub form: Form,
    pub orth: String,
    pub n: Option<Vec<NValue>>,
    pub wid: i32,
}

impl Stem {
    pub fn new() -> Stem {
        Stem {
            pos: PartOfSpeech::Unknown,
            form: Form::StrForm(String::new()),
            orth: String::new(),
            n: None,
            wid: 0,
        }
    }

    pub fn set_pos(&mut self, pos: PartOfSpeech) {
        self.pos = pos;
    }

    pub fn set_form(&mut self, form: Form) {
        self.form = form;
    }

    pub fn set_orth(&mut self, orth: &str) {
        self.orth = orth.to_string();
    }

    pub fn set_n(&mut self, n: Vec<NValue>) {
        self.n = Some(n);
    }

    pub fn set_wid(&mut self, wid: i32) {
        self.wid = wid;
    }
}

impl Serialize for Stem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "pos".to_string(),
            serde_json::Value::String(self.pos.as_str().to_string()),
        );
        map.insert(
            "form".to_string(),
            match &self.form {
                Form::StrForm(form) => serde_json::Value::String(form.to_string()),
                Form::LongForm(long_form) => serde_json::Value::Object(
                    serde_json::to_value(long_form)
                        .unwrap()
                        .as_object()
                        .unwrap()
                        .clone(),
                ),
            },
        );
        map.insert(
            "orth".to_string(),
            serde_json::Value::String(self.orth.to_string()),
        );
        map.insert(
            "n".to_string(),
            match &self.n {
                Some(n) => serde_json::Value::Array(
                    n.iter().map(|n| serde_json::to_value(n).unwrap()).collect(),
                ),
                None => serde_json::Value::Null,
            },
        );
        map.insert(
            "wid".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.wid)),
        );
        serde_json::Value::Object(map).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Stem {
    fn deserialize<D>(deserializer: D) -> Result<Stem, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let pos: String =
            serde_json::from_value(map.remove("pos").unwrap()).expect("Failed to deserialize pos");
        let form = serde_json::from_value(map.remove("form").unwrap())
            .expect("Failed to deserialize form");
        let orth: String = serde_json::from_value(map.remove("orth").unwrap())
            .expect("Failed to deserialize orth");
        let n = match map.remove("n") {
            Some(n) => {
                let n_value = serde_json::from_value(n).expect("Failed to deserialize n");
                match n_value {
                    serde_json::Value::Array(n) => Some(
                        n.iter()
                            .map(|n| {
                                if n.is_array() {
                                    NValue::IntInt(
                                        n[0].to_string().parse().unwrap(),
                                        n[1].to_string().parse().unwrap(),
                                    )
                                } else if n.is_string() {
                                    NValue::String(serde_json::from_value(n.to_owned()).unwrap())
                                } else {
                                    NValue::Integer(n.to_string().parse().unwrap())
                                }
                            })
                            .collect(),
                    ),
                    _ => None,
                }
            }
            None => None,
        };
        let wid =
            serde_json::from_value(map.remove("wid").unwrap()).expect("Failed to deserialize wid");

        Ok(Stem {
            pos: PartOfSpeech::dict_key_to_part_of_speech(&pos),
            form: Form::StrForm(form),
            orth: orth.to_string().trim_matches('"').to_string(),
            n,
            wid,
        })
    }
}

#[derive(Debug, Clone)]
pub enum ModifierType {
    Prefix,
    Suffix,
    Tackon,
    Packon,
    NotPackon,
    Unspecified,
}

impl ModifierType {
    pub fn as_str(&self) -> String {
        match self {
            ModifierType::Prefix => "prefix".to_string(),
            ModifierType::Suffix => "suffix".to_string(),
            ModifierType::Tackon => "enclitic tackon".to_string(),
            ModifierType::Packon => "enclitic packon".to_string(),
            ModifierType::NotPackon => "enclitic not packon".to_string(),
            ModifierType::Unspecified => "unspecified".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Modifier {
    pub pos: PartOfSpeech,
    pub form: Option<Form>,
    pub senses: Vec<String>,
    pub orth: String,
    pub modifier: ModifierType,
}

impl Modifier {
    pub fn new() -> Modifier {
        Modifier {
            pos: PartOfSpeech::Unknown,
            form: None,
            senses: Vec::new(),
            orth: String::new(),
            modifier: ModifierType::Unspecified,
        }
    }

    pub fn set_pos(&mut self, pos: PartOfSpeech) {
        self.pos = pos;
    }

    pub fn set_form(&mut self, form: Form) {
        self.form = Some(form);
    }

    pub fn set_senses(&mut self, senses: &Vec<String>) {
        self.senses = senses.to_vec();
    }

    pub fn set_orth(&mut self, orth: &str) {
        self.orth = orth.to_string();
    }

    pub fn set_modifier(&mut self, modifier: ModifierType) {
        self.modifier = modifier;
    }
}

impl<'de> Deserialize<'de> for Modifier {
    fn deserialize<D>(deserializer: D) -> Result<Modifier, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let pos: String =
            serde_json::from_value(map.remove("pos").unwrap()).expect("Failed to deserialize pos");
        let form = match map.remove("form") {
            Some(form) => Some(serde_json::from_value(form).expect("Failed to deserialize form")),
            None => None,
        };
        let senses: Vec<String> = serde_json::from_value(map.remove("senses").unwrap())
            .expect("Failed to deserialize senses");
        let orth: String = serde_json::from_value(map.remove("orth").unwrap())
            .expect("Failed to deserialize orth");

        Ok(Modifier {
            pos: PartOfSpeech::dict_key_to_part_of_speech(&pos),
            form,
            senses,
            orth: orth.to_string().trim_matches('"').to_string(),
            modifier: ModifierType::Unspecified,
        })
    }
}

impl Serialize for Modifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "pos".to_string(),
            serde_json::Value::String(self.pos.as_str().to_string()),
        );
        map.insert(
            "form".to_string(),
            match &self.form {
                Some(form) => match form {
                    Form::StrForm(form) => serde_json::Value::String(form.to_string()),
                    Form::LongForm(long_form) => serde_json::Value::Object(
                        serde_json::to_value(long_form)
                            .unwrap()
                            .as_object()
                            .unwrap()
                            .clone(),
                    ),
                },
                None => serde_json::Value::Null,
            },
        );
        map.insert(
            "senses".to_string(),
            serde_json::Value::Array(
                self.senses
                    .iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            ),
        );
        map.insert(
            "orth".to_string(),
            serde_json::Value::String(self.orth.to_string()),
        );
        map.insert(
            "modifier".to_string(),
            serde_json::Value::String(self.modifier.as_str()),
        );
        serde_json::Value::Object(map).serialize(serializer)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum NValue {
    StrIntInt(String, i8, i8),
    IntInt(i8, i8),
    Integer(i8),
    String(String),
}

impl NValue {
    pub fn get_n_value_1(&self) -> i8 {
        match self {
            NValue::StrIntInt(_, i1, _) => *i1,
            NValue::IntInt(i1, _) => *i1,
            NValue::Integer(i1) => *i1,
            NValue::String(_) => 0,
        }
    }

    pub fn get_n_value_2(&self) -> i8 {
        match self {
            NValue::StrIntInt(_, _, i2) => *i2,
            NValue::IntInt(_, i2) => *i2,
            NValue::Integer(i2) => *i2,
            NValue::String(_) => 0,
        }
    }
}

impl Serialize for NValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            NValue::StrIntInt(s, i1, i2) => serde_json::Value::Array(vec![
                serde_json::Value::String(s.to_string()),
                serde_json::Value::Number(serde_json::Number::from(*i1)),
                serde_json::Value::Number(serde_json::Number::from(*i2)),
            ])
            .serialize(serializer),
            NValue::IntInt(i1, i2) => serde_json::Value::Array(vec![
                serde_json::Value::Number(serde_json::Number::from(*i1)),
                serde_json::Value::Number(serde_json::Number::from(*i2)),
            ])
            .serialize(serializer),
            NValue::Integer(i) => {
                serde_json::Value::Number(serde_json::Number::from(*i)).serialize(serializer)
            }
            NValue::String(s) => serde_json::Value::String(s.to_string()).serialize(serializer),
        }
    }
}

impl PartialEq for NValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NValue::StrIntInt(s1, i1, i2), NValue::StrIntInt(s2, i3, i4)) => {
                s1 == s2 && i1 == i3 && i2 == i4
            }
            (NValue::IntInt(i1, i2), NValue::IntInt(i3, i4)) => i1 == i3 && i2 == i4,
            (NValue::Integer(i1), NValue::Integer(i2)) => i1 == i2,
            (NValue::String(s1), NValue::String(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub enum Form {
    StrForm(String),
    LongForm(LongForm),
}

impl Form {
    pub fn as_str(&self) -> String {
        match self {
            Form::StrForm(s) => s.to_string(),
            Form::LongForm(form) => form.as_str(),
        }
    }

    pub fn str_form_to_long_form(&mut self, part_of_speech: PartOfSpeech, structure: Structure) {
        let string_form = match self {
            Form::StrForm(s) => s,
            Form::LongForm(_) => {
                return;
            }
        };

        let form_array: Vec<&str> = string_form.split_whitespace().collect();

        let mut long_form = LongForm::new();

        match structure {
            Structure::Inflection => {
                match part_of_speech {
                    PartOfSpeech::Noun
                    | PartOfSpeech::Pronoun
                    | PartOfSpeech::Adjective
                    | PartOfSpeech::Numeral
                    | PartOfSpeech::Supine => {
                        // GEN P C -> Genitive Plural Common
                        if form_array.len() == 3 {
                            long_form.declension =
                                Some(Declension::dict_key_to_declension(form_array[0]));
                            long_form.number = Some(Number::dict_key_to_number(form_array[1]));
                            long_form.gender = Some(Gender::dict_key_to_gender(form_array[2]));
                        }

                        *self = Form::LongForm(long_form);
                    }
                    PartOfSpeech::Verb => {
                        // PRES  ACTIVE  IND  2 S -> Present Active Indicative Second Person Singular
                        if form_array.len() == 5 {
                            long_form.tense = Some(Tense::dict_key_to_tense(form_array[0]));
                            long_form.voice = Some(Voice::dict_key_to_voice(form_array[1]));
                            long_form.mood = Some(Mood::dict_key_to_mood(form_array[2]));
                            long_form.person = {
                                let fourth_spot = form_array[3].parse::<i8>().unwrap_or(0);
                                let person = if fourth_spot == 0 {
                                    "unknown".to_string()
                                } else {
                                    format!("{} person", number_with_ending(fourth_spot))
                                };

                                Some(person)
                            };
                            long_form.number = Some(Number::dict_key_to_number(form_array[4]));
                        }

                        *self = Form::LongForm(long_form);
                    }
                    PartOfSpeech::Participle => {
                        // NOM S X PRES ACTIVE  PPL -> Nominative Singular Present Active Participle
                        if form_array.len() == 5 {
                            long_form.declension =
                                Some(Declension::dict_key_to_declension(form_array[0]));
                            long_form.number = Some(Number::dict_key_to_number(form_array[1]));
                            long_form.gender = Some(Gender::dict_key_to_gender(form_array[2]));
                            long_form.tense = Some(Tense::dict_key_to_tense(form_array[3]));
                            long_form.voice = Some(Voice::dict_key_to_voice(form_array[4]));
                        }

                        *self = Form::LongForm(long_form);
                    }
                    _ => {
                        *self = Form::StrForm(string_form.to_string());
                    }
                }
            }
            Structure::LatinWordInfo | Structure::Stem => {
                let word_type = if form_array.len() >= 3 {
                    form_array[2].to_string()
                } else {
                    return *self = Form::StrForm(string_form.to_string());
                };

                let n_value_1 = form_array[0].parse::<i8>().unwrap_or(0);
                let n_value_2 = form_array[1].parse::<i8>().unwrap_or(0);
                let n_value = NValue::IntInt(n_value_1, n_value_2);

                match part_of_speech {
                    PartOfSpeech::Noun => {
                        long_form.gender = Some(Gender::dict_key_to_gender(&word_type));
                        long_form.noun = Some(Noun::dict_key_to_noun(form_array[3]));
                        long_form.declension_type = Some(translate_type(n_value, part_of_speech));
                        *self = Form::LongForm(long_form);
                    }
                    PartOfSpeech::Verb | PartOfSpeech::Participle => {
                        long_form.verb = Some(Verb::dict_key_to_verb(&word_type));
                        long_form.verb_type = Some(translate_type(n_value, part_of_speech));
                        *self = Form::LongForm(long_form);
                    }
                    PartOfSpeech::Pronoun | PartOfSpeech::Packon => {
                        long_form.pronoun = Some(Pronoun::dict_key_to_pronoun(&word_type));
                        *self = Form::LongForm(long_form);
                    }
                    PartOfSpeech::Adjective => {
                        long_form.declension_type = Some(translate_type(n_value, part_of_speech));
                        *self = Form::LongForm(long_form);
                    }
                    _ => {
                        *self = Form::StrForm(string_form.to_string());
                    }
                }
            }
            Structure::Modifier => {
                *self = Form::StrForm(string_form.to_string());
            }
        }
    }
}

impl<'de> Deserialize<'de> for Form {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if let Ok(long_form) = serde_json::from_str::<LongForm>(&s) {
            Ok(Form::LongForm(long_form))
        } else {
            Ok(Form::StrForm(s))
        }
    }
}

impl PartialEq for Form {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Form::StrForm(s1), Form::StrForm(s2)) => s1 == s2,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct LongForm {
    pub comparison: Option<Comparison>,
    pub declension: Option<Declension>,
    pub declension_type: Option<String>,
    pub gender: Option<Gender>,
    pub mood: Option<Mood>,
    pub noun: Option<Noun>,
    pub number: Option<Number>,
    pub numeral: Option<Numeral>,
    pub person: Option<String>,
    pub part_of_speech: Option<PartOfSpeech>,
    pub pronoun: Option<Pronoun>,
    pub tense: Option<Tense>,
    pub verb: Option<Verb>,
    pub verb_type: Option<String>,
    pub voice: Option<Voice>,
}

impl Serialize for LongForm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "comparison".to_string(),
            serde_json::Value::String(
                self.comparison
                    .unwrap_or(Comparison::Unknown)
                    .as_str()
                    .to_string(),
            ),
        );
        map.insert(
            "declension".to_string(),
            serde_json::Value::String(
                self.declension
                    .unwrap_or(Declension::Unknown)
                    .as_str()
                    .to_string(),
            ),
        );
        map.insert(
            "declension_type".to_string(),
            serde_json::Value::String(
                self.declension_type
                    .to_owned()
                    .unwrap_or("unknown".to_string()),
            ),
        );
        map.insert(
            "gender".to_string(),
            serde_json::Value::String(self.gender.unwrap_or(Gender::Unknown).as_str().to_string()),
        );
        map.insert(
            "mood".to_string(),
            serde_json::Value::String(self.mood.unwrap_or(Mood::Unknown).as_str().to_string()),
        );
        map.insert(
            "noun".to_string(),
            serde_json::Value::String(self.noun.unwrap_or(Noun::Unknown).as_str().to_string()),
        );
        map.insert(
            "number".to_string(),
            serde_json::Value::String(self.number.unwrap_or(Number::Unknown).as_str().to_string()),
        );
        map.insert(
            "numeral".to_string(),
            serde_json::Value::String(
                self.numeral
                    .unwrap_or(Numeral::Unknown)
                    .as_str()
                    .to_string(),
            ),
        );
        map.insert(
            "person".to_string(),
            serde_json::Value::String(self.person.to_owned().unwrap_or(String::new())),
        );
        map.insert(
            "pos".to_string(),
            serde_json::Value::String(
                self.part_of_speech
                    .unwrap_or(PartOfSpeech::Unknown)
                    .as_str()
                    .to_string(),
            ),
        );
        map.insert(
            "pronoun".to_string(),
            serde_json::Value::String(
                self.pronoun
                    .unwrap_or(Pronoun::Unknown)
                    .as_str()
                    .to_string(),
            ),
        );
        map.insert(
            "tense".to_string(),
            serde_json::Value::String(self.tense.unwrap_or(Tense::Unknown).as_str().to_string()),
        );
        map.insert(
            "verb".to_string(),
            serde_json::Value::String(self.verb.unwrap_or(Verb::Unknown).as_str().to_string()),
        );
        map.insert(
            "verb_type".to_string(),
            serde_json::Value::String(self.verb_type.to_owned().unwrap_or("unknown".to_string())),
        );
        map.insert(
            "voice".to_string(),
            serde_json::Value::String(self.voice.unwrap_or(Voice::Unknown).as_str().to_string()),
        );

        serde_json::Value::Object(map).serialize(serializer)
    }
}

impl LongForm {
    pub fn new() -> LongForm {
        LongForm {
            comparison: None,
            declension: None,
            declension_type: None,
            gender: None,
            mood: None,
            noun: None,
            number: None,
            numeral: None,
            person: None,
            part_of_speech: None,
            pronoun: None,
            tense: None,
            verb: None,
            verb_type: None,
            voice: None,
        }
    }

    pub fn as_str(&self) -> String {
        format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
            self.comparison.unwrap_or(Comparison::Unknown).as_str(),
            self.declension.unwrap_or(Declension::Unknown).as_str(),
            self.declension_type
                .to_owned()
                .unwrap_or("unknown".to_string()),
            self.gender.unwrap_or(Gender::Unknown).as_str(),
            self.mood.unwrap_or(Mood::Unknown).as_str(),
            self.noun.unwrap_or(Noun::Unknown).as_str(),
            self.number.unwrap_or(Number::Unknown).as_str(),
            self.numeral.unwrap_or(Numeral::Unknown).as_str(),
            self.person.to_owned().unwrap_or("unknown".to_string()),
            self.part_of_speech
                .unwrap_or(PartOfSpeech::Unknown)
                .as_str(),
            self.pronoun.unwrap_or(Pronoun::Unknown).as_str(),
            self.tense.unwrap_or(Tense::Unknown).as_str(),
            self.verb.unwrap_or(Verb::Unknown).as_str(),
            self.verb_type.to_owned().unwrap_or("unknown".to_string()),
            self.voice.unwrap_or(Voice::Unknown).as_str(),
        )
    }

    pub fn as_clean_str(&self) -> String {
        let mut not_clean = self.as_str();
        not_clean = not_clean.replace("unknown", "");
        not_clean
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    pub fn set_comparison(mut self, comparison: Comparison) -> LongForm {
        self.comparison = Some(comparison);
        self
    }

    pub fn set_declension(mut self, declension: Declension) -> LongForm {
        self.declension = Some(declension);
        self
    }

    pub fn set_gender(mut self, gender: Gender) -> LongForm {
        self.gender = Some(gender);
        self
    }

    pub fn set_mood(mut self, mood: Mood) -> LongForm {
        self.mood = Some(mood);
        self
    }

    pub fn set_noun(mut self, noun: Noun) -> LongForm {
        self.noun = Some(noun);
        self
    }

    pub fn set_number(mut self, number: Number) -> LongForm {
        self.number = Some(number);
        self
    }

    pub fn set_numeral(mut self, numeral: Numeral) -> LongForm {
        self.numeral = Some(numeral);
        self
    }

    pub fn set_part_of_speech(mut self, part_of_speech: PartOfSpeech) -> LongForm {
        self.part_of_speech = Some(part_of_speech);
        self
    }

    pub fn set_pronoun(mut self, pronoun: Pronoun) -> LongForm {
        self.pronoun = Some(pronoun);
        self
    }

    pub fn set_tense(mut self, tense: Tense) -> LongForm {
        self.tense = Some(tense);
        self
    }

    pub fn set_verb(mut self, verb: Verb) -> LongForm {
        self.verb = Some(verb);
        self
    }

    pub fn set_voice(mut self, voice: Voice) -> LongForm {
        self.voice = Some(voice);
        self
    }
}

#[derive(Clone, Debug)]
pub struct Attachment {
    pub pos: PartOfSpeech,
    pub senses: Vec<String>,
    pub orth: String,
}

impl Attachment {
    pub fn new() -> Attachment {
        Attachment {
            pos: PartOfSpeech::Unknown,
            senses: Vec::new(),
            orth: String::new(),
        }
    }

    pub fn set_pos(&mut self, pos: PartOfSpeech) {
        self.pos = pos;
    }

    pub fn set_senses(&mut self, senses: Vec<String>) {
        self.senses = senses;
    }

    pub fn set_orth(&mut self, orth: &str) {
        self.orth = orth.to_string();
    }
}

impl Serialize for Attachment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut map = serde_json::Map::new();
        map.insert(
            "pos".to_string(),
            serde_json::Value::String(self.pos.as_str().to_string()),
        );
        map.insert(
            "senses".to_string(),
            serde_json::Value::Array(
                self.senses
                    .iter()
                    .map(|s| serde_json::Value::String(s.to_string()))
                    .collect(),
            ),
        );
        map.insert(
            "orth".to_string(),
            serde_json::Value::String(self.orth.to_string()),
        );
        serde_json::Value::Object(map).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Attachment {
    fn deserialize<D>(deserializer: D) -> Result<Attachment, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let pos: String =
            serde_json::from_value(map.remove("pos").unwrap()).expect("Failed to deserialize pos");
        let senses: Vec<String> = serde_json::from_value(map.remove("senses").unwrap())
            .expect("Failed to deserialize senses");
        let orth = serde_json::from_value(map.remove("orth").unwrap())
            .expect("Failed to deserialize orth");

        Ok(Attachment {
            pos: PartOfSpeech::dict_key_to_part_of_speech(&pos),
            senses,
            orth,
        })
    }
}
