import sys
import numpy as np
from functools import reduce

# left shift or
def lsor(a, b):
    return a << 1 | b

nums = np.array(list(map(lambda x: list(map(int, x.strip())), sys.stdin)))
mid = len(nums) / 2

# how many 1s are at each position
counts = list(map(lambda x: np.count_nonzero(x == 1), nums.T))

gamma = reduce(lambda a, b: a << 1 | b, map(lambda x: 1 if x > mid else 0, counts))
epsilon = reduce(lambda a, b: a << 1 | b, map(lambda x: 0 if x > mid else 1, counts))

print("part1:", gamma * epsilon)

sums = [sum(map(lambda y: 1 if y == 0 else -1, x)) for x in nums.T]
sums = [len(nums) - 2 * sum(x) for x in nums.T]
gamma = [int(x >= 0) for x in sums]
epsil = [int(x < 0) for x in sums]
gamma = reduce(lsor, gamma)
epsil = reduce(lsor, epsil)

print("part1:", gamma * epsil)


filtered = nums 
for i in range(len(nums.T)):
    if len(filtered) == 1:
        break

    mid = len(filtered) / 2

    common = 1 if np.count_nonzero(filtered.T[i] == 1) >= mid else 0

    l = filter(lambda x: x[i] == common, filtered)
    filtered = np.array(list(l))

oxygen = reduce(lsor, filtered[0])

filtered = nums 
for i in range(len(nums.T)):
    if len(filtered) == 1:
        break

    mid = len(filtered) / 2

    common = 1 if np.count_nonzero(filtered.T[i] == 1) >= mid else 0
    l = filter(lambda x: x[i] != common, filtered)
    filtered = np.array(list(l))

co2 = reduce(lsor, filtered[0])

print("part2:", co2 * oxygen)
