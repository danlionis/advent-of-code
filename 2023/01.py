import re
from sre_compile import IN

DAY = 1

with open(f"./input/01.txt", "r") as f:
    INPUT = f.read().strip()

EXAMPLE_INPUT = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""

mappings = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    "zero": 0,
}


def part1():
    s = INPUT.splitlines()
    nums = [re.sub("\D", "", line) for line in s]
    # print(nums)
    nums = [10 * int(l[0]) + int(l[-1]) for l in nums]
    print("part1:", sum(nums))


def part2():
    # s = EXAMPLE_INPUT
    s = INPUT

    res = 0

    for line in s.splitlines():
        nums = []
        i = 0
        while i < len(line):
            for k, v in mappings.items():
                if line[i:].startswith(k):
                    nums.append(v)
            # if char at i is a number
            if line[i].isdigit():
                nums.append(int(line[i]))
            i += 1

        res += 10 * nums[0] + nums[-1]

    print("part2:", res)


part1()
part2()
