import json

def process_data_line(line1, line2, line3):
    orth = line1.strip()
    
    pos = line2[:6]
    pos = pos.strip()
    
    n = line2[6:10]
    n = n.strip()
    n = n.split(" ")
    n = [int(i) for i in n]

    form = line2[6:52]
    form = " ".join(form.split())

    info = line2[52:]
    info = info.split(" ")
    info = [i.strip() for i in info]
    info = [i for i in info if i]
    
    age = info[0]
    area = info[1]
    geo = info[2]
    freq = info[3]
    source = info[4]

    senses = line3.strip()
    senses = senses.split(";")
    senses = [sense.strip() for sense in senses]

    if not senses[-1]:
        senses = senses[:-1]

    entry = {
        "orth": orth,
        "senses": senses,
        "pos": pos,
        "form": form,
        "n": n,
        "info": {
            "age": age,
            "area": area,
            "geo": geo,
            "freq": freq,
            "source": source
        },
    }
    return entry

def read_data_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
    entries = []
    for i in range(0, len(lines), 3):
        entries += [process_data_line(lines[i], lines[i+1], lines[i+2])]
    
    return [entry for entry in entries if entry]

def write_to_json(entries, json_file_path):
    with open(json_file_path, 'w') as json_file:
        json.dump(entries, json_file, separators=(',', ':'))

if __name__ == "__main__":
    input_file_path = "../UNIQUES.LAT"
    output_json_file_path = "new_data/unique_latin_words.json"
    data_entries = read_data_file(input_file_path)
    write_to_json(data_entries, output_json_file_path)
