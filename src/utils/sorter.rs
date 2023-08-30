use crate::latin_to_english::Word;
use crate::{Translation, TranslationType};
use std::char;
use std::collections::HashMap;

const FREQUENCY_ORDER: [char; 10] = ['A', 'B', 'C', 'D', 'E', 'F', 'I', 'M', 'N', 'X'];

pub fn sort_output(mut translations: Vec<Translation>) -> Vec<Translation> {
    translations
        .iter_mut()
        .filter_map(|translation| {
            if let TranslationType::Latin(definitions) = &mut translation.definitions {
                // first item is the entries position within the original def list
                // the second item is the position of the entries frequency in the frequency order
                let mut entry_freq_map: HashMap<usize, usize> = HashMap::new();

                for (position, entry) in definitions.iter_mut().enumerate() {
                    let mut frequency = 0;
                    let letter_freq = match &entry.word {
                        Word::LatinWordInfo(word) => word.info.freq.clone(),
                        Word::UniqueLatinWordInfo(word) => word.info.freq.clone(),
                    };

                    for letter in letter_freq.chars() {
                        if let Some(pos) = FREQUENCY_ORDER.iter().position(|&x| x == letter) {
                            frequency += pos;
                        }
                    }

                    entry_freq_map.insert(position, frequency);
                }

                // if an entry has a higher freq than the entry before it, swap them
                // if an entry has the same freq as the entry before it, do nothing
                let mut sorted_entry_freq_map: Vec<(&usize, &usize)> =
                    entry_freq_map.iter().collect();
                sorted_entry_freq_map.sort_by(|a, b| a.1.cmp(b.1));

                let mut sorted_definitions = Vec::new();

                for (position, _) in sorted_entry_freq_map {
                    sorted_definitions.push(definitions[*position].clone());
                }

                return Some(Translation {
                    definitions: TranslationType::Latin(sorted_definitions),
                    ..translation.clone()
                });
            }

            None
        })
        .collect()
}
