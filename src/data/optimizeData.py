import json

with open("latin_addons.json", "r") as f:
    data = json.load(f)
    print(data["tickons"])

#with open("latin_addons.json", "w") as f:
#    json.dump(data, f, separators=(',', ':'))
