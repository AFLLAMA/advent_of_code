from pprint import pprint
from typing import List

def read_input(data_path:str) -> str:
    with open(data_path) as f:
        return f.read()

NEXT = {
    'X': 'M',
    'M': 'A',
    'A': 'S',
}
DIRECTION = {
    'lt': (-1, -1),
    't': (-1, 0),
    'tr': (-1, 1),
    'l': (0, -1),
    'r': (0, 1),
    'lb': (1, -1),
    'b': (1, 0),
    'br': (1, 1),
}

def is_in_bounds(x, y) -> bool:
    if x >= 0 and x < X_LEN and y >= 0 and y < Y_LEN:
        return True
    return False


def letter_candidate(
    x: int, y: int,
    data_2d: List[List[str]],
    first: bool = False,
    direction: str|None = None
) -> int:
    if first and data_2d[x][y] != 'X':
        return 0
    if data_2d[x][y] == 'S':
        return 1
    next = NEXT.get(data_2d[x][y])
    if not next:
        return 0

    if first:
        score = 0
        for k, v in DIRECTION.items():
            nx, ny = v
            if is_in_bounds(nx+x, ny+y) and next == data_2d[nx+x][ny+y]:
                score += letter_candidate(nx+x, ny+y, data_2d, False, k)
        direction = None
        return score
    else:
        nx, ny = DIRECTION.get(direction)

        if is_in_bounds(nx+x, ny+y) and next == data_2d[x+nx][y+ny]:
            return letter_candidate(nx+x, ny+y, data_2d, False, direction)
    return 0

def mas(data_lists):
    cnt = 0
    for i, data in enumerate(data_lists):
        for j, d in enumerate(data):
            if d == 'A':
                if i>0 and j>0 and i<X_LEN-1 and j<Y_LEN-1:
                    neibs = [
                        data_lists[i-1][j-1],
                        data_lists[i+1][j-1],
                        data_lists[i-1][j+1],
                        data_lists[i+1][j+1]
                    ]
                    neibs.sort()
                    if neibs == ['M','M','S','S'] and data_lists[i-1][j-1]!=data_lists[i+1][j+1]:
                        cnt += 1
    return cnt

if __name__ == '__main__':
    text = read_input('data.txt')
    print(text)
    data_lists = [list(d) for d in text.split('\n')]
    data_lists = data_lists[:-1]
    Y_LEN = len(data_lists[0])
    X_LEN = len(data_lists)

    processed_grid = [
        [letter_candidate(x,y,data_lists,True) for y,_ in enumerate(data)]
        for x,data in enumerate(data_lists)
    ]
    pprint(processed_grid)
    print('part1')
    print(sum([sum(p) for p in processed_grid]))

    print('part2')
    print(mas(data_lists))
