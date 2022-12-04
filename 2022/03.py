import sys

rucksacks = [x.strip() for x in sys.stdin.readlines()]


def get_score(x):
    res = ord(x)
    if x.isupper():
        res -= 38
    else:
        res -= 96
    return res


def find_common_letter(x):
    [a, b] = x
    for l in a:
        if l in b:
            return get_score(l)
    return 0


part1 = [[s[:len(s) // 2], s[len(s) // 2:]] for s in rucksacks]

part1 = list(map(find_common_letter, part1))

print("part1:", sum(part1))


part2 = 0
for i in range(0, len(rucksacks), 3):
    x, y, z = rucksacks[i], rucksacks[i+1], rucksacks[i+2]
    res = set(x).intersection(set(y)).intersection(set(z))
    res = get_score(list(res)[0])
    part2 += res

print("part2:", part2)


