# Open Words

Latin Words is a port of [Open Words TS](https://github.com/Templar-Development/Open-Words-TS) TypeScript code to Rust for future maintenance and improvement.

Find the original Whitaker's Words written in Ada at https://github.com/dsanson/Words, thoughtfully documented and maintained by [dsanson](https://github.com/dsanson).  More information about William Whitaker and the Words program is available there.  


## CLI

### Command Line Arguments

- `--help` or `-h` - Display help information
- `transEng`
    - "some words" (required)
    - `-f` format output (optional)
- `transLat`
    - "some words" (required)
    - `-f` format output (optional)


## Output

````
>>>> [{'defs': [{'orth': ['sunt'], 'senses': ['to be, exist', 'also used to form verb perfect passive tenses with NOM PERF PPL'], 'infls': [{'form': {'form': 'PRES ACTIVE IND 3 P'}, 'ending': '', 'pos': 'verb'}]}], 'word': 'sunt'}, {'defs': [{'orth': ['gemin', 'gemin'], 'senses': ['twins (pl.)'], 'infls': [{'form': {'number': 'plural', 'declension': 'nominative', 'gender': 'neuter'}, 'ending': 'a', 'pos': 'noun'}, {'form': {'number': 'plural', 'declension': 'vocative', 'gender': 'neuter'}, 'ending': 'a', 'pos': 'noun'}, {'form': {'number': 'plural', 'declension': 'accusative', 'gender': 'neuter'}, 'ending': 'a', 'pos': 'noun'}, {'form': {'voice': 'active', 'person': 2, 'tense': 'present', 'mood': 'imperative', 'number': 'singular'}, 'ending': 'a', 'pos': 'verb'}]}], 'word': 'geminae'}, {'defs': [{'orth': ['somnius', 'somni'], 'senses': ['dream, vision', 'fantasy, day-dream'], 'infls': [{'form': {'number': 'singular', 'declension': 'nominative', 'gender': ''}, 'ending': '', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'vocative', 'gender': ''}, 'ending': '', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'genitive', 'gender': ''}, 'ending': '', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'vocative', 'gender': 'masculine'}, 'ending': '', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'genitive', 'gender': 'masculine'}, 'ending': '', 'pos': 'noun'}]}, {'orth': ['somnos', 'somni'], 'senses': ['sleep'], 'infls': [{'form': {'number': 'singular', 'declension': 'genitive', 'gender': ''}, 'ending': 'i', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'locative', 'gender': ''}, 'ending': 'i', 'pos': 'noun'}, {'form': {'number': 'plural', 'declension': 'nominative', 'gender': 'C'}, 'ending': 'i', 'pos': 'noun'}, {'form': {'number': 'plural', 'declension': 'vocative', 'gender': 'C'}, 'ending': 'i', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'genitive', 'gender': 'neuter'}, 'ending': 'i', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'genitive', 'gender': 'masculine'}, 'ending': 'i', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'genitive', 'gender': 'C'}, 'ending': 'i', 'pos': 'noun'}]}], 'word': 'somni'}, {'defs': [{'orth': ['porta', 'portae'], 'senses': ['gate, entrance', 'city gates', 'door', 'avenue', 'goal (soccer)'], 'infls': [{'form': {'number': 'singular', 'declension': 'genitive', 'gender': 'C'}, 'ending': 'ae', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'locative', 'gender': 'C'}, 'ending': 'ae', 'pos': 'noun'}, {'form': {'number': 'singular', 'declension': 'dative', 'gender': 'C'}, 'ending': 'ae', 'pos': 'noun'}, {'form': {'number': 'plural', 'declension': 'nominative', 'gender': 'C'}, 'ending': 'ae', 'pos': 'noun'}, {'form': {'number': 'plural', 'declension': 'vocative', 'gender': 'C'}, 'ending': 'ae', 'pos': 'noun'}]}], 'word': 'portae'}]

````
