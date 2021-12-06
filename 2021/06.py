import sys
from itertools import repeat
from collections import defaultdict
import numpy as np

state_list = list(map(int, sys.stdin.readline().split(",")))

'''
# old solution
def part1(state):
    for i in range(18):
        zeroes = sum(1 for _ in filter(lambda x: x == 0, state))

        state = map(lambda x: 7 if x == 0 else x, state)
        state = list(map(lambda x: x - 1, state))
        state = state + ([8] * zeroes)
    print("part1", len(state))
'''

def simulate(state, rounds):

    ds = defaultdict(lambda: 0)
    for s in state:
        ds[s] += 1

    for i in range(rounds):
        new = ds[0]

        for i in range(8):
            ds[i] = ds[i+1]

        ds[8] = new
        ds[6] += new

    return sum(ds.values())

print("part1:", simulate(state_list, 80))
print("part2:", simulate(state_list, 256))
