sample = open('samples/input.txt')
chars = [set(line[0:int(len(line)/2)]).intersection(line[int(len(line)/2):]).pop()
        for line in sample]

print(sum([ord(ch)-96 if (ord(ch)-94) > 0 else ord(ch)-64+26 for ch in chars]))

