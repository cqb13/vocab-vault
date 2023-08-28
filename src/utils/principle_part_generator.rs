// based on: https://github.com/mk270/whitakers-words/blob/9b11477e53f4adfb17d6f6aa563669dc71e0a680/src/support_utils/support_utils-dictionary_form.adb
use std::vec;

use crate::data::data::NValue;

pub enum Comparison {
    POS,
    COMP,
    SUPER,
    UNKNOWN,
}

impl Comparison {
    pub fn from_str(s: &str) -> Comparison {
        match s {
            "POS" => Comparison::POS,
            "COMP" => Comparison::COMP,
            "SUPER" => Comparison::SUPER,
            _ => Comparison::UNKNOWN,
        }
    }
}

pub enum VerbType {
    DEP,
    PERFDEF,
    IMPERS,
    SEMIDEP,
    OTHER,
}

impl VerbType {
    pub fn from_str(s: &str) -> VerbType {
        match s {
            "DEP" => VerbType::DEP,
            "PERFDEF" => VerbType::PERFDEF,
            "IMPERS" => VerbType::IMPERS,
            "SEMIDEP" => VerbType::SEMIDEP,
            _ => VerbType::OTHER,
        }
    }
}

impl PartialEq for VerbType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VerbType::DEP, VerbType::DEP) => true,
            (VerbType::PERFDEF, VerbType::PERFDEF) => true,
            (VerbType::IMPERS, VerbType::IMPERS) => true,
            (VerbType::SEMIDEP, VerbType::SEMIDEP) => true,
            _ => false,
        }
    }
}

pub enum NumeralType {
    CARD,
    ORD,
    DIST,
    UNKNOWN,
}

impl NumeralType {
    pub fn from_str(s: &str) -> NumeralType {
        match s {
            "CARD" => NumeralType::CARD,
            "ORD" => NumeralType::ORD,
            "DIST" => NumeralType::DIST,
            _ => NumeralType::UNKNOWN,
        }
    }
}

pub fn generate_for_nouns(
    number_types: Vec<NValue>,
    gender: String,
    parts: Vec<String>,
) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match (num_type_1, num_type_2) {
        // first declension
        (1, 1) => set_principle_parts(parts, vec!["a", "ae"], None),
        // greek nouns
        (1, 6) => set_principle_parts(parts, vec!["e", "es"], None),
        (1, 7) => set_principle_parts(parts, vec!["es", "ae"], None),
        (1, 8) => set_principle_parts(parts, vec!["as", "ae"], None),
        // second declension
        (2, 1) => set_principle_parts(parts, vec!["us", "i"], None),
        (2, 2) => set_principle_parts(parts, vec!["um", "i"], None),
        (2, 3) => set_principle_parts(parts, vec!["", "i"], None),
        (2, 4) => {
            if gender == "M" {
                set_principle_parts(parts, vec!["us", "(i)"], None)
            } else if gender == "N" {
                set_principle_parts(parts, vec!["um", "(i)"], None)
            } else {
                parts
            }
        }
        (2, 5) => set_principle_parts(parts, vec!["us", ""], None),
        (2, 6) | (2, 7) => set_principle_parts(parts, vec!["os", "i"], None),
        (2, 8) => set_principle_parts(parts, vec!["on", "i"], None),
        (2, 9) => set_principle_parts(parts, vec!["us", "i"], None),
        // third declension
        (3, 1) | (3, 2) | (3, 3) | (3, 4) => set_principle_parts(parts, vec!["", "is"], None),
        (3, 7) | (3, 9) => set_principle_parts(parts, vec!["", "os/is"], None),
        // fourth declension
        (4, 1) => set_principle_parts(parts, vec!["us", "us"], None),
        (4, 2) => set_principle_parts(parts, vec!["u", "us"], None),
        (4, 3) => set_principle_parts(parts, vec!["us", "u"], None),
        // fifth declension
        (5, 1) => set_principle_parts(parts, vec!["es", "ei"], None),
        // special
        (9, 8) => set_principle_parts(parts, vec!["", ""], Some("abbreviation")),
        (9, 9) => set_principle_parts(parts, vec!["", ""], Some("undeclined")),
        _ => parts,
    }
}

pub fn generate_for_pronouns(number_types: Vec<NValue>, parts: Vec<String>) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match (num_type_1, num_type_2) {
        // proximal demonstrative pronouns (hic, haec hoc)
        (3, 1) => set_principle_parts(parts, vec!["ic", "aec", "oc"], None),
        (3, 2) => set_principle_parts(parts, vec!["ic", "aec", "uc"], None),

        (4, 1) => set_principle_parts(parts, vec!["s", "a", "d"], None),
        (4, 2) => set_principle_parts(parts, vec!["dem", "adem", "dem"], None),
        // Distal (ille, illa, illud) and medial (iste, ista, istud)
        // demonstrative pronouns
        (6, 1) => set_principle_parts(parts, vec!["e", "a", "ud"], None),
        (6, 2) => set_principle_parts(parts, vec!["e", "a", "um"], None),
        // special
        (9, 8) => set_principle_parts(parts, vec!["", "", ""], Some("abbreviation")),
        (9, 9) => set_principle_parts(parts, vec!["", "", ""], Some("undeclined")),
        _ => parts,
    }
}

pub fn generate_for_adjectives(
    number_types: Vec<NValue>,
    parts: Vec<String>,
    comparison: Comparison,
) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match comparison {
        Comparison::COMP => set_principle_parts(parts, vec!["or", "or", "us"], None),
        Comparison::SUPER => set_principle_parts(parts, vec!["mus", "ma", "mum"], None),
        Comparison::POS => {
            match (num_type_1, num_type_2) {
                // first declension
                (1, 1) => set_principle_parts(parts, vec!["us", "a", "um"], None),
                (1, 2) | (1, 4) => set_principle_parts(parts, vec!["", "a", "um"], None),
                (1, 3) => set_principle_parts(parts, vec!["us", "a", "um (gen -ius)"], None),
                (1, 5) => set_principle_parts(parts, vec!["us", "a", "ud"], None),
                // second declension
                (2, 1) => set_principle_parts(parts, vec!["", "e", ""], None),
                (2, 2) => set_principle_parts(parts, vec!["", "a", ""], None),
                (2, 3) => set_principle_parts(parts, vec!["es", "es", "es"], None),
                (2, 6) => set_principle_parts(parts, vec!["os", "os", ""], None),
                (2, 7) => set_principle_parts(parts, vec!["os", "", ""], None),
                (2, 8) => set_principle_parts(parts, vec!["", "", "on"], None),
                // third declension
                (3, 1) => set_principle_parts(parts, vec!["", "", "is"], None),
                (3, 2) => set_principle_parts(parts, vec!["is", "is", "e"], None),
                (3, 3) => set_principle_parts(parts, vec!["", "is", "e"], None),
                (3, 6) => set_principle_parts(parts, vec!["", "", "os"], None),
                // special
                (9, 8) => set_principle_parts(parts, vec!["", "", ""], Some("abbreviation")),
                (9, 9) => set_principle_parts(parts, vec!["", "", ""], Some("undeclined")),
                _ => parts,
            }
        }
        Comparison::UNKNOWN => {
            match (num_type_1, num_type_2) {
                // unknown first declension
                (1, 1) => set_principle_parts(
                    parts,
                    vec!["us", "a -um", "or -or -us", "mus -a -um"],
                    None,
                ),
                (1, 2) => {
                    set_principle_parts(parts, vec!["", "a -um", "or -or -us", "mus -a -um"], None)
                }
                // unknown third declension
                (3, 1) => set_principle_parts(
                    parts,
                    vec!["", "is (gen .)", "or -or -us", "mus -a -um"],
                    None,
                ),
                (3, 2) => {
                    set_principle_parts(parts, vec!["is", "e", "or -or -us", "mus -a -um"], None)
                }
                (3, 3) => {
                    set_principle_parts(parts, vec!["", "is -e", "or -or -us", "mus -a -um"], None)
                }
                // special
                (9, 8) => set_principle_parts(parts, vec!["", "", ""], Some("abbreviation")),
                (9, 9) => set_principle_parts(parts, vec!["", "", ""], Some("undeclined")),
                _ => parts,
            }
        }
    }
}

pub fn generate_for_verbs(
    number_types: Vec<NValue>,
    parts: Vec<String>,
    verb_type: VerbType,
) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    if num_type_1 == 9 && num_type_2 == 8 {
        return set_principle_parts(parts, vec!["", "", "", ""], Some("abbreviation"));
    }

    if num_type_1 == 9 && num_type_2 == 9 {
        return set_principle_parts(parts, vec!["", "", "", ""], Some("undeclined"));
    }

    match verb_type {
        VerbType::DEP => match num_type_1 {
            1 => set_principle_parts(parts, vec!["or", "ari", "", "us sum"], None),
            2 => set_principle_parts(parts, vec!["eor", "eri", "", "us sum"], None),
            3 => {
                if num_type_2 == 4 {
                    set_principle_parts(parts, vec!["or", "iri", "", "us sum"], None)
                } else {
                    set_principle_parts(parts, vec!["or", "i", "", "us sum"], None)
                }
            }
            _ => parts,
        },
        VerbType::PERFDEF => set_principle_parts(parts, vec!["i", "isse", "us", ""], None),
        _ => {
            if verb_type == VerbType::IMPERS && parts[0].trim() == "zzz" && parts[1].trim() == "zzz"
            {
                set_principle_parts(parts, vec!["it", "isse", "us est", ""], None)
            } else if verb_type == VerbType::IMPERS {
                match num_type_1 {
                    1 => set_principle_parts(parts, vec!["at", "", "", ""], None),
                    2 => set_principle_parts(parts, vec!["et", "", "", ""], None),
                    3 => {
                        if num_type_2 == 2 {
                            set_principle_parts(parts, vec!["t", "", "", ""], None)
                        } else {
                            if parts[0].ends_with("i") {
                                set_principle_parts(parts, vec!["t", "", "", ""], None)
                            } else {
                                set_principle_parts(parts, vec!["it", "", "", ""], None)
                            }
                        }
                    }
                    5 => {
                        if num_type_2 == 1 {
                            set_principle_parts(parts, vec!["est", "", "", ""], None)
                        } else {
                            parts
                        }
                    }
                    7 => {
                        if num_type_2 == 1 || num_type_2 == 2 {
                            set_principle_parts(parts, vec!["t", "", "", ""], None)
                        } else {
                            parts
                        }
                    }
                    _ => parts,
                }
            } else {
                // building array instead of each case, because lots of options / overlap
                let mut ending_array = vec![""; 4];

                // first part ending
                if num_type_1 == 2 {
                    ending_array[0] = "eo";
                } else if num_type_1 == 5 {
                    ending_array[0] = "um";
                } else if num_type_1 == 7 && num_type_2 == 2 {
                    ending_array[0] = "am";
                } else {
                    ending_array[0] = "o";
                }

                // second part ending
                match num_type_1 {
                    1 => ending_array[1] = "are",
                    2 => ending_array[1] = "ere",
                    3 => {
                        match num_type_2 {
                            2 => ending_array[1] = "re",
                            3 => {
                                // special case for fio, fieri: it follows the usual
                                // conjugation everywhere except for present infinitive
                                if parts[1].trim() == "f" {
                                    ending_array[1] = "ieri";
                                } else {
                                    ending_array[1] = "eie";
                                }
                            }
                            4 => ending_array[1] = "ire",
                            _ => ending_array[1] = "ere",
                        }
                    }
                    5 => ending_array[1] = "esse", // simplified
                    6 => {
                        if num_type_2 == 1 {
                            ending_array[1] = "ere";
                        } else if num_type_2 == 2 {
                            ending_array[1] = "le";
                        }
                    }
                    7 => {
                        if num_type_2 == 2 {
                            ending_array[1] = "iam";
                        } else if num_type_2 == 3 {
                            ending_array[1] = "se";
                        }
                    }
                    8 => {
                        match num_type_2 {
                            1 => ending_array[1] = "are",
                            4 => ending_array[1] = "ire",
                            _ => ending_array[1] = "ere", // 2 & 3 & everything else
                        }
                    }
                    _ => ending_array[1] = "",
                }

                // third and fourth part endings
                if verb_type == VerbType::IMPERS {
                    // might be a 2 setting here, but it should be covered earlier
                    ending_array[3] = "us est";
                } else if verb_type == VerbType::SEMIDEP {
                    ending_array[3] = "us sum";
                } else if num_type_1 == 5 && num_type_2 == 1 {
                    ending_array[2] = "i";
                    ending_array[3] = "urus";
                } else if num_type_1 == 8 {
                    // additional forms, undefined
                } else {
                    ending_array[2] = "i";
                    ending_array[3] = "us";
                }

                // small correction
                if num_type_1 == 6 && num_type_2 == 1 {
                    ending_array[2] = "(ii)";
                }

                set_principle_parts(parts, ending_array, None)
            }
        }
    }
}

