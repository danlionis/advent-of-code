from sys import stdin
import itertools
from operator import mul
from functools import reduce

adapters = list(sorted(map(int, stdin)))
adapters = [0] + adapters + [adapters[-1] + 3]
diffs = [y - x for (x, y) in zip(adapters, adapters[1:])]

ones = diffs.count(1)
threes = diffs.count(3)

print("part1:", ones * threes)

# all possibilities calculated by hand :)
possibilities = {0: 1, 1: 1, 2: 1, 3: 2, 4: 4, 5: 7}

res = []
x = 1
for i in range(1, len(adapters)):
    if adapters[i] - adapters[i - 1] == 3:
        res.append(x)
        x = 1
    else:
        x += 1
res.append(x)

part2 = reduce(mul, [possibilities[x] for x in res])

print("part2:", part2)
