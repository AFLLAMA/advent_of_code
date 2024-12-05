from functools import cmp_to_key

def read_input(data_path:str) -> str:
    with open(data_path) as f:
        return f.read()


def custom_sort2(sequence, rules):
    rule_set = set(rules)

    def compare(x, y):
        if f"{x}|{y}" in rule_set:
            return -1
        if f"{y}|{x}" in rule_set:
            return 1
        return 0
    return sorted(sequence, key=cmp_to_key(compare))

if __name__ == '__main__':
    text = read_input('data.txt')
    rules,data = text.split('\n\n')
    rules = rules.split('\n')
    rules_tuples = [r.split('|') for r in rules]
    data = data.split('\n')[:-1]
    print(rules)
    print(data)
    res = 0
    not_valid_seq = []
    for seq in data:
        valid = True
        for r in rules:
            b,a = r.split('|')
            if b in seq and a in seq:
                if seq.split(',').index(b) > seq.split(',').index(a):
                    valid = False
        if valid:
            mid_index = int(len(seq.split(','))/2)
            res += int(seq.split(',')[mid_index])
        if not valid:
            not_valid_seq.append(seq.split(','))
    print(res)
    print(f'{not_valid_seq=}')

    sorted_sequences = [custom_sort2(seq, rules) for seq in not_valid_seq]
    print(sorted_sequences)
    res_corrected = 0
    for sq in sorted_sequences:
        res_corrected += int(sq[int(len(sq)/2)])
    print(res_corrected)