pub fn generate_for_numerals(number_types: Vec<NValue>, parts: Vec<String>, numeral_type: NumeralType) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match numeral_type {
        NumeralType::UNKNOWN => {
            match (num_type_1, num_type_2) {
                (1, 1) => set_principle_parts(parts, vec!["us -a -um", "us -a -um", "i -ae -a", ""], None), 
                (1, 2) => set_principle_parts(parts, vec!["o -ae o", "us -a -um", "i -ae -a", ""], None),
                (1, 3) => set_principle_parts(parts, vec!["es -es -ia", "us -a -um", "i -ae -a", ""], None),
                (1, 4) => set_principle_parts(parts, vec!["i -ae -a", "us -a -um", "i -ae -a", "ie (n)s"], None),
                _ => {
                    if num_type_1 == 2 {
                        set_principle_parts(parts, vec!["", "us -a -um", "i -ae -a", "ie (n)s"], None)
                    } else {
                        parts
                    }
                }
            }
        }
        NumeralType::CARD => {
            match (num_type_1, num_type_2) {
                (1, 1) => set_principle_parts(parts, vec!["us", "a", "um"], None), 
                (1, 2) => set_principle_parts(parts, vec!["o", "ae", "o"], None),
                (1, 3) => set_principle_parts(parts, vec!["es", "es", "ia"], None),
                (1, 4) => set_principle_parts(parts, vec!["i", "ae", "a"], None),
                _ => parts,
            }
        }
        NumeralType::ORD => set_principle_parts(parts, vec!["us", "a", "um"] , None),
        NumeralType::DIST => set_principle_parts(parts, vec!["i", "ae", "a"] , None),
    }
}

fn set_principle_parts(
    parts: Vec<String>,
    endings: Vec<&str>,
    special_case: Option<&str>,
) -> Vec<String> {
    let mut principle_parts = Vec::new();

    if endings.iter().all(|x| *x == "") {
        return vec![parts[0].clone() + " | " + special_case.unwrap_or("")];
    }

    for (i, part) in parts.iter().enumerate() {
        if part == "" || part == "zzz" {
            principle_parts.push("---".to_string());
        } else {
            principle_parts.push(part.clone() + endings[i]);
        }
    }

    principle_parts
}

fn translate_number_types(number_types: Vec<NValue>) -> (i8, i8) {
    let num_type_1 = match &number_types.get(0) {
        Some(NValue::Integer(num)) => *num,
        _ => 0,
    };

    let num_type_2 = match &number_types.get(1) {
        Some(NValue::Integer(num)) => *num,
        _ => 0,
    };

    (num_type_1, num_type_2)
}
