import sys
import json
import itertools
from functools import cmp_to_key

data = [[json.loads(line) for line in pair.splitlines()]
        for pair in sys.stdin.read().split("\n\n")]


def compare_lists(la, lb):

    for a, b in itertools.zip_longest(la, lb):

        if a == b:
            continue

        if a == None:
            return True  # left side ran out of items

        if b == None:
            return False  # right side ran out of items

        if type(a) is int and type(b) is int:
            if a != b:
                return a < b  # rigt side smaller -> not in right order

        if type(a) is list and type(b) is int:
            return compare_lists(a, [b])

        if type(a) is int and type(b) is list:
            return compare_lists([a], b)

        if type(a) is list and type(b) is list:
            return compare_lists(a, b)

    return True


def cmp_fn(a, b):
    return -1 if compare_lists(a, b) else 1


print("part1:", sum(
    [i + 1 for i, (a, b) in enumerate(data) if compare_lists(a, b)]))

flat_data = [item for sublist in data for item in sublist] + [[[2]], [[6]]]
flat_data = sorted(flat_data, key=cmp_to_key(cmp_fn))

a = flat_data.index([[2]]) + 1
b = flat_data.index([[6]]) + 1
print("part2:",  a * b)
