import json
#!!!: senses format issued
#EX: "\"word\"" should be "word"

def format_senses(parts):
    senses = parts.strip()
    parts.lower()
    return senses

def process_data_line(line, id):
    parts = line[:76]
    parts = format_senses(parts)
    parts = parts.split()
    
    orth = parts[0]
    
    pos = line[76:83]
    pos = pos.strip()
    
    if len( line[83:87].strip() ) > 0:
        n = line[83:87].strip().split(" ")
        for n_i, v in enumerate(n):
            try:
                n[n_i] = int(v)
            except ValueError:
                pass
    else:
        n = ['X']
        
    form = line[83:100]
    form = form.strip()
    if len(form) == 0:
        form = "X"

    info = line[100:110]
    info = info.strip()
    info = info.split(" ")
    age = info[0]
    area = info[1]
    geo = info[2]
    freq = info[3]
    source = info[4]
    
    senses = line[110:]
    senses = senses.split(";")
    
    for i, sense in enumerate(senses):
        senses[i] = sense.strip()
        
    if senses[-1] == "":
        senses = senses[:-1]
    elif senses[-1] == " ":
        senses = senses[:-1]
    elif senses[-1] == "\n":
        senses = senses[:-1]
    
    entry = {
        "pos": pos,
        "n": n,
        "parts": parts,
        "senses": senses,
        "form": form,
        "info": {
            "age": age,
            "area": area,
            "geo": geo,
            "freq": freq,
            "source": source
        },
        "orth": orth,
        "id": id
    }
    return entry

def read_data_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
    entries = [process_data_line(line, id+1) for id, line in enumerate(lines)]
    return [entry for entry in entries if entry]

def write_to_json(entries, json_file_path):
    with open(json_file_path, 'w') as json_file:
        json.dump(entries, json_file, separators=(',', ':'))

if __name__ == "__main__":
    input_file_path = "../DICTLINE.GEN"
    output_json_file_path = "new_data/latin_dictionary.json"
    data_entries = read_data_file(input_file_path)
    write_to_json(data_entries, output_json_file_path)
