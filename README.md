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
$ cargo build --release
$ cargo run --release -- [command] [arguments]
# to add to path
$ cargo install --path .
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

### Usage

```sh
  vocab-vault [COMMAND] [OPTIONS]
```

#### Commands

```
  transEng
      Translate english to latin
                                <WORDS>      The words to translate
      -m           --max        <MAX>        The maximum number of translations per definition (default: 6)
      -s           --sort       <>           Sort the output by word frequency
      -p           --pretty     <>           Prints the output in a pretty format
      -d           --detailed   <>           Adds more information to the pretty output
  transLat
      Translate latin to english
                                <WORDS>      The words to translate
      -m           --max        <MAX>        The maximum number of translations per definition (default: 6)
      -s           --sort       <>           Sort the output by word frequency
      -p           --pretty     <>           Prints the output in a pretty format
      -d           --detailed   <>           Adds more information to the pretty output
      -t           --tricks     <>           Will attempt to use various tricks to find the translation
  getList
      Gets a list of words based on the options provided
                                <TYPE>       The type of words to get. Options: english, latin, inflections, not_packons, packons, prefixes, stems, suffixes, tackons, tickons, unique_latin
      -p           --pos        <POS>        The part of speeches to include, separated by commas
      -m           --max        <MAX>        The maximum word length
      -n           --min        <MIN>        The minimum word length
      -e           --exact      <EXACT>      The exact word length
      -a           --amount     <AMOUNT>     The amount of words to get
      -r           --random     <>           Get words from a random position
      -d           --display    <>           Will display as json
      -t           --to         <TO>         The file to export the results to
  help
      Helps you
                                <COMMAND>    A command to help with
  tui
      Starts the tui (.help for info)
```

### Example Usage

Help:

```bash
$ vocab_vault help

$ vocab_vault help transEng
```

Translate English to Latin with 2 options per translation which are sorted by frequency:

```bash
$ vocab_vault transEng "why" -m 2 -s
```

Translate Latin to English with tricks and pretty output:

```bash
$ vocab_vault transLat "cur sum hic" -t -p
```

Get a list of Latin words with a specific part of speech and save it to a file:

```bash
$ vocab_vault getList "latin" -p noun,verb -m 6 -n 3 -t "latin_words.json"
```
