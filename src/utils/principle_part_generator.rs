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
        (1, 1) => set_principle_parts(parts, vec![("a", 1), ("ae", 2)], None),
        // greek nouns
        (1, 6) => set_principle_parts(parts, vec![("e", 1), ("es", 2)], None),
        (1, 7) => set_principle_parts(parts, vec![("es", 1), ("ae", 2)], None),
        (1, 8) => set_principle_parts(parts, vec![("as", 1), ("ae", 2)], None),
        // second declension
        (2, 1) => set_principle_parts(parts, vec![("us", 1), ("i", 2)], None),
        (2, 2) => set_principle_parts(parts, vec![("um", 1), ("i", 2)], None),
        (2, 3) => set_principle_parts(parts, vec![("", 1), ("i", 2)], None),
        (2, 4) => {
            if gender == "M" {
                set_principle_parts(parts, vec![("us", 1), ("(i)", 2)], None)
            } else if gender == "N" {
                set_principle_parts(parts, vec![("um", 1), ("(i)", 2)], None)
            } else {
                parts
            }
        }
        (2, 5) => set_principle_parts(parts, vec![("us", 1), ("", 2)], None),
        (2, 6) | (2, 7) => set_principle_parts(parts, vec![("os", 1), ("i", 2)], None),
        (2, 8) => set_principle_parts(parts, vec![("on", 1), ("i", 2)], None),
        (2, 9) => set_principle_parts(parts, vec![("us", 1), ("i", 2)], None),
        // third declension
        (3, 1) | (3, 2) | (3, 3) | (3, 4) => {
            set_principle_parts(parts, vec![("", 1), ("is", 2)], None)
        }
        (3, 7) | (3, 9) => set_principle_parts(parts, vec![("", 1), ("os/is", 2)], None),
        // fourth declension
        (4, 1) => set_principle_parts(parts, vec![("us", 1), ("us", 2)], None),
        (4, 2) => set_principle_parts(parts, vec![("u", 1), ("us", 2)], None),
        (4, 3) => set_principle_parts(parts, vec![("us", 1), ("u", 2)], None),
        // fifth declension
        (5, 1) => set_principle_parts(parts, vec![("es", 1), ("ei", 2)], None),
        // special
        (9, 8) => set_principle_parts(parts, vec![("", 0), ("", 0)], Some("abbreviation")),
        (9, 9) => set_principle_parts(parts, vec![("", 0), ("", 0)], Some("undeclined")),
        _ => parts,
    }
}

