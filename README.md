# Open Words

Latin Words is a port of [Open Words TS](https://github.com/Templar-Development/Open-Words-TS) TypeScript code to Rust for future maintenance and improvement.

Find the original Whitaker's Words written in Ada at https://github.com/dsanson/Words, thoughtfully documented and maintained by [dsanson](https://github.com/dsanson).  More information about William Whitaker and the Words program is available there.  

## CLI Usage Guide

The CLI is designed to provide translation functionality between English and Latin, using the Whitaker's Words Dictionary.

### Installation

#### From Source

To install and build from source, you must have [Rust](https://www.rust-lang.org/tools/install) installed. Then, run the following command:

```bash
$ git clone https://github.com/cqb13/latin_words.git
$ cd latin_words
$ cargo install --path .
$ cargo build --release
$ cargo run --release -- [command] [arguments]
```

#### From Binary

To install from a binary, download the latest release from the [releases page](https://github.com/cqb13/latin_words/releases)

```bash
$ cd [download directory]
$ latin_words [command] [arguments]
```

#### Website

You can also use the [website](https://learninglatin.net/translate) to translate text.

### Command Line Arguments

- `--help` or `-h`: Display help information

#### `transEng` Command (Translate English to Latin)

Translate English text to Latin using the following command:

```bash
$ latin_words transEng "English text to translate" [-f]
```

- `"English text to translate"`: The English text you want to translate
- Additional global arguments (see below)

#### `transLat` Command (Translate Latin to English)

Translate Latin text to English using the following command:

```bash
$ latin_words transLat "Latin text to translate" [-t] [-f]
```

- `"Latin text to translate"`: The Latin text you want to translate
- `-t` or `--tricks`: Attempt to use various tricks on words for better results
- `-u` or `--filter-uncommon`: Removes uncommon words
- Additional global arguments (see below)

### Global Arguments

These arguments are applicable to both translation commands (`transEng` and `transLat`):

- `-f` or `--formatted`: Format the output
- `-c` or `--clean`: Remove objects with vague values, such as 'unknown' (requires `-f`)
- `-s` or `--sort`: Sort the output by frequency
- `-p` or `--pretty`: Display a pretty version of the output (requires `-f`)
- `-d` or `--detailed`: Add more information to prettified output (requires `-p`)

### Example Usage

Translate English to Latin with formatted output and cleaning:

```bash
$ latin_words transEng "Hello world" -f -c
```

Translate Latin to English with tricks and detailed, pretty, and formatted output:

```bash
$ latin_words transLat "Salve mundi" -t -d -p -f
```
