# Dictionary Data Structure

The dictionary is a collection of JSON files, with json objects in an array.

## English Words

```json
{
  "orth": "word",
  "wid": 0,
  "pos": "part of speech",
  "frequency_type": "very frequent",
  "true_frequency": 0,
  "frequency": 0,
  "compound": 0,
  "semi": 0
}
```

orth: the word in English
wid: the id of corresponding Latin word
pos: part of speech
frequency_type: frequency of the word
true_frequency: frequency + compound - semi
frequency: frequency of the word (A=>70, B=>60, C=>50, D=>40, E=>30, F=>20)
compound: compound word (('very tall' vs. 'tall')) yes(0)/no(10)
semi: part of a meaning set off by semi-colons

## Latin Dictionary

```json
{
  "orth": "latin word",
  "parts": ["first part", "second part", "third part", "fourth part"],
  "senses": ["meaning 1", "meaning 2", "meaning 3"],
  "pos": "part of speech",
  "form": "form of the word",
  "info": {
    "age": "when it was used",
    "area": "the field in which it was used",
    "geo": "the area in which it was used",
    "freq": "frequency of use",
    "source": "source for the word in the dictionary"
  },
  "n": [0, 0],
  "id": 0
}
```
