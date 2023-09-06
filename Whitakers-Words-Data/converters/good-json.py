import json


if __name__ == "__main__":
    input_file_path = "../../src/data/unique_latin_words.json"
    output_json_file_path = "../../src/data/unique_latin_words.json"
    
    with open(input_file_path, "r") as input_file:
        data = json.load(input_file)
        
    with open(output_json_file_path, "w") as output_file:
        json.dump(data, output_file, separators=(',', ':'))
    