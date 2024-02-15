use crate::dictionary_structures::dictionary_keys::Gender;
use crate::utils::principle_part_generator::set_principle_parts;

pub fn generate_for_nouns(
    num_type_1: i8,
    num_type_2: i8,
    gender: Gender,
    parts: Vec<String>,
) -> Vec<String> {
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
            if gender == Gender::Masculine {
                set_principle_parts(parts, vec![("us", 1), ("(i)", 2)], None)
            } else if gender == Gender::Neuter {
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
