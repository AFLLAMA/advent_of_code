lines = [line.strip() for line in open('samples/input.txt')]
games = {l.split(': ')[0].split()[1]: l.split(': ')[1] for l in lines}
bag = {'red':12, 'green':13, 'blue':14}
print(sum([int(k) for k,v in games.items() if all([all([bag[res.split()[1]]>=int(res.split()[0]) for res in results.split(', ')]) for results in v.split('; ')])]))

