# Vocab Vault

Vocab Vault is a port of [Open Words TS](https://github.com/Templar-Development/Open-Words-TS) TypeScript code to Rust for future maintenance and improvement.

Find the original Whitaker's Words written in Ada at https://github.com/dsanson/Words, thoughtfully documented and maintained by [dsanson](https://github.com/dsanson). More information about William Whitaker and the Words program is available there.

## CLI Usage Guide

The CLI is designed to provide translation functionality between English and Latin, using the Whitaker's Words Dictionary.

### Installation

#### From Source

To install and build from source, you must have [Rust](https://www.rust-lang.org/tools/install) installed. Then, run the following command:

```bash
$ git clone https://github.com/cqb13/vocab-vault.git
$ cd vocab-vault
$ cargo install --path .
$ cargo build --release
$ cargo run --release -- [command] [arguments]
```

#### From Binary

To install from a binary, download the latest release from the [releases page](https://github.com/cqb13/vocab-vault/releases)

```bash
$ cd [download directory]
$ vocab-vault [command] [arguments]
```

#### Website

You can also use the [website](https://learninglatin.net/translate) to translate text.

**Note:** The website is currently using the original TypeScript code, not the Rust code.

### Command Line Arguments

- `--help` or `-h`: Display help information

#### `transEng` Command (Translate English to Latin)

Translate English text to Latin using the following command:

```bash
$ vocab_vault transEng "English text to translate"
```

- `"English text to translate"`: The English text you want to translate
- `-m` or `--max ` `<MAX_ENTRIES>`: The maximum number of entries to return (default: 6)
- `-s` or `--sort`: Sort the output by frequency
- `-p` or `--pretty`: Display a pretty version of the output (requires `-f`)
- `-d` or `--detailed`: Add more information to prettified output (requires `-p`)

#### `transLat` Command (Translate Latin to English)

Translate Latin text to English using the following command:

```bash
$ vocab_vault transLat "Latin text to translate"
```

- `"Latin text to translate"`: The Latin text you want to translate
- `-t` or `--tricks`: Attempt to use various tricks on words for better results
- `-m` or `--max ` `<MAX_ENTRIES>`: The maximum number of entries to return (default: 6)
- `-s` or `--sort`: Sort the output by frequency
- `-p` or `--pretty`: Display a pretty version of the output (requires `-f`)
- `-d` or `--detailed`: Add more information to prettified output (requires `-p`)

#### `getList` Command (Gets a specific list of words from the dictionary)

Get a specific list of words from the dictionary using the following command:

```bash
$ vocab_vault getList "word_type"
```

- `"word_type"`: The type of word list you want to get
  - english, latin, inflections, not_packons, packons, prefixes, stems, suffixes, tackons, tickons, unique_latin
- `-p` or `--pos` `<part Of Speech List>`: A list of parts of speech to filter by (separated by commas)
  - noun, verb, participle, adjective, preposition, pronoun, interjection, numeral, conjunction, adverb, number, supine, packon, tackon, prefix, suffix
- `-m` or `--max` `<MAX_WORD_LEN>`: The maximum length of words to return
- `-n` or `--min` `<MIN_WORD_LEN>`: The minimum length of words to return
- `-e` or `--exact` `<EXACT>`: Only return words that match the exact length
- `-a` or `--amount` `<AMOUNT>`: The amount of words to return
- `-r` or `--random`: Picks words from random positions
- `-d` or `--display`: Display the words as json
- `-t` or `--to` `<TO>`: Saves the list of words to a json file

### Example Usage

Translate English to Latin with 2 options per translation which are sorted by frequency:

```bash
$ vocab_vault transEng "why" -m 2 -s
```

Translate Latin to English with tricks and pretty output:

```bash
$ vocab_vault transLat "cur" -t -p
```

Get a list of Latin words with a specific part of speech and save it to a file:

```bash
$ vocab_vault getList "latin" -p noun,verb -m 6 -n 3 -t "latin_words.json"
```
