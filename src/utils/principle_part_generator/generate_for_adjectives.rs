use crate::dictionary_structures::dictionary_keys::Comparison;
use crate::utils::principle_part_generator::set_principle_parts;

pub fn generate_for_adjectives(
    num_type_1: i8,
    num_type_2: i8,
    parts: Vec<String>,
    comparison: Comparison,
) -> Vec<String> {
    match comparison {
        Comparison::Comparative => {
            set_principle_parts(parts, vec![("or", 1), ("or", 1), ("us", 1)], None)
        }
        Comparison::Superlative => {
            set_principle_parts(parts, vec![("mus", 1), ("ma", 1), ("mum", 1)], None)
        }
        Comparison::Positive => {
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
        Comparison::Unknown => {
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
