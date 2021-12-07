import sys
import numpy as np

initial = list(map(int, sys.stdin.readline().split(",")))

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

def simulate(state, days):

    ds = [0] * 9
    for s in state:
        ds[s] += 1

    for i in range(days):
        new = ds[0]
        ds[0:8] = ds[1:9]
        ds[8] = new
        ds[6] += new

    return sum(ds)

print("part1:", simulate(initial, 80))
print("part2:", simulate(initial, 256))
# print("part3:", simulate(initial, 100000))
