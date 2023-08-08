use crate::data::data::{Form, Inflection, LatinWordInfo};

pub fn entry_has_unknown_parts(latin_word_info: LatinWordInfo) -> bool {
    let parts = latin_word_info.parts;

    if parts.contains(&"---".to_string()) || parts.contains(&"zzz".to_string()) {
        return true;
    }

    false
}

pub fn filter_inflections(
    inflections: Vec<Inflection>,
    pos: String,
    clean: bool,
) -> Vec<Inflection> {
    let mut clean_inflections: Vec<Inflection> = inflections;

    clean_inflections = remove_inflections_without_endings(clean_inflections);
    clean_inflections = remove_inflections_with_wrong_pos(clean_inflections, pos);

    if clean {
        clean_inflections = remove_vague_inflections(clean_inflections);
    }

    clean_inflections
}

fn remove_inflections_without_endings(inflections: Vec<Inflection>) -> Vec<Inflection> {
    let mut clean_inflections: Vec<Inflection> = Vec::new();

    for inflection in inflections {
        if inflection.ending != "" {
            clean_inflections.push(inflection);
        }
    }

    clean_inflections
}

// Canis generates with a pos of "verb", but is a noun. This removes those.
fn remove_inflections_with_wrong_pos(inflections: Vec<Inflection>, pos: String) -> Vec<Inflection> {
    let mut clean_inflections: Vec<Inflection> = Vec::new();

    for inflection in inflections {
        if inflection.pos == pos {
            clean_inflections.push(inflection);
        }
    }

    clean_inflections
}

fn remove_vague_inflections(inflections: Vec<Inflection>) -> Vec<Inflection> {
    let mut clean_inflections: Vec<Inflection> = Vec::new();

    for inflection in inflections {
        let string_form: String = match inflection.form.clone() {
            Form::LongForm(_) => "X".to_string(),
            Form::StrForm(string_form) => string_form,
        };

        if !string_form.contains("X") && !string_form.contains("unknown") {
            clean_inflections.push(inflection);
        }
    }

    clean_inflections
}
