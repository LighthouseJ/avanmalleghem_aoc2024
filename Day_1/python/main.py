def get_sorted_vectors(file_path):
    col1 = []
    col2 = []
    try:
        with open(file_path) as f:
            for line in f:
                value1, value2 = map(int, line.split())
                col1.append(value1)
                col2.append(value2)
    except ValueError as e:
        raise ValueError(f"Error parsing the file: {e}")
    except FileNotFoundError as e:
        raise FileNotFoundError(f"File not found: {e}")
    
    return sorted(col1), sorted(col2)

def compute_distance(v1, v2):
    return sum(abs(e1 - e2) for e1, e2 in zip(v1, v2))


v1, v2 = get_sorted_vectors("./../input")

distance = compute_distance(v1, v2)

print(f"The answer is : {distance}")