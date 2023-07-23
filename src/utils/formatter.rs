use regex::Regex;

pub fn format() {

}

pub fn sanitize_word(word: &str) -> String {
    let mut word = word.to_owned();
    word = word.trim().to_lowercase();
    let re = Regex::new(r"[^a-z ]|\d|\s+").unwrap();
    word = re.replace_all(&word, " ").to_string();
    word
}