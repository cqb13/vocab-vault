#[derive(Debug)]
pub enum WordType {
    English,
    Latin,
    Inflections,
    NotPackons,
    Packons,
    Prefixes,
    Stems,
    Suffixes,
    Tackons,
    Tickons,
    UniqueLatin,
}

impl WordType {
    pub fn from_str(s: &str) -> Result<WordType, String> {
        match s {
            "english" => Ok(WordType::English),
            "latin" => Ok(WordType::Latin),
            "inflections" => Ok(WordType::Inflections),
            "not_packons" => Ok(WordType::NotPackons),
            "packons" => Ok(WordType::Packons),
            "prefixes" => Ok(WordType::Prefixes),
            "stems" => Ok(WordType::Stems),
            "suffixes" => Ok(WordType::Suffixes),
            "tackons" => Ok(WordType::Tackons),
            "tickons" => Ok(WordType::Tickons),
            "unique_latin" => Ok(WordType::UniqueLatin),
            _ => Err(format!("Invalid word type: {}", s)),
        }
    }
}
