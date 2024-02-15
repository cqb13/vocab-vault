use crate::dictionary_structures::dictionary_keys::Numeral;
use crate::utils::principle_part_generator::set_principle_parts;

pub fn generate_for_numerals(
    num_type_1: i8,
    num_type_2: i8,
    parts: Vec<String>,
    numeral_type: Numeral,
) -> Vec<String> {
    match numeral_type {
        Numeral::Unknown | Numeral::Adverbial => match (num_type_1, num_type_2) {
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
        Numeral::Cardinal => match (num_type_1, num_type_2) {
            (1, 1) => set_principle_parts(parts, vec![("us", 1), ("a", 1), ("um", 1)], None),
            (1, 2) => set_principle_parts(parts, vec![("o", 1), ("ae", 1), ("o", 1)], None),
            (1, 3) => set_principle_parts(parts, vec![("es", 1), ("es", 1), ("ia", 1)], None),
            (1, 4) => set_principle_parts(parts, vec![("i", 1), ("ae", 1), ("a", 1)], None),
            _ => parts,
        },
        Numeral::Ordinal => set_principle_parts(parts, vec![("us", 1), ("a", 1), ("um", 1)], None),
        Numeral::Distributive => {
            set_principle_parts(parts, vec![("i", 1), ("ae", 1), ("a", 1)], None)
        }
    }
}
