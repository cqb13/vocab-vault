# Principle Parts Generation Module

This module is designed to generate principle parts for various types of words in Latin, including nouns, pronouns, adjectives, verbs, and numerals. It is based on the structure and implementation found in the [Whitaker's Words](https://github.com/mk270/whitakers-words) project. The module is divided into several submodules, each responsible for handling a specific type of word.

## Modules

- `generate_for_adjectives`: Handles the generation of principle parts for adjectives.
- `generate_for_nouns`: Handles the generation of principle parts for nouns.
- `generate_for_numerals`: Handles the generation of principle parts for numerals.
- `generate_for_pronouns`: Handles the generation of principle parts for pronouns.
- `generate_for_verbs`: Handles the generation of principle parts for verbs.

## Usage

To generate principle parts, the main function `generate_principle_parts` is used. It requires specifying the type of word (Generator) and the necessary parameters for that type. The function then delegates the task to the appropriate submodule based on the type of word.

### Parameters

- `generator`: The type of word for which principle parts are being generated. It must be one of the `Generator` enum values (`Noun`, `Pronoun`, `Adjective`, `Verb`, `Numeral`).
- `num_type_1` and `num_type_2`: Integer parameters used differently by each generator to specify the form or declension.
- `parts`: A vector of strings representing the base forms from which the principle parts are generated.
- `gender`: (Optional) Specifies the gender for nouns. Required for `Generator::Noun`.
- `comparison`: (Optional) Specifies the degree of comparison for adjectives. Required for `Generator::Adjective`.
- `verb_type`: (Optional) Specifies the type of verb. Required for `Generator::Verb`.
- `numeral_type`: (Optional) Specifies the type of numeral. Required for `Generator::Numeral`.

### Example

```rust
let parts = generate_principle_parts(
    Generator::Noun,
    2,
    1,
    vec!["discipul".to_string(), "discipul".to_string()],
    Some(Gender::Masculine),
    None,
    None,
    None,
);
```

## Principle Part Setting

The `set_principle_parts` function is used internally to append specified endings to the base parts or apply special cases. It takes a vector of base parts, a vector of tuples specifying the endings and the part number to which they should be appended, and an optional special case string.

## Submodule Functions

Each submodule (`generate_for_nouns`, `generate_for_adjectives`, etc.) implements specific logic to handle the generation of principle parts based on the type and characteristics of the word. These functions utilize `set_principle_parts` to construct the final principle parts.
