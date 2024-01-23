import string
import re
from dataclasses import dataclass

DAY = 3

with open(f"./input/{DAY:02}.txt", "r") as f:
    INPUT = f.read().strip()

EXAMPLE_INPUT = """467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"""

SYMBOL = re.sub("(\d|\.|\s)", "", INPUT)
SYMBOL = "".join(set(SYMBOL))

deltas = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]


@dataclass
class Number:
    y: int
    x: int
    len: int
    value: int
    marked: bool = False

    def matches(self, x, y):
        return self.y == y and self.x <= x and x < (self.x + self.len)

    def adjacent(self, x, y):
        for dx, dy in deltas:
            if self.matches(x + dx, y + dy):
                return True
        return False

    def mark(self, x, y):
        if self.adjacent(x, y):
            self.marked = True


def parse_number(line: str):
    # take while digit
    i = 0
    while i < len(line) and line[i].isdigit():
        i += 1
    number_str = line[:i]

    return number_str


def parse_line(line: str, line_idx: int):
    i = 0
    nums = []
    special = []
    while i < len(line):
        c = line[i]
        if c.isdigit():
            numStr = parse_number(line[i:])
            numLen = len(numStr)
            value = int(numStr)
            num = Number(line_idx, i, numLen, value)
            nums.append(num)
            i += numLen
            continue

        # if c is special character
        if c in SYMBOL:
            special.append((line_idx, i, c))

        i += 1

    return nums, special


def parse_input(schematic: str):
    lines = schematic.splitlines()
    nums = []
    symbols = []

    for i, line in enumerate(lines):
        n, s = parse_line(line, i)
        nums.extend(n)
        symbols.extend(s)

    return nums, symbols


def part1():
    nums, symbols = parse_input(INPUT)

    for sy, sx, _ in symbols:
        for num in nums:
            num.mark(sx, sy)

    res = [num.value for num in nums if num.marked]

    print("part1:", sum(res))


def part2():
    nums, symbols = parse_input(INPUT)
    stars = [s for s in symbols if s[2] == "*"]

    res = []
    for sy, sx, _ in stars:
        neighbors = []
        for num in nums:
            if num.adjacent(sx, sy):
                neighbors.append(num)
        if len(neighbors) == 2:
            res.append(neighbors[0].value * neighbors[1].value)

    print("part1:", sum(res))


part1()
part2()
