use super::data::NValue;

pub fn generate_for_nouns(
    number_types: Vec<NValue>,
    gender: String,
    parts: Vec<String>,
) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);
    //TODO account for 0 in num2

    match (num_type_1, num_type_2) {
        (1, 1) => set_principle_parts(parts, vec!["a", "ae"], None),
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
        (3, 1) | (3, 2) | (3, 3) | (3, 4) => set_principle_parts(parts, vec!["", "is"], None),
        (4, 1) => set_principle_parts(parts, vec!["us", "us"], None),
        (4, 2) => set_principle_parts(parts, vec!["u", "us"], None),
        (4, 3) => set_principle_parts(parts, vec!["us", "u"], None),
        (5, 1) => set_principle_parts(parts, vec!["es", "ei"], None),
        (9, 8) => set_principle_parts(parts, vec!["", ""], Some("abbreviation")),
        (9, 9) => set_principle_parts(parts, vec!["", ""], Some("undeclined")),
        _ => parts,
    }
}

//pub fn generate_for_adjectives(number_types: Vec<NValue>, parts: Vec<String>) -> Vec<String> {
//
//}
//

pub fn generate_for_verbs(number_types: Vec<NValue>, parts: Vec<String>) -> Vec<String> {
    let (num_type_1, num_type_2) = translate_number_types(number_types);

    //TODO account for 0 in num2

    match (num_type_1, num_type_2) {
        (1, 1) => set_principle_parts(parts, vec!["o", "are", "i", "us"], None),
        (2, 1) => set_principle_parts(parts, vec!["eo", "ere", "i", "us"], None),
        (3, 1) => set_principle_parts(parts, vec!["o", "ere", "i", "us"], None),
        (3, 2) => set_principle_parts(parts, vec!["o", "re", "i", "us"], None),
        (3, 3) => set_principle_parts(parts, vec!["o", "eri", "i", "us"], None),
        (3, 4) => set_principle_parts(parts, vec!["o", "ire", "i", "us"], None),
        (5, 2) => set_principle_parts(parts, vec!["um", "esse", "i", ""], None),
        (6, 1) => set_principle_parts(parts, vec!["o", "re", "i", "us"], None),
        (6, 2) => set_principle_parts(parts, vec!["o", "le", "i", ""], None),
        (7, 1) => set_principle_parts(parts, vec!["o", "", "", ""], None),
        (7, 2) => set_principle_parts(parts, vec!["am", "iam", "", ""], None),
        (7, 3) => set_principle_parts(parts, vec!["o", "se", "", ""], None),
        (8, 1) => set_principle_parts(parts, vec!["o", "are", "i", ""], None),
        (8, 2) => set_principle_parts(parts, vec!["o", "ere", "", ""], None),
        (8, 3) => set_principle_parts(parts, vec!["o", "ere", "i", ""], None),
        (9, 9) => set_principle_parts(parts, vec!["", "", "", ""], Some("undeclined")),
        _ => parts,
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
        if part == "" || endings[i] == "zzz" {
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
