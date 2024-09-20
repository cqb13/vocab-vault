# Output Structure

Vocab Vault has two output formats: `json` and a custom `pretty` format like the one used in Whitakers Words.

## JSON

JSON is the default output format. It is a simple format that is easy to parse and use in other programs. by default all fields are present, but adding the `-c` or `--clean` flag will remove all fields that are not present in the entry.

### Latin to English

```json
[
  {
    "word": "searched word",
    "definitions": [
      {
        "tricks": [], // a list of modification applied to the searched word
        "word": {
          "orth": "word",
          "parts": ["first part", "second part", "third part", "fourth part"],
          "senses": ["meaning 1", "meaning 2", "meaning 3"],
          "pos": "part of speech",
          "form": {
            // note that that in clean format, not all of these fields are present
            "declension": "declension of hte word",
            "number": "singular / plural",
            "gender": "gender of the word",
            "tense": "tense of the word",
            "voice": "voice that the word is used with",
            "mood": "the mood of the word",
            "verb": "the kind of verb that the word is",
            "kind": "the kind of word that it is",
            "person": 0 // 1st, 2nd, 3rd person
          },
          "info": {
            "age": "when it was used",
            "area": "the field in which it was used",
            "geo": "the area in which it was used",
            "freq": "frequency of use",
            "source": "source for the word in the dictionary"
          },
          "n": [0, 0],
          "modifiers": [
            // prefixes and suffixes or other modifiers
            {
              "pos": "part of speech",
              "senses": ["meaning 1", "meaning 2", "meaning 3"],
              "orth": "modifier word",
              "modifier": "type of modifier"
            }
          ],
          "id": 0, // id of the latin word
          "extension_senses": ["other meanings of the word"]
        },
        "stem": {
          "pos": "noun",
          "form": {
            "declension": "2nd declension",
            "number": "",
            "gender": "masculine",
            "tense": "",
            "voice": "",
            "mood": "",
            "verb": "",
            "kind": "person",
            "person": 0
          },
          "orth": "discipul",
          "n": [0, 0],
          "wid": 18070
        },
        "inflections": [
          // a list of all possible inflections for the word
          {
            "ending": "i",
            "pos": "noun",
            "note": "",
            "n": [2, 1],
            "form": {
              "declension": "genitive",
              "number": "singular",
              "gender": "unknown",
              "tense": "",
              "voice": "",
              "mood": "",
              "verb": "",
              "kind": "",
              "person": 0
            }
          }
        ],
        "addon": ""
      }
    ]
  }
]
```

### English to Latin

```json
[
  {
    "word": "searched word",
    "definitions": [
      {
        "word": {
          "orth": "word",
          "wid": 0, // id of the latin word
          "pos": "part of speech",
          "frequency_type": "very frequent",
          "true_frequency": 0, // frequency + compound - semi
          "frequency": 0, // frequency in latin language
          "compound": 0,
          "semi": 0
        },
        "translation": {
          "orth": "latin word",
          "parts": ["first part", "second part", "third part", "fourth part"],
          "senses": ["meaning 1", "meaning 2", "meaning 3"],
          "pos": "part of speech",
          "form": {
            // note that that in clean format, not all of these fields are present
            "declension": "declension of hte word",
            "number": "singular / plural",
            "gender": "gender of the word",
            "tense": "tense of the word",
            "voice": "voice that the word is used with",
            "mood": "the mood of the word",
            "verb": "the kind of verb that the word is",
            "kind": "the kind of word that it is",
            "person": 0 // 1st, 2nd, 3rd person
          },
          "info": {
            "age": "when it was used",
            "area": "the field in which it was used",
            "geo": "the area in which it was used",
            "freq": "frequency of use",
            "source": "source for the word in the dictionary"
          },
          "n": [
            // information on word type
            0, 0
          ],
          "id": 0 // id of the latin word
        }
      }
    ]
  }
]
```

## Pretty Output

The pretty output format is a custom format that is designed to be easy to read and use. It is designed to be similar to the output format used in Whitakers Words. To use this format add the `-p` or `--pretty` flag. The pretty format is designed to be used with the `-c` or `--clean` flag. Some information is hidden in the format, to make it visible add the `-d` or `--detailed` flag.

### Format

```text
searched word

word
part of speech
information on word type
age: when it was used | area: the field in which it was used | geo: the area in which it was used | freq: frequency of use | source: source for the word in the dictionary // only present if -d flag is used
meaning 1 | meaning 2 | meaning 3

word
part of speech
information on word type
age: when it was used | area: the field in which it was used | geo: the area in which it was used | freq: frequency of use | source: source for the word in the dictionary // only present if -d flag is used
meaning 1 | meaning 2 | meaning 3

word
part of speech
information on word type
age: when it was used | area: the field in which it was used | geo: the area in which it was used | freq: frequency of use | source: source for the word in the dictionary // only present if -d flag is used
meaning 1 | meaning 2 | meaning 3

---------------------------------
second searched word

word
part of speech
information on word type
age: when it was used | area: the field in which it was used | geo: the area in which it was used | freq: frequency of use | source: source for the word in the dictionary // only present if -d flag is used
meaning 1 | meaning 2 | meaning 3
```
