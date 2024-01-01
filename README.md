# Vocab Vault

Vocab Vault is a Rust-based API for translating text between English and Latin, leveraging the Whitaker's Words Dictionary. This project originated as a port of [Open Words TS](https://github.com/Templar-Development/Open-Words-TS), which was originally written in Ada by [dsanson](https://github.com/dsanson). The original Words program by William Whitaker can be found at [dsanson/Words](https://github.com/dsanson/Words).

## API Usage Guide

The Vocab Vault has transitioned from a CLI application to a web API. Below are the updated instructions for utilizing the API.

### Installation

The Vocab Vault API can be accessed through the provided HTTP endpoints. There is no longer a need for local installation as it's hosted as a service.

#### Website

You can use the [website](https://learninglatin.net/translate) to interact with the API and translate text.

### HTTP Endpoints

#### Latin to English Translation

- **Endpoint:** `/latin_to_english`
- **Method:** GET
- **Query Parameters:**
  - `text`: The Latin text you want to translate
  - `max_definitions`: Maximum number of entries to return
  - `use_tricks`: Attempt to use various tricks on words for better results
  - `format_output`: Format the output
  - `clean_output`: Remove objects with vague values, such as 'unknown'
  - `sort_output`: Sort the output by frequency
  - `filter_uncommon_translations`: Removes uncommon words

**Example Request:**

```bash
curl "http://localhost:8080/latin_to_english?text=cur&max_definitions=2&use_tricks=true&format_output=true&clean_output=true&sort_output=true&filter_uncommon_translations=true"
```

#### English to Latin Translation

- **Endpoint:** `/english_to_latin`
- **Method:** GET
- **Query Parameters:**
  - `text`: The English text you want to translate
  - `max_definitions`: Maximum number of entries to return
  - `format_output`: Format the output
  - `clean_output`: Remove objects with vague values, such as 'unknown'
  - `sort_output`: Sort the output by frequency

**Example Request:**

```bash
curl "http://localhost:8080/english_to_latin?text=why&max_definitions=2&format_output=true&clean_output=true&sort_output=true"
```

The API uses two main endpoints, `latin_to_english` and `english_to_latin`, for translation between Latin and English. The responses are serialized in JSON format.

### Running the API

To run the API, execute the following command:

```bash
$ cargo run --release
```

The API will be accessible at `http://0.0.0.0:8080`.
