import json

with open("unique_latin_words.json", "r") as f:
    data = json.load(f)

with open("unique_latin_words.json", "w") as f:
    json.dump(data, f, separators=(',', ':'))
