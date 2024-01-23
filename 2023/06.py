import math

DAY = 6

with open(f"./input/{DAY:02}.txt", "r") as f:
    INPUT = f.read().strip()

EXAMPLE_INPUT = """
Time:      7  15   30
Distance:  9  40  200
"""


def parse_input(text: str):
    lines = text.strip().splitlines()
    time = list(map(int, lines[0].split()[1:]))
    dist = list(map(int, lines[1].split()[1:]))
    return time, dist


def parse_input_kerning(text: str):
    lines = text.strip().splitlines()
    time = "".join(lines[0].split()[1:])
    dist = "".join(lines[1].split()[1:])
    return int(time), int(dist)


def ways_to_win(time, distance):
    res = 0

    for i in range(1, time):
        print(i)
        remaining = time - i
        d = remaining * i

        if d > distance:
            res += 1

    return res


def part1():
    times, ds = parse_input(INPUT)
    res = math.prod([ways_to_win(t, d) for t, d in zip(times, ds)])
    print("part1:", res)


def part2():
    time, dist = parse_input_kerning(INPUT)
    res = ways_to_win(time, dist)
    print("part2:", res)


part1()
part2()