pub fn generate_for_pronouns(number_types: Vec<NValue>, parts: Vec<String>) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match (num_type_1, num_type_2) {
        // proximal demonstrative pronouns (hic, haec hoc)
        (3, 1) => set_principle_parts(parts, vec![("ic", 1), ("aec", 1), ("oc", 1)], None),
        (3, 2) => set_principle_parts(parts, vec![("ic", 1), ("aec", 1), ("uc", 1)], None),

        (4, 1) => set_principle_parts(parts, vec![("s", 1), ("a", 2), ("d", 1)], None),
        (4, 2) => set_principle_parts(parts, vec![("dem", 1), ("adem", 2), ("dem", 1)], None),
        // Distal (ille, illa, illud) and medial (iste, ista, istud)
        // demonstrative pronouns
        (6, 1) => set_principle_parts(parts, vec![("e", 1), ("a", 1), ("ud", 1)], None),
        (6, 2) => set_principle_parts(parts, vec![("e", 1), ("a", 1), ("um", 1)], None),
        // special
        (9, 8) => set_principle_parts(parts, vec![("", 0), ("", 0), ("", 0)], Some("abbreviation")),
        (9, 9) => set_principle_parts(parts, vec![("", 0), ("", 0), ("", 0)], Some("undeclined")),
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
        Comparison::COMP => set_principle_parts(parts, vec![("or", 1), ("or", 1), ("us", 1)], None),
        Comparison::SUPER => {
            set_principle_parts(parts, vec![("mus", 1), ("ma", 1), ("mum", 1)], None)
        }
        Comparison::POS => {
            match (num_type_1, num_type_2) {
                // first declension
                (1, 1) => set_principle_parts(parts, vec![("us", 1), ("a", 2), ("um", 2)], None),
                (1, 2) | (1, 4) => {
                    set_principle_parts(parts, vec![("", 1), ("a", 2), ("um", 2)], None)
                }
                (1, 3) => set_principle_parts(
                    parts,
                    vec![("us", 1), ("a", 2), ("um (gen -ius)", 2)],
                    None,
                ),
                (1, 5) => set_principle_parts(parts, vec![("us", 1), ("a", 2), ("ud", 2)], None),
                // second declension
                (2, 1) => set_principle_parts(parts, vec![("", 0), ("e", 1), ("", 0)], None),
                (2, 2) => set_principle_parts(parts, vec![("", 0), ("a", 0), ("", 0)], None),
                (2, 3) => set_principle_parts(parts, vec![("es", 1), ("es", 1), ("es", 1)], None),
                (2, 6) => set_principle_parts(parts, vec![("os", 1), ("os", 1), ("", 0)], None),
                (2, 7) => set_principle_parts(parts, vec![("os", 1), ("", 0), ("", 0)], None),
                (2, 8) => set_principle_parts(parts, vec![("", 0), ("", 0), ("on", 2)], None),
                // third declension
                (3, 1) => set_principle_parts(parts, vec![("", 1), ("(gen.)", 0), ("is", 2)], None),
                (3, 2) => set_principle_parts(parts, vec![("is", 1), ("is", 2), ("e", 2)], None),
                (3, 3) => set_principle_parts(parts, vec![("", 1), ("is", 2), ("e", 2)], None),
                (3, 6) => set_principle_parts(parts, vec![("", 1), ("(gen.)", 0), ("os", 2)], None),
                // special
                (9, 8) => set_principle_parts(
                    parts,
                    vec![("", 0), ("", 0), ("", 0)],
                    Some("abbreviation"),
                ),
                (9, 9) => {
                    set_principle_parts(parts, vec![("", 0), ("", 0), ("", 0)], Some("undeclined"))
                }
                _ => parts,
            }
        }
        Comparison::UNKNOWN => {
            match (num_type_1, num_type_2) {
                // unknown first declension
                (1, 1) => set_principle_parts(
                    parts,
                    vec![
                        ("us", 1),
                        ("a -um", 2),
                        ("or -or -us", 3),
                        ("mus -a -um", 4),
                    ],
                    None,
                ),
                (1, 2) => set_principle_parts(
                    parts,
                    vec![("", 1), ("a -um", 2), ("or -or -us", 3), ("mus -a -um", 4)],
                    None,
                ),
                // unknown third declension
                (3, 1) => set_principle_parts(
                    parts,
                    vec![
                        ("", 1),
                        ("is (gen .)", 2),
                        ("or -or -us", 3),
                        ("mus -a -um", 4),
                    ],
                    None,
                ),
                (3, 2) => set_principle_parts(
                    parts,
                    vec![("is", 1), ("e", 2), ("or -or -us", 3), ("mus -a -um", 4)],
                    None,
                ),
                (3, 3) => set_principle_parts(
                    parts,
                    vec![("", 1), ("is -e", 2), ("or -or -us", 3), ("mus -a -um", 4)],
                    None,
                ),
                // special
                (9, 8) => set_principle_parts(
                    parts,
                    vec![("", 0), ("", 0), ("", 0)],
                    Some("abbreviation"),
                ),
                (9, 9) => {
                    set_principle_parts(parts, vec![("", 0), ("", 0), ("", 0)], Some("undeclined"))
                }
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
        return set_principle_parts(
            parts,
            vec![("", 0), ("", 0), ("", 0), ("", 0)],
            Some("abbreviation"),
        );
    }

    if num_type_1 == 9 && num_type_2 == 9 {
        return set_principle_parts(
            parts,
            vec![("", 0), ("", 0), ("", 0), ("", 0)],
            Some("undeclined"),
        );
    }

    match verb_type {
        VerbType::DEP => match num_type_1 {
            1 => set_principle_parts(
                parts,
                vec![("or", 1), ("ari", 2), ("", 0), ("us sum", 4)],
                None,
            ),
            2 => set_principle_parts(
                parts,
                vec![("eor", 1), ("eri", 2), ("", 0), ("us sum", 4)],
                None,
            ),
            3 => {
                if num_type_2 == 4 {
                    set_principle_parts(
                        parts,
                        vec![("or", 1), ("iri", 2), ("", 0), ("us sum", 4)],
                        None,
                    )
                } else {
                    set_principle_parts(
                        parts,
                        vec![("or", 1), ("i", 2), ("", 0), ("us sum", 4)],
                        None,
                    )
                }
            }
            _ => parts,
        },
        VerbType::PERFDEF => {
            set_principle_parts(parts, vec![("i", 3), ("isse", 3), ("us", 4), ("", 0)], None)
        }
        _ => {
            if verb_type == VerbType::IMPERS && parts[0].trim() == "zzz" && parts[1].trim() == "zzz"
            {
                set_principle_parts(
                    parts,
                    vec![("it", 3), ("isse", 3), ("us est", 4), ("", 0)],
                    None,
                )
            } else if verb_type == VerbType::IMPERS {
                match num_type_1 {
                    1 => {
                        set_principle_parts(parts, vec![("at", 1), ("", 0), ("", 0), ("", 0)], None)
                    }
                    2 => {
                        set_principle_parts(parts, vec![("et", 1), ("", 0), ("", 0), ("", 0)], None)
                    }
                    3 => {
                        if num_type_2 == 2 {
                            set_principle_parts(
                                parts,
                                vec![("t", 1), ("", 0), ("", 0), ("", 0)],
                                None,
                            )
                        } else {
                            if parts[0].ends_with("i") {
                                set_principle_parts(
                                    parts,
                                    vec![("t", 1), ("", 0), ("", 0), ("", 0)],
                                    None,
                                )
                            } else {
                                set_principle_parts(
                                    parts,
                                    vec![("it", 1), ("", 0), ("", 0), ("", 0)],
                                    None,
                                )
                            }
                        }
                    }
                    5 => {
                        if num_type_2 == 1 {
                            set_principle_parts(
                                parts,
                                vec![("est", 1), ("", 0), ("", 0), ("", 0)],
                                None,
                            )
                        } else {
                            parts
                        }
                    }
                    7 => {
                        if num_type_2 == 1 || num_type_2 == 2 {
                            set_principle_parts(
                                parts,
                                vec![("t", 1), ("", 0), ("", 0), ("", 0)],
                                None,
                            )
                        } else {
                            parts
                        }
                    }
                    _ => parts,
                }
            } else {
                // building array instead of each case, because lots of options / overlap
                let mut ending_array = vec![("", 0); 4];

                // first part ending
                if num_type_1 == 2 {
                    ending_array[0] = ("eo", 1);
                } else if num_type_1 == 5 {
                    ending_array[0] = ("um", 1);
                } else if num_type_1 == 7 && num_type_2 == 2 {
                    ending_array[0] = ("am", 1);
                } else {
                    ending_array[0] = ("o", 1);
                }

                // second part ending
                match num_type_1 {
                    1 => ending_array[1] = ("are", 2),
                    2 => ending_array[1] = ("ere", 2),
                    3 => {
                        match num_type_2 {
                            2 => ending_array[1] = ("re", 2),
                            3 => {
                                // special case for fio, fieri: it follows the usual
                                // conjugation everywhere except for present infinitive
                                if parts[1].trim() == "f" {
                                    ending_array[1] = ("ieri", 2);
                                } else {
                                    ending_array[1] = ("eie", 2);
                                }
                            }
                            4 => ending_array[1] = ("ire", 2),
                            _ => ending_array[1] = ("ere", 2),
                        }
                    }
                    5 => {
                        if num_type_2 == 1 {
                            ending_array[1] = ("esse", 2);
                        } else if num_type_2 == 2 {
                            ending_array[1] = ("e", 1); // uses first part
                        }
                    }
                    6 => {
                        if num_type_2 == 1 {
                            ending_array[1] = ("ere", 2);
                        } else if num_type_2 == 2 {
                            ending_array[1] = ("le", 2);
                        }
                    }
                    7 => {
                        if num_type_2 == 2 {
                            ending_array[1] = ("iam", 2);
                        } else if num_type_2 == 3 {
                            ending_array[1] = ("se", 2);
                        }
                    }
                    8 => {
                        match num_type_2 {
                            1 => ending_array[1] = ("are", 2),
                            4 => ending_array[1] = ("ire", 2),
                            _ => ending_array[1] = ("ere", 2), // 2 & 3 & everything else
                        }
                    }
                    _ => ending_array[1] = ("", 0),
                }

                // third and fourth part endings
                if verb_type == VerbType::IMPERS {
                    // might be a 2 setting here, but it should be covered earlier
                    ending_array[3] = ("us est", 4);
                } else if verb_type == VerbType::SEMIDEP {
                    ending_array[3] = ("us sum", 4);
                } else if num_type_1 == 5 && num_type_2 == 1 {
                    ending_array[2] = ("i", 3);
                    ending_array[3] = ("urus", 4);
                } else if num_type_1 == 8 {
                    // additional forms, undefined
                } else {
                    ending_array[2] = ("i", 3);
                    ending_array[3] = ("us", 4);
                }

                // small correction
                if num_type_1 == 6 && num_type_2 == 1 {
                    ending_array[2] = ("(ii)", 3);
                }

                set_principle_parts(parts, ending_array, None)
            }
        }
    }
}

pub fn generate_for_numerals(
    number_types: Vec<NValue>,
    parts: Vec<String>,
    numeral_type: NumeralType,
) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    match numeral_type {
        NumeralType::UNKNOWN => match (num_type_1, num_type_2) {
            (1, 1) => set_principle_parts(
                parts,
                vec![("us -a -um", 1), ("us -a -um", 2), ("i -ae -a", 3), ("", 4)],
                None,
            ),
            (1, 2) => set_principle_parts(
                parts,
                vec![("o -ae o", 1), ("us -a -um", 2), ("i -ae -a", 3), ("", 4)],
                None,
            ),
            (1, 3) => set_principle_parts(
                parts,
                vec![
                    ("es -es -ia", 1),
                    ("us -a -um", 2),
                    ("i -ae -a", 3),
                    ("", 4),
                ],
                None,
            ),
            (1, 4) => set_principle_parts(
                parts,
                vec![
                    ("i -ae -a", 1),
                    ("us -a -um", 2),
                    ("i -ae -a", 3),
                    ("ie (n)s", 4),
                ],
                None,
            ),
            _ => {
                if num_type_1 == 2 {
                    set_principle_parts(
                        parts,
                        vec![("", 1), ("us -a -um", 2), ("i -ae -a", 3), ("ie (n)s", 4)],
                        None,
                    )
                } else {
                    parts
                }
            }
        },
        NumeralType::CARD => match (num_type_1, num_type_2) {
            (1, 1) => set_principle_parts(parts, vec![("us", 1), ("a", 1), ("um", 1)], None),
            (1, 2) => set_principle_parts(parts, vec![("o", 1), ("ae", 1), ("o", 1)], None),
            (1, 3) => set_principle_parts(parts, vec![("es", 1), ("es", 1), ("ia", 1)], None),
            (1, 4) => set_principle_parts(parts, vec![("i", 1), ("ae", 1), ("a", 1)], None),
            _ => parts,
        },
        NumeralType::ORD => set_principle_parts(parts, vec![("us", 1), ("a", 1), ("um", 1)], None),
        NumeralType::DIST => set_principle_parts(parts, vec![("i", 1), ("ae", 1), ("a", 1)], None),
    }
}

// first item in endings is the ending, and the second item is the number of the stem the ending is attached to
fn set_principle_parts(
    parts: Vec<String>,
    endings: Vec<(&str, i8)>,
    special_case: Option<&str>,
) -> Vec<String> {
    let mut principle_parts = Vec::new();

    if endings.iter().all(|x| x.0 == "" && x.1 == 0) {
        if special_case.is_none() {
            panic!("No Endings or Special Case provided");
        }
        return vec![parts[0].clone() + " | " + special_case.unwrap_or("")];
    }

    for ending in endings {
        if ending.0 == "" && ending.1 == 0 {
            // when there is no principle part
            principle_parts.push("---".to_string());
        } else if ending.0 == "" && ending.1 != 0 {
            // when the stem is the principle part
            principle_parts.push(parts[ending.1 as usize - 1].clone());
        } else if ending.0 != "" && ending.1 == 0 {
            // when the stem is replaced with the ending
            principle_parts.push(ending.0.to_string());
        } else {
            // adding
            principle_parts.push(parts[ending.1 as usize - 1].clone() + &ending.0);
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
