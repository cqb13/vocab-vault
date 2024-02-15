use crate::utils::principle_part_generator::set_principle_parts;

pub fn generate_for_pronouns(num_type_1: i8, num_type_2: i8, parts: Vec<String>) -> Vec<String> {
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
