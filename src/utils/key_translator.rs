pub fn translate_comparison(key: &str) -> &str {
    match key {
        "POS" => "positive",
        "COMP" => "comparative",
        "SUPER" => "superlative",
        _ => "unknown",
    }
}

pub fn translate_declension(key: &str) -> &str {
    match key {
        "NOM" => "nominative",
        "GEN" => "genitive",
        "DAT" => "dative",
        "ACC" => "accusative",
        "VOC" => "vocative",
        "LOC" => "locative",
        "ABL" => "ablative",
        _ => "unknown",
    }
}

pub fn translate_gender(key: &str) -> &str {
    match key {
        "M" => "masculine",
        "F" => "feminine",
        "N" => "neuter",
        "C" => "common",
        _ => "unknown",
    }
}

pub fn translate_mood(key: &str) -> &str {
    match key {
        "IND" => "indicative",
        "SUB" => "subjunctive",
        "IMP" => "imperative",
        "INF" => "infinitive",
        _ => "unknown",
    }
}

pub fn translate_noun(key: &str) -> &str {
    match key {
        "S" => "singular",          
        "M" => "plural / multiple",
        "A" => "abstract idea",
        "G" => "group name (ex. Romans)",
        "N" => "proper name",
        "P" => "person",
        "T" => "thing",
        "L" => "location",
        "W" => "a place (where)",
        _ => "unknown",
    }
}

pub fn translate_numeral(key: &str) -> &str {
    match key {
        "CARD" => "cardinal",
        "ORD" => "ordinal",
        "DIST" => "distributive",
        "ADVERB" => "numeral adverb",
        _ => "unknown",
    }
}

pub fn translate_number(key: &str) -> &str {
    match key {
        "S" => "singular",
        "P" => "plural",
        _ => "unknown",
    }
}

pub fn translate_part_of_speech(key: &str) -> &str {
    match key {
        "N" => "noun",
        "V" => "verb",
        "VPAR" => "participle",
        "ADJ" => "adjective",
        "PREP" => "preposition",
        "PRON" => "pronoun",
        "INTERJ" => "interjection",
        "NUM" => "numeral",
        "CONJ" => "conjunction",
        "ADV" => "adverb",
        "INT" => "number",
        "SUPINE" => "supine",
        "PACK" => "packon",
        "TACKON" => "tackon",
        "PREFIC" => "prefix",
        "SUFFIX" => "suffix",
        _ => "unknown",
    }
}

pub fn translate_pronoun(key: &str) -> &str {
    match key {
        "PERS" => "personal",
        "DEMONS" => "demonstrative",
        "REL" => "relative",
        "INTERR" => "interrogative",
        "INDEF" => "indefinite",
        "REFLEX" => "reflexive",
        "ADJECT" => "adjective",
        _ => "unknown",
    }
}

pub fn translate_tense(key: &str) -> &str {
    match key {
        "PRES" => "present",
        "IMPF" => "imperfect",
        "FUT" => "future",
        "PERF" => "perfect",
        "PLUP" => "pluperfect",
        "FUTP" => "future perfect",
        "INF" => "infinitive",
        _ => "unknown",
    }
}

pub fn translate_verb(key: &str) -> &str {
    match key {
        "TO_BE" => "to be",
        "TO_BEING" => "to being",
        "GEN" => "takes genitive",
        "DAT" => "takes dative",
        "ABL" => "takes ablative",
        "TRANS" => "transitive",
        "INTRANS" => "intransitive",
        "IMPERS" => "impersonal (it/they/god)",
        "DEP" => "deponent",
        "SEMIDEP" => "semi-deponent",
        "PERFDEF" => "perfect definite",
        _ => "unknown",
    }
}

pub fn translate_voice(key: &str) -> &str {
    match key {
        "ACTIVE" => "active",
        "PASSIVE" => "passive",
        _ => "unknown",
    }
}
