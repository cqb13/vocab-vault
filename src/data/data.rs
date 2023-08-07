use serde::{Deserialize, Deserializer, Serialize};
use std::include_bytes;

#[derive(Serialize, Deserialize, Debug)]
pub struct LatinWordInfo {
    pub orth: String,
    pub parts: Vec<String>,
    pub senses: Vec<String>,
    pub pos: String,
    pub form: Form,
    pub info: WordInfo,
    pub n: Vec<NValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<UniqueLatinWordInfo>>,
    pub id: i32,
}

impl Clone for LatinWordInfo {
    fn clone(&self) -> LatinWordInfo {
        LatinWordInfo {
            orth: self.orth.clone(),
            parts: self.parts.clone(),
            senses: self.senses.clone(),
            pos: self.pos.clone(),
            form: self.form.clone(),
            info: self.info.clone(),
            n: self.n.clone(),
            modifiers: self.modifiers.clone(),
            id: self.id.clone(),
        }
    }
}

impl LatinWordInfo {
    pub fn new() -> LatinWordInfo {
        LatinWordInfo {
            orth: "".to_string(),
            parts: Vec::new(),
            senses: Vec::new(),
            pos: "".to_string(),
            form: Form::new_str(),
            info: WordInfo::new(),
            n: Vec::new(),
            modifiers: None,
            id: 0,
        }
    }

