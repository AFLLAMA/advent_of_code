def read_input(data_path:str) -> str:
    with open(data_path) as f:
        return f.read()


def count_ways(k, v, current_value, index=1):
    # print(k,v,current_value, index)
    if current_value == k:
        return 1
    if index == len(v):
        return 0

    return (
        count_ways(k, v, current_value + v[index], index + 1) +
            count_ways(k, v, current_value * v[index] if index > 0 else v[index], index + 1)
    )


def count_ways2(k, v, current_value, index=1):
    # print(k,v,current_value, index)
    if current_value == k:
        return 1
    if index == len(v):
        return 0

    return (
        count_ways(k, v, current_value + v[index], index + 1) +
            count_ways(k, v, current_value * v[index] if index > 0 else v[index], index + 1)+
                count_ways(k, v, int(f'{current_value}{v[index]}'), index + 1)
    )


if __name__ == '__main__':
    text = read_input('data.txt')
    text_lines = text.split('\n')[:-1]
    data_dict = {
        int(line.split(':')[0]): list(map(int, line.split(':')[1].split()))
        for line in text_lines
    }
    res = 0
    for k,v in data_dict.items():
        ways = count_ways(k,v,v[0])
        res+=k if ways>0 else 0
    print(f'{res=}')
    res2 = 0
    for k,v in data_dict.items():
        ways = count_ways2(k,v,v[0])
        res+=k if ways>0 else 0
    print(f'{res2=}')
