# Part 1
lines = []

for line in open('samples/input.txt'):
    nums = []
    for i in range(len(line)):
        if line[i].isdigit():
            nums.append(line[i])

    lines.append(int(''.join([nums[0],nums[-1]])))

print(sum(lines))


# Part 2
ptmatch = {'one': '1', 'two': '2', 'three': '3', 'four': '4', 'five': '5', 'six': '6',
           'seven': '7', 'eight': '8', 'nine': '9'}
lines = []

for line in open('samples/input.txt'):
    nums = []
    for i in range(len(line)):
        if line[i].isdigit():
            nums.append(line[i])
        for k,v in ptmatch.items():
            if line[i:i+len(k)] == k:
                nums.append(v)

    lines.append(int(''.join([nums[0],nums[-1]])))

print(sum(lines))