    pub fn set_modifiers(&mut self, modifiers: Vec<UniqueLatinWordInfo>) {
        self.modifiers = Some(modifiers);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueLatinWordInfo {
    pub orth: String,
    pub senses: Vec<String>,
    pub pos: String,
    pub form: Form,
    pub n: Vec<NValue>,
    pub info: WordInfo,
}

impl UniqueLatinWordInfo {
    pub fn new() -> UniqueLatinWordInfo {
        UniqueLatinWordInfo {
            orth: "".to_string(),
            senses: Vec::new(),
            pos: "".to_string(),
            form: Form::new_str(),
            n: Vec::new(),
            info: WordInfo::new(),
        }
    }
}

impl Clone for UniqueLatinWordInfo {
    fn clone(&self) -> UniqueLatinWordInfo {
        UniqueLatinWordInfo {
            orth: self.orth.clone(),
            senses: self.senses.clone(),
            pos: self.pos.clone(),
            form: self.form.clone(),
            n: self.n.clone(),
            info: self.info.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WordInfo {
    pub age: String,
    pub area: String,
    pub geo: String,
    pub freq: String,
    pub source: String,
}

impl WordInfo {
    pub fn new() -> WordInfo {
        WordInfo {
            age: "".to_string(),
            area: "".to_string(),
            geo: "".to_string(),
            freq: "".to_string(),
            source: "".to_string(),
        }
    }

    pub fn new_set(
        age: String,
        area: String,
        geo: String,
        freq: String,
        source: String,
    ) -> WordInfo {
        WordInfo {
            age,
            area,
            geo,
            freq,
            source,
        }
    }

    pub fn info_to_string(info: WordInfo) -> String {
        let mut info_parts: Vec<String> = Vec::new();

        macro_rules! add_info {
            ($info:expr) => {
                let info = $info;
                info_parts.push(info);
            };
        }

        add_info!("age: ".to_owned() + &info.age);
        add_info!("area: ".to_owned() + &info.area);
        add_info!("geo: ".to_owned() + &info.geo);
        add_info!("freq: ".to_owned() + &info.freq);
        add_info!("source: ".to_owned() + &info.source);

        info_parts.join(" | ")
    }
}

impl Clone for WordInfo {
    fn clone(&self) -> WordInfo {
        WordInfo {
            age: self.age.clone(),
            area: self.area.clone(),
            geo: self.geo.clone(),
            freq: self.freq.clone(),
            source: self.source.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inflection {
    pub ending: String,
    pub pos: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub n: Vec<NValue>,
    pub form: Form,
}

impl Clone for Inflection {
    fn clone(&self) -> Inflection {
        Inflection {
            ending: self.ending.clone(),
            pos: self.pos.clone(),
            note: self.note.clone(),
            n: self.n.clone(),
            form: self.form.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stem {
    pub pos: String,
    pub form: Form,
    pub orth: String,
    pub n: Vec<NValue>,
    pub wid: i32,
}

impl Stem {
    pub fn new() -> Stem {
        Stem {
            pos: "".to_string(),
            form: Form::new_str(),
            orth: "".to_string(),
            n: Vec::new(),
            wid: 0,
        }
    }
}

impl PartialEq for Stem {
    fn eq(&self, other: &Self) -> bool {
        self.orth == other.orth
    }
}

impl Clone for Stem {
    fn clone(&self) -> Stem {
        Stem {
            pos: self.pos.clone(),
            form: self.form.clone(),
            orth: self.orth.clone(),
            n: self.n.clone(),
            wid: self.wid.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Modifier {
    pub pos: String,
    pub form: Form,
    pub senses: Vec<String>,
    pub orth: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Attachment {
    pub pos: String,
    pub senses: Vec<String>,
    pub orth: String,
}

impl Attachment {
    pub fn new() -> Attachment {
        Attachment {
            pos: "".to_string(),
            senses: Vec::new(),
            orth: "".to_string(),
        }
    }
}

impl PartialEq for Attachment {
    fn eq(&self, other: &Self) -> bool {
        self.orth == other.orth
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnglishWordInfo {
    pub orth: String,
    pub wid: i32,
    pub pos: String,
    pub frequency_type: String,
    pub true_frequency: Option<i16>,
    pub frequency: i16,
    pub compound: i16,
    pub semi: i16,
}

impl Clone for EnglishWordInfo {
    fn clone(&self) -> EnglishWordInfo {
        EnglishWordInfo {
            orth: self.orth.clone(),
            wid: self.wid.clone(),
            pos: self.pos.clone(),
            frequency_type: self.frequency_type.clone(),
            true_frequency: self.true_frequency.clone(),
            frequency: self.frequency.clone(),
            compound: self.compound.clone(),
            semi: self.semi.clone(),
        }
    }
}

impl From<EnglishWordInfo> for Vec<String> {
    fn from(word_info: EnglishWordInfo) -> Self {
        vec![
            word_info.orth,
            word_info.wid.to_string(),
            word_info.pos,
            word_info.frequency_type,
            word_info
                .true_frequency
                .map_or_else(|| "None".to_string(), |freq| freq.to_string()),
            word_info.frequency.to_string(),
            word_info.compound.to_string(),
            word_info.semi.to_string(),
        ]
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum NValue {
    StrIntInt(String, i8, i8),
    IntInt(i8, i8),
    Integer(i8),
    String(String),
}

impl Clone for NValue {
    fn clone(&self) -> NValue {
        match self {
            NValue::StrIntInt(s, i1, i2) => NValue::StrIntInt(s.clone(), i1.clone(), i2.clone()),
            NValue::IntInt(i1, i2) => NValue::IntInt(i1.clone(), i2.clone()),
            NValue::Integer(i) => NValue::Integer(i.clone()),
            NValue::String(s) => NValue::String(s.clone()),
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

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Form {
    StrForm(String),
    LongForm(LongForm),
}

impl Form {
    pub fn new_str() -> Form {
        Form::StrForm("".to_string())
    }

    pub fn new_long() -> Form {
        Form::LongForm(LongForm::new())
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

impl Clone for Form {
    fn clone(&self) -> Form {
        match self {
            Form::StrForm(s) => Form::StrForm(s.clone()),
            Form::LongForm(lf) => Form::LongForm(lf.clone()),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct LongForm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub declension: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tense: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mood: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<i8>,
}

impl LongForm {
    pub fn new() -> LongForm {
        LongForm {
            declension: None,
            number: None,
            gender: None,
            tense: None,
            voice: None,
            mood: None,
            verb: None,
            kind: None,
            person: None,
        }
    }

    pub fn long_form_to_string(form: LongForm) -> String {
        let mut form_parts = Vec::new();

        macro_rules! add_part {
            ($part:expr) => {
                if let Some(part) = $part {
                    form_parts.push(part);
                }
            };
        }

        add_part!(form.declension);
        add_part!(form.number);
        add_part!(form.gender);
        add_part!(form.tense);
        add_part!(form.voice);
        add_part!(form.mood);
        add_part!(form.verb);
        add_part!(form.kind);
        if let Some(person) = form.person {
            form_parts.push(person.to_string());
        }

        form_parts.join(" ")
    }
}


impl Clone for LongForm {
    fn clone(&self) -> LongForm {
        LongForm {
            declension: self.declension.clone(),
            number: self.number.clone(),
            gender: self.gender.clone(),
            tense: self.tense.clone(),
            voice: self.voice.clone(),
            mood: self.mood.clone(),
            verb: self.verb.clone(),
            kind: self.kind.clone(),
            person: self.person.clone(),
        }
    }
}

pub fn get_english_words() -> Vec<EnglishWordInfo> {
    let english_words_json = include_bytes!("../data/english_words.json");
    serde_json::from_slice(english_words_json).unwrap()
}

pub fn get_latin_dictionary() -> Vec<LatinWordInfo> {
    let latin_dictionary_json = include_bytes!("../data/latin_dictionary.json");
    serde_json::from_slice(latin_dictionary_json).unwrap()
}

pub fn get_unique_latin_words() -> Vec<UniqueLatinWordInfo> {
    let unique_latin_words_json = include_bytes!("../data/unique_latin_words.json");
    serde_json::from_slice(unique_latin_words_json).unwrap()
}

pub fn get_latin_stems() -> Vec<Stem> {
    let latin_stems_json = include_bytes!("../data/latin_stems.json");
    serde_json::from_slice(latin_stems_json).unwrap()
}

pub fn get_latin_inflections() -> Vec<Inflection> {
    let latin_inflections_json = include_bytes!("../data/latin_inflections.json");
    serde_json::from_slice(latin_inflections_json).unwrap()
}

pub fn get_latin_prefixes() -> Vec<Modifier> {
    let latin_prefixes_json = include_bytes!("../data/latin_prefixes.json");
    serde_json::from_slice(latin_prefixes_json).unwrap()
}

pub fn get_latin_suffixes() -> Vec<Modifier> {
    let latin_suffixes_json = include_bytes!("../data/latin_suffixes.json");
    serde_json::from_slice(latin_suffixes_json).unwrap()
}

pub fn get_latin_packons() -> Vec<Attachment> {
    let latin_packons_json = include_bytes!("../data/latin_packons.json");
    serde_json::from_slice(latin_packons_json).unwrap()
}

pub fn get_latin_not_packons() -> Vec<Attachment> {
    let latin_not_packons_json = include_bytes!("../data/latin_not_packons.json");
    serde_json::from_slice(latin_not_packons_json).unwrap()
}

pub fn get_latin_tackons() -> Vec<Attachment> {
    let latin_tackons_json = include_bytes!("../data/latin_tackons.json");
    serde_json::from_slice(latin_tackons_json).unwrap()
}

pub fn get_latin_tickons() -> Vec<Attachment> {
    let latin_tickons_json = include_bytes!("../data/latin_tickons.json");
    serde_json::from_slice(latin_tickons_json).unwrap()
}
