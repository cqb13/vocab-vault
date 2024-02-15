use crate::dictionary_structures::dictionary_values::NValue;
use crate::dictionary_structures::dictionary_keys::PartOfSpeech;

pub fn translate_type(number_types: NValue, pos: PartOfSpeech) -> String {
    let num_type_1 = number_types.get_n_value_1();

    let type_name = match pos {
        PartOfSpeech::Noun => translate_noun_type(num_type_1),
        PartOfSpeech::Verb => translate_verb_type(num_type_1),
        PartOfSpeech::Adjective => translate_adjective_type(num_type_1),
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
