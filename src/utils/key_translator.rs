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

pub fn translate_age(key: &str) -> &str {
    match key {
        "A" => "archaic",
        "B" => "early",
        "C" => "classical",
        "D" => "late",
        "E" => "later",
        "F" => "medieval",
        "G" => "scholar",
        "H" => "modern",
        "X" => "used throughout ages",
        _ => "unknown",
    }
}

pub fn translate_area(key: &str) -> &str {
    match key {
        "A" => "agriculture",
        "B" => "biological",
        "D" => "art",
        "E" => "religious",
        "G" => "grammar",
        "L" => "legal",
        "P" => "poetic",
        "S" => "scientific",
        "T" => "technical",
        "W" => "warfare",
        "Y" => "mythological",
        "X" => "all or none",
        _ => "unknown",
    }
}

pub fn translate_geo(key: &str) -> &str {
    match key {
        "A" => "Africa",
        "B" => "Britain",
        "C" => "China",
        "D" => "Scandinavia",
        "E" => "Egypt",
        "F" => "France / Gaul",
        "G" => "Germany",
        "H" => "Greece",
        "I" => "Italy / Rome",
        "J" => "India",
        "K" => "Balkans",
        "N" => "Netherlands",
        "P" => "Persia",
        "Q" => "Near East",
        "R" => "Russia",
        "S" => "Spain / Iberia",
        "U" => "Eastern Europe",
        "X" => "all or none",
        _ => "unknown",
    }
}

pub fn translate_frequency(key: &str) -> &str {
    match key {
        "A" => "very frequent",
        "B" => "frequent",
        "C" => "common",
        "D" => "lesser",
        "E" => "uncommon",
        "F" => "very rare",
        "I" => "inscription",
        "M" => "graffiti",
        "N" => "Pliny (only in Pliny Natural History)",
        "X" => "all or none",
        _ => "unknown",
    }
}

pub fn translate_source(key: &str) -> &str {
    match key {
        "B" => "C.H.Beeson, A Primer of Medieval Latin, 1925 (Bee)",
        "C" => "Charles Beard, Cassell's Latin Dictionary 1892 (CAS)",
        "D" => "J.N.Adams, Latin Sexual Vocabulary, 1982 (Sex)",
        "E" => "L.F.Stelten, Dictionary of Eccles. Latin, 1995 (Ecc)",
        "F" => "Roy J. Deferrari, Dictionary of St. Thomas Aquinas, 1960 (DeF)",
        "G" => "Gildersleeve + Lodge, Latin Grammar 1895 (G+L)",
        "H" => "Collatinus Dictionary by Yves Ouvrard",
        "I" => "Leverett, F.P., Lexicon of the Latin Language, Boston 1845",
        "K" => "Calepinus Novus, modern Latin, by Guy Licoppe (Cal)",
        "L" => "Lewis, C.S., Elementary Latin Dictionary 1891",
        "M" => "Latham, Revised Medieval Word List, 1980",
        "N" => "Lynn Nelson, Wordlist",
        "O" => "Oxford Latin Dictionary, 1982 (OLD)",
        "P" => "Souter, A Glossary of Later Latin to 600 A.D., Oxford 1949",
        "Q" => "Other, cited or unspecified dictionaries",
        "R" => "Plater & White, A Grammar of the Vulgate, Oxford 1926",
        "S" => "Lewis and Short, A Latin Dictionary, 1879 (L+S)",
        "T" => "Found in a translation  --  no dictionary reference",
        "U" => "Du Cange",
        "V" => "Vademecum in opus Saxonis - Franz Blatt (Saxo)",
        "W" => "My personal guess",
        "Y" => "Temp special code",
        "Z" => "Sent by user --  no dictionary reference",
        "X" => "general",
        _ => "unknown",
    }
}
