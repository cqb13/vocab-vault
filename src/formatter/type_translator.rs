use crate::data::data::NValue;

pub fn translate_type(number_types: Vec<NValue>, pos: String) -> String {
    let (num_type_1, _num_type_2) = translate_number_types(number_types);

    let type_name = match pos.as_str() {
        "noun" => translate_noun_type(num_type_1),
        "verb" => translate_verb_type(num_type_1),
        "adjective" => translate_adjective_type(num_type_1),
        _ => "unknown".to_string(),
    };

    type_name
}

fn translate_noun_type(num_type_1: i8) -> String {
    let type_name = match num_type_1 {
        1 => "1st declension",
        2 => "2nd declension",
        3 => "3rd declension",
        4 => "4th declension",
        5 => "5th declension",
        9 => "irregular",
        _ => "unknown",
    };

    type_name.to_string()
}

fn translate_verb_type(num_type_1: i8) -> String {
    let type_name = match num_type_1 {
        1 => "1st conjugation",
        2 => "2nd conjugation",
        3 => "3rd conjugation",
        4 => "4th conjugation",
        9 => "irregular",
        _ => "unknown",
    };

    type_name.to_string()
}

fn translate_adjective_type(num_type_1: i8) -> String {
    let type_name = match num_type_1 {
        1 => "1st declension",
        2 => "2nd declension",
        3 => "3rd declension",
        9 => "irregular",
        _ => "unknown",
    };

    type_name.to_string()
}

pub fn translate_number_types(number_types: Vec<NValue>) -> (i8, i8) {
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
