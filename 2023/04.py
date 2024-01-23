from collections import defaultdict
from functools import cache

# increase max recursion depth
import sys

sys.setrecursionlimit(10**6)

DAY = 4

with open(f"./input/{DAY:02}.txt", "r") as f:
    INPUT = f.read().strip()

EXAMPLE_INPUT = """
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"""


def parse_line(line: str):
    cardStr, nums = line.split(":", 1)
    card = int(cardStr.split()[-1])

    winning_nums, my_nums = nums.split("|")

    winning = [int(n) for n in winning_nums.strip().split()]
    my = [int(n) for n in my_nums.strip().split()]

    return card, winning, my


def points(winning: list, my: list):
    c = common(winning, my)

    if c == 0:
        return 0

    return 1 << (c - 1)


def common(winning: list, my: list):
    i = 0
    for n in my:
        if n in winning:
            i += 1

    return i


def part1():
    lines = INPUT.strip().splitlines()
    cards = [parse_line(line) for line in lines]

    res = sum(points(winning, my) for _, winning, my in cards)
    print("part1:", res)


def part2():
    lines = INPUT.strip().splitlines()
    cards = [parse_line(line) for line in lines]

    # tuple because list does not work with @cache
    cs = tuple([common(winning, my) for _, winning, my in cards])

    res = sum(card_count(cs, i) for i in range(len(cs)))

    print("part2:", res)


@cache
def card_count(cs, card):
    res = 1
    for i in range(1, cs[card] + 1):
        res += card_count(cs, card + i)
    return res


part1()
part2()
