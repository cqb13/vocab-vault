# Vocab Vault

Vocab Vault is a Rust-based API designed to facilitate translations between English and Latin, utilizing the comprehensive Whitaker's Words Dictionary. Initially inspired by the [Open Words TS](https://github.com/Templar-Development/Open-Words-TS) project, Vocab Vault aims to extend the legacy of William Whitaker's original Words program, which can be explored further at [dsanson/Words](https://github.com/dsanson/Words).

## API Usage Guide

Transitioning from a CLI tool to a more accessible web API, Vocab Vault now offers several HTTP endpoints for users to easily translate texts between English and Latin without the need for local installation, as it operates as a hosted service.

### Website

For an interactive experience, users can translate text using the [Learning Latin website](https://learninglatin.net/translate).

### HTTP Endpoints

Vocab Vault provides two primary endpoints for translations, detailed below:

#### Latin to English Translation

- **Endpoint:** `/latin_to_english`
- **Method:** GET
- **Query Parameters:**
  - `latin_text`: The Latin text you want to translate.
  - `max`: Optional. The maximum number of definitions to return per word.
  - `tricks`: Boolean. Whether to apply various tricks for improved translation accuracy.
  - `sort`: Boolean. Whether to sort the translations (e.g., by frequency).

**Example Request:**

```bash
curl "http://localhost:8080/latin_to_english?latin_text=amor&max=5&tricks=true&sort=true"
```

#### English to Latin Translation

- **Endpoint:** `/english_to_latin`
- **Method:** GET
- **Query Parameters:**
  - `english_text`: The English text you want to translate.
  - `max`: Optional. The maximum number of definitions to return per word.
  - `sort`: Boolean. Whether to sort the translations (e.g., by frequency).

**Example Request:**

```bash
curl "http://localhost:8080/english_to_latin?english_text=love&max=5&sort=true"
```

#### Getting a List of Words

- **Endpoint:** `/get_list`
- **Method:** GET
- **Query Parameters:**
  - `type_of_words`: The category of words to retrieve (e.g., verbs, nouns).
  - `pos_list`: A comma-separated list of parts of speech to filter the words.
  - Other optional parameters include `max`, `min`, `exact`, `amount`, and `random` to refine the search.

**Example Request:**

```bash
curl "http://localhost:8080/get_list?type_of_words=noun&pos_list=nominative,accusative&max=10&random=true"
```

### Running the API

To launch the Vocab Vault API, use the following command:

```bash
cargo run --release
```

After starting, the API will be available at `http://0.0.0.0:8080`, ready to serve translation requests.
