use crate::dictionary_structures::dictionary_keys::Verb;
use crate::utils::principle_part_generator::set_principle_parts;

pub fn generate_for_verbs(
    num_type_1: i8,
    num_type_2: i8,
    parts: Vec<String>,
    verb_type: Verb,
) -> Vec<String> {
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
        Verb::Deponent => match num_type_1 {
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
        Verb::PerfectDefinite => {
            set_principle_parts(parts, vec![("i", 3), ("isse", 3), ("us", 4), ("", 0)], None)
        }
        _ => {
            if verb_type == Verb::Impersonal && parts[0].trim() == "zzz" && parts[1].trim() == "zzz"
            {
                set_principle_parts(
                    parts,
                    vec![("it", 3), ("isse", 3), ("us est", 4), ("", 0)],
                    None,
                )
            } else if verb_type == Verb::Impersonal {
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
                if verb_type == Verb::Impersonal {
                    // might be a 2 setting here, but it should be covered earlier
                    ending_array[3] = ("us est", 4);
                } else if verb_type == Verb::SemiDeponent {
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
