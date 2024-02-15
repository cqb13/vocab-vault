use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Comparison {
    Positive,
    Comparative,
    Superlative,
    Unknown,
}

impl Comparison {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Comparison::Positive => "positive",
            Comparison::Comparative => "comparative",
            Comparison::Superlative => "superlative",
            Comparison::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_comparison(key: &str) -> Comparison {
        match key.trim_matches('"') {
            "POS" => Comparison::Positive,
            "COMP" => Comparison::Comparative,
            "SUPER" => Comparison::Superlative,
            _ => Comparison::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Declension {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Vocative,
    Locative,
    Ablative,
    Unknown,
}

impl Declension {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Declension::Nominative => "nominative",
            Declension::Genitive => "genitive",
            Declension::Dative => "dative",
            Declension::Accusative => "accusative",
            Declension::Vocative => "vocative",
            Declension::Locative => "locative",
            Declension::Ablative => "ablative",
            Declension::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_declension(key: &str) -> Declension {
        match key.trim_matches('"') {
            "NOM" => Declension::Nominative,
            "GEN" => Declension::Genitive,
            "DAT" => Declension::Dative,
            "ACC" => Declension::Accusative,
            "VOC" => Declension::Vocative,
            "LOC" => Declension::Locative,
            "ABL" => Declension::Ablative,
            _ => Declension::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
    Common,
    Unknown,
}

impl PartialEq for Gender {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Gender::Masculine => "masculine",
            Gender::Feminine => "feminine",
            Gender::Neuter => "neuter",
            Gender::Common => "common",
            Gender::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_gender(key: &str) -> Gender {
        match key.trim_matches('"') {
            "M" => Gender::Masculine,
            "F" => Gender::Feminine,
            "N" => Gender::Neuter,
            "C" => Gender::Common,
            _ => Gender::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Mood {
    Indicative,
    Subjunctive,
    Imperative,
    Infinitive,
    Unknown,
}

impl Mood {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Mood::Indicative => "indicative",
            Mood::Subjunctive => "subjunctive",
            Mood::Imperative => "imperative",
            Mood::Infinitive => "infinitive",
            Mood::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_mood(key: &str) -> Mood {
        match key.trim_matches('"') {
            "IND" => Mood::Indicative,
            "SUB" => Mood::Subjunctive,
            "IMP" => Mood::Imperative,
            "INF" => Mood::Infinitive,
            _ => Mood::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Noun {
    Singular,
    Plural,
    Abstract,
    GroupName,
    ProperName,
    Person,
    Thing,
    Location,
    Place,
    Unknown,
}

impl Noun {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Noun::Singular => "singular",
            Noun::Plural => "plural / multiple",
            Noun::Abstract => "abstract idea",
            Noun::GroupName => "group name",
            Noun::ProperName => "proper name",
            Noun::Person => "person",
            Noun::Thing => "thing",
            Noun::Location => "location",
            Noun::Place => "a place (where)",
            Noun::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_noun(key: &str) -> Noun {
        match key.trim_matches('"') {
            "S" => Noun::Singular,
            "M" => Noun::Plural,
            "A" => Noun::Abstract,
            "G" => Noun::GroupName,
            "N" => Noun::ProperName,
            "P" => Noun::Person,
            "T" => Noun::Thing,
            "L" => Noun::Location,
            "W" => Noun::Place,
            _ => Noun::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Numeral {
    Cardinal,
    Ordinal,
    Distributive,
    Adverbial,
    Unknown,
}

impl Numeral {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Numeral::Cardinal => "cardinal",
            Numeral::Ordinal => "ordinal",
            Numeral::Distributive => "distributive",
            Numeral::Adverbial => "numeral adverb",
            Numeral::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_numeral(key: &str) -> Numeral {
        match key.trim_matches('"') {
            "CARD" => Numeral::Cardinal,
            "ORD" => Numeral::Ordinal,
            "DIST" => Numeral::Distributive,
            "ADVERB" => Numeral::Adverbial,
            _ => Numeral::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Number {
    Singular,
    Plural,
    Unknown,
}

impl Number {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Number::Singular => "singular",
            Number::Plural => "plural",
            Number::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_number(key: &str) -> Number {
        match key.trim_matches('"') {
            "S" => Number::Singular,
            "P" => Number::Plural,
            _ => Number::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PartOfSpeech {
    Noun,
    Verb,
    Participle,
    Adjective,
    Preposition,
    Pronoun,
    Interjection,
    Numeral,
    Conjunction,
    Adverb,
    Number,
    Supine,
    Packon,
    Tackon,
    Prefix,
    Suffix,
    Unknown,
}

impl PartialEq for PartOfSpeech {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartOfSpeech {
    pub fn as_str(&self) -> &'static str {
        match &self {
            PartOfSpeech::Noun => "noun",
            PartOfSpeech::Verb => "verb",
            PartOfSpeech::Participle => "participle",
            PartOfSpeech::Adjective => "adjective",
            PartOfSpeech::Preposition => "preposition",
            PartOfSpeech::Pronoun => "pronoun",
            PartOfSpeech::Interjection => "interjection",
            PartOfSpeech::Numeral => "numeral",
            PartOfSpeech::Conjunction => "conjunction",
            PartOfSpeech::Adverb => "adverb",
            PartOfSpeech::Number => "number",
            PartOfSpeech::Supine => "supine",
            PartOfSpeech::Packon => "packon",
            PartOfSpeech::Tackon => "tackon",
            PartOfSpeech::Prefix => "prefix",
            PartOfSpeech::Suffix => "suffix",
            PartOfSpeech::Unknown => "unknown",
        }
    }

    pub fn convert_to_generator(&self) -> crate::utils::principle_part_generator::Generator {
        match &self {
            PartOfSpeech::Noun => crate::utils::principle_part_generator::Generator::Noun,
            PartOfSpeech::Pronoun => crate::utils::principle_part_generator::Generator::Pronoun,
            PartOfSpeech::Adjective => crate::utils::principle_part_generator::Generator::Adjective,
            PartOfSpeech::Verb => crate::utils::principle_part_generator::Generator::Verb,
            PartOfSpeech::Participle => crate::utils::principle_part_generator::Generator::Verb,
            PartOfSpeech::Numeral => crate::utils::principle_part_generator::Generator::Numeral,
            _ => panic!("Cannot convert {:?} to a generator", self),
        }
    }

    pub fn dict_key_to_part_of_speech(key: &str) -> PartOfSpeech {
        match key.trim_matches('"') {
            "N" => PartOfSpeech::Noun,
            "V" => PartOfSpeech::Verb,
            "VPAR" => PartOfSpeech::Participle,
            "ADJ" => PartOfSpeech::Adjective,
            "PREP" => PartOfSpeech::Preposition,
            "PRON" => PartOfSpeech::Pronoun,
            "INTERJ" => PartOfSpeech::Interjection,
            "NUM" => PartOfSpeech::Numeral,
            "CONJ" => PartOfSpeech::Conjunction,
            "ADV" => PartOfSpeech::Adverb,
            "INT" => PartOfSpeech::Number,
            "SUPINE" => PartOfSpeech::Supine,
            "PACK" => PartOfSpeech::Packon,
            "TACKON" => PartOfSpeech::Tackon,
            "PREFIX" => PartOfSpeech::Prefix,
            "SUFFIX" => PartOfSpeech::Suffix,
            _ => PartOfSpeech::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Pronoun {
    Personal,
    Demonstrative,
    Relative,
    Interrogative,
    Reflexive,
    Indefinite,
    Adjective,
    Unknown,
}

impl Pronoun {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Pronoun::Personal => "personal",
            Pronoun::Demonstrative => "demonstrative",
            Pronoun::Relative => "relative",
            Pronoun::Interrogative => "interrogative",
            Pronoun::Reflexive => "reflexive",
            Pronoun::Indefinite => "indefinite",
            Pronoun::Adjective => "adjective",
            Pronoun::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_pronoun(key: &str) -> Pronoun {
        match key.trim_matches('"') {
            "PERS" => Pronoun::Personal,
            "DEMONS" => Pronoun::Demonstrative,
            "REL" => Pronoun::Relative,
            "INTERR" => Pronoun::Interrogative,
            "REFLEX" => Pronoun::Reflexive,
            "INDEF" => Pronoun::Indefinite,
            "ADJECT" => Pronoun::Adjective,
            _ => Pronoun::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Tense {
    Present,
    Imperfect,
    Future,
    Perfect,
    Pluperfect,
    FuturePerfect,
    Infinitive,
    Unknown,
}

impl Tense {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Tense::Present => "present",
            Tense::Imperfect => "imperfect",
            Tense::Future => "future",
            Tense::Perfect => "perfect",
            Tense::Pluperfect => "pluperfect",
            Tense::FuturePerfect => "future perfect",
            Tense::Infinitive => "infinitive",
            Tense::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_tense(key: &str) -> Tense {
        match key.trim_matches('"') {
            "PRES" => Tense::Present,
            "IMPF" => Tense::Imperfect,
            "FUT" => Tense::Future,
            "PERF" => Tense::Perfect,
            "PLUP" => Tense::Pluperfect,
            "FUTP" => Tense::FuturePerfect,
            "INF" => Tense::Infinitive,
            _ => Tense::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Verb {
    ToBe,
    ToBeing,
    TakesGenitive,
    TakesDative,
    TakesAblative,
    Transitive,
    Intransitive,
    Impersonal,
    Deponent,
    SemiDeponent,
    PerfectDefinite,
    Unknown,
}

impl PartialEq for Verb {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Verb {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Verb::ToBe => "to be",
            Verb::ToBeing => "to being",
            Verb::TakesGenitive => "takes genitive",
            Verb::TakesDative => "takes dative",
            Verb::TakesAblative => "takes ablative",
            Verb::Transitive => "transitive",
            Verb::Intransitive => "intransitive",
            Verb::Impersonal => "impersonal",
            Verb::Deponent => "deponent",
            Verb::SemiDeponent => "semi-deponent",
            Verb::PerfectDefinite => "perfect definite",
            Verb::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_verb(key: &str) -> Verb {
        match key.trim_matches('"') {
            "TO_BE" => Verb::ToBe,
            "TO_BEING" => Verb::ToBeing,
            "GEN" => Verb::TakesGenitive,
            "DAT" => Verb::TakesDative,
            "ABL" => Verb::TakesAblative,
            "TRANS" => Verb::Transitive,
            "INTRANS" => Verb::Intransitive,
            "IMPERS" => Verb::Impersonal,
            "DEP" => Verb::Deponent,
            "SEMIDEP" => Verb::SemiDeponent,
            "PERFDEF" => Verb::PerfectDefinite,
            _ => Verb::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Voice {
    Active,
    Passive,
    Unknown,
}

impl Voice {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Voice::Active => "active",
            Voice::Passive => "passive",
            Voice::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_voice(key: &str) -> Voice {
        match key.trim_matches('"') {
            "ACTIVE" => Voice::Active,
            "PASSIVE" => Voice::Passive,
            _ => Voice::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Age {
    Archaic,
    Early,
    Classical,
    Late,
    Later,
    Medieval,
    Scholar,
    Modern,
    UsedThroughoutAges,
    Unknown,
}

impl Age {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Age::Archaic => "archaic",
            Age::Early => "early",
            Age::Classical => "classical",
            Age::Late => "late",
            Age::Later => "later",
            Age::Medieval => "medieval",
            Age::Scholar => "scholar",
            Age::Modern => "modern",
            Age::UsedThroughoutAges => "used throughout ages",
            Age::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_age(key: &str) -> Age {
        match key.trim_matches('"') {
            "A" => Age::Archaic,
            "B" => Age::Early,
            "C" => Age::Classical,
            "D" => Age::Late,
            "E" => Age::Later,
            "F" => Age::Medieval,
            "G" => Age::Scholar,
            "H" => Age::Modern,
            "X" => Age::UsedThroughoutAges,
            _ => Age::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Area {
    Agriculture,
    Biological,
    Art,
    Religious,
    Grammar,
    Legal,
    Poetic,
    Scientific,
    Technical,
    Warfare,
    Mythological,
    AllOrNone,
    Unknown,
}

impl Area {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Area::Agriculture => "agriculture",
            Area::Biological => "biological",
            Area::Art => "art",
            Area::Religious => "religious",
            Area::Grammar => "grammar",
            Area::Legal => "legal",
            Area::Poetic => "poetic",
            Area::Scientific => "scientific",
            Area::Technical => "technical",
            Area::Warfare => "warfare",
            Area::Mythological => "mythological",
            Area::AllOrNone => "all or none",
            Area::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_area(key: &str) -> Area {
        match key.trim_matches('"') {
            "A" => Area::Agriculture,
            "B" => Area::Biological,
            "D" => Area::Art,
            "E" => Area::Religious,
            "G" => Area::Grammar,
            "L" => Area::Legal,
            "P" => Area::Poetic,
            "S" => Area::Scientific,
            "T" => Area::Technical,
            "W" => Area::Warfare,
            "Y" => Area::Mythological,
            "X" => Area::AllOrNone,
            _ => Area::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Geography {
    Africa,
    Britain,
    China,
    Scandinavia,
    Egypt,
    FranceGaul,
    Germany,
    Greece,
    ItalyRome,
    India,
    Balkans,
    Netherlands,
    Persia,
    NearEast,
    Russia,
    SpainIberia,
    EasternEurope,
    AllOrNone,
    Unknown,
}

impl Geography {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Geography::Africa => "Africa",
            Geography::Britain => "Britain",
            Geography::China => "China",
            Geography::Scandinavia => "Scandinavia",
            Geography::Egypt => "Egypt",
            Geography::FranceGaul => "France / Gaul",
            Geography::Germany => "Germany",
            Geography::Greece => "Greece",
            Geography::ItalyRome => "Italy / Rome",
            Geography::India => "India",
            Geography::Balkans => "Balkans",
            Geography::Netherlands => "Netherlands",
            Geography::Persia => "Persia",
            Geography::NearEast => "Near East",
            Geography::Russia => "Russia",
            Geography::SpainIberia => "Spain / Iberia",
            Geography::EasternEurope => "Eastern Europe",
            Geography::AllOrNone => "all or none",
            Geography::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_geography(key: &str) -> Geography {
        match key.trim_matches('"') {
            "A" => Geography::Africa,
            "B" => Geography::Britain,
            "C" => Geography::China,
            "D" => Geography::Scandinavia,
            "E" => Geography::Egypt,
            "F" => Geography::FranceGaul,
            "G" => Geography::Germany,
            "H" => Geography::Greece,
            "I" => Geography::ItalyRome,
            "J" => Geography::India,
            "K" => Geography::Balkans,
            "N" => Geography::Netherlands,
            "P" => Geography::Persia,
            "Q" => Geography::NearEast,
            "R" => Geography::Russia,
            "S" => Geography::SpainIberia,
            "U" => Geography::EasternEurope,
            "X" => Geography::AllOrNone,
            _ => Geography::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Frequency {
    VeryFrequent,
    Frequent,
    Common,
    Lesser,
    Uncommon,
    VeryRare,
    Inscription,
    Graffiti,
    Pliny,
    AllOrNone,
    Unknown,
}

impl Frequency {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Frequency::VeryFrequent => "very frequent",
            Frequency::Frequent => "frequent",
            Frequency::Common => "common",
            Frequency::Lesser => "lesser",
            Frequency::Uncommon => "uncommon",
            Frequency::VeryRare => "very rare",
            Frequency::Inscription => "inscription",
            Frequency::Graffiti => "graffiti",
            Frequency::Pliny => "Pliny (only in Pliny Natural History)",
            Frequency::AllOrNone => "all or none",
            Frequency::Unknown => "unknown",
        }
    }

    pub fn as_number(&self) -> u8 {
        match &self {
            Frequency::VeryFrequent => 1,
            Frequency::Frequent => 2,
            Frequency::Common => 3,
            Frequency::Lesser => 4,
            Frequency::Uncommon => 5,
            Frequency::VeryRare => 6,
            Frequency::Inscription => 7,
            Frequency::Graffiti => 8,
            Frequency::Pliny => 9,
            Frequency::AllOrNone => 10,
            Frequency::Unknown => 11,
        }
    }

    pub fn dict_key_to_frequency(key: &str) -> Frequency {
        match key.trim_matches('"') {
            "A" => Frequency::VeryFrequent,
            "B" => Frequency::Frequent,
            "C" => Frequency::Common,
            "D" => Frequency::Lesser,
            "E" => Frequency::Uncommon,
            "F" => Frequency::VeryRare,
            "I" => Frequency::Inscription,
            "M" => Frequency::Graffiti,
            "N" => Frequency::Pliny,
            "X" => Frequency::AllOrNone,
            _ => Frequency::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Source {
    Beeson,
    Beard,
    Adams,
    Stelten,
    Deferrari,
    Gildersleeve,
    Collatinus,
    Leverett,
    Novus,
    Lewis,
    Latham,
    Nelson,
    Oxford,
    Souter,
    Other,
    PlaterWhite,
    LewisShort,
    Translation,
    DuCange,
    Vademecum,
    PersonalGuess,
    TempSpecialCode,
    SentByUser,
    General,
    Unknown,
}

impl Source {
    pub fn as_str(&self) -> &'static str {
        match &self {
            Source::Beeson => "C.H.Beeson, A Primer of Medieval Latin, 1925 (Bee)",
            Source::Beard => "Charles Beard, Cassell's Latin Dictionary 1892 (CAS)",
            Source::Adams => "J.N.Adams, Latin Sexual Vocabulary",
            Source::Stelten => "L.F.Stelten, Dictionary of Eccles. Latin, 1995 (Ecc)",
            Source::Deferrari => "Roy J. Deferrari, Dictionary of St. Thomas Aquinas, 1960 (DeF)",
            Source::Gildersleeve => "Gildersleeve + Lodge, Latin Grammar 1895 (G+L)",
            Source::Collatinus => "Collatinus Dictionary by Yves Ouvrard",
            Source::Leverett => "Leverett, F.P., Lexicon of the Latin Language, Boston 1845",
            Source::Novus => "Calepinus Novus, modern Latin, by Guy Licoppe (Cal)",
            Source::Lewis => "Lewis, C.S., Elementary Latin Dictionary 1891",
            Source::Latham => "Latham, Revised Medieval Word List, 1980",
            Source::Nelson => "Lynn Nelson, Wordlist",
            Source::Oxford => "Oxford Latin Dictionary, 1982 (OLD)",
            Source::Souter => "Souter, A Glossary of Later Latin to 600 A.D., Oxford 1949",
            Source::Other => "Other, cited or unspecified dictionaries",
            Source::PlaterWhite => "Plater & White, A Grammar of the Vulgate, Oxford 1926",
            Source::LewisShort => "Lewis and Short, A Latin Dictionary, 1879 (L+S)",
            Source::Translation => "Found in a translation  --  no dictionary reference",
            Source::DuCange => "Du Cange",
            Source::Vademecum => "Vademecum in opus Saxonis - Franz Blatt (Saxo)",
            Source::PersonalGuess => "My personal guess",
            Source::TempSpecialCode => "Temp special code",
            Source::SentByUser => "Sent by user --  no dictionary reference",
            Source::General => "general",
            Source::Unknown => "unknown",
        }
    }

    pub fn dict_key_to_source(key: &str) -> Source {
        match key.trim_matches('"') {
            "B" => Source::Beeson,
            "C" => Source::Beard,
            "D" => Source::Adams,
            "E" => Source::Stelten,
            "F" => Source::Deferrari,
            "G" => Source::Gildersleeve,
            "H" => Source::Collatinus,
            "I" => Source::Leverett,
            "K" => Source::Novus,
            "L" => Source::Lewis,
            "M" => Source::Latham,
            "N" => Source::Nelson,
            "O" => Source::Oxford,
            "P" => Source::Souter,
            "Q" => Source::Other,
            "R" => Source::PlaterWhite,
            "S" => Source::LewisShort,
            "T" => Source::Translation,
            "U" => Source::DuCange,
            "V" => Source::Vademecum,
            "W" => Source::PersonalGuess,
            "Y" => Source::TempSpecialCode,
            "Z" => Source::SentByUser,
            "X" => Source::General,
            _ => Source::Unknown,
        }
    }
}
