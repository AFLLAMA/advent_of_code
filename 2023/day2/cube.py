lines = [line.strip() for line in open('samples/input.txt')]
games = {l.split(': ')[0].split()[1]: l.split(': ')[1] for l in lines}

# Part1
bag = {'red':12, 'green':13, 'blue':14}
print(sum([int(k) for k,v in games.items() if all([all([bag[res.split()[1]]>=int(res.split()[0]) for res in results.split(', ')]) for results in v.split('; ')])]))

# Part2
import math

obags = []
for k,v in games.items():
    optimal_bag = {'blue': 0, 'red': 0, 'green': 0}
    for game in v.split('; '):
        for se in game.split(', '):
            num,col = se.split()
            if int(num) > optimal_bag[col]:
                optimal_bag[col] = int(num)
    obags.append(optimal_bag.values())

print(sum([math.prod(bag) for bag in obags]))

