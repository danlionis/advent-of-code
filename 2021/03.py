import sys
import numpy as np
from functools import reduce

nums = np.array(list(map(lambda x: list(map(int, x.strip())), sys.stdin)))
mid = len(nums) / 2

# how many 1s are at each position
counts = list(map(lambda x: np.count_nonzero(x == 1), nums.T))

gamma = reduce(lambda a, b: a << 1 | b, map(lambda x: 1 if x > mid else 0, counts))
epsilon = reduce(lambda a, b: a << 1 | b, map(lambda x: 0 if x > mid else 1, counts))

print("part1:", gamma * epsilon)






# most common bit at each position


filtered = nums 
for i in range(len(nums.T)):
    if len(filtered) == 1:
        break

    mid = len(filtered) / 2

    common = 1 if np.count_nonzero(filtered.T[i] == 1) >= mid else 0

    l = filter(lambda x: x[i] == common, filtered)
    filtered = np.array(list(l))

oxygen = reduce(lambda a, b: a << 1 | b, filtered[0])

filtered = nums 
for i in range(len(nums.T)):
    if len(filtered) == 1:
        break

    mid = len(filtered) / 2

    common = 1 if np.count_nonzero(filtered.T[i] == 1) >= mid else 0
    l = filter(lambda x: x[i] != common, filtered)
    filtered = np.array(list(l))

co2 = reduce(lambda a, b: a << 1 | b, filtered[0])

print("part2:", co2 * oxygen)
