import sys
import time
from typing import Tuple

data = sys.stdin.read().splitlines()

# data = """R 4
# U 4
# L 3
# D 1
# R 4
# D 1
# L 5
# R 2
# """.splitlines()

moves = [(direction, int(steps))
         for direction, steps in map(str.split, data)]

position_head = (0, 0)
position_tail = (0, 0)

move_deltas = {"U": (0, 1), "R": (1, 0), "D": (0, -1), "L": (-1, 0)}


def visualize(knots):
    time.sleep(1/100)
    max_x = 100
    max_y = 60

    knots = set(knots)

    print(chr(27) + "[2J")

    for y in range(max_y):
        line = ["#" if (x - (max_x // 2), y - (max_y // 2))
                in knots else "." for x in range(max_x)]
        print("".join(line))


def get_step_from_delta(x):
    if x == 0:
        return 0

    return 1 if x > 0 else -1


def sign(x):
    return 1 if x >= 0 else -1


def update_tail(head: Tuple[int, int], tail: Tuple[int, int]):
    if head == tail:
        return tail

    tail_x, tail_y = tail
    head_x, head_y = head

    delta_x = head_x - tail_x
    delta_y = head_y - tail_y

    if max(abs(delta_x), abs(delta_y)) <= 1:
        return tail

    return (tail_x + get_step_from_delta(delta_x), tail_y + get_step_from_delta(delta_y))


def solve(moves, knot_amount=10):
    knots = [(0, 0)] * knot_amount
    part1 = set()
    part2 = set()
    for direction, amount in moves:
        for i in range(amount):
            ds = move_deltas[direction]
            head = knots[0]
            knots[0] = (head[0] + ds[0], head[1] + ds[1])

            for i in range(len(knots) - 1):
                tmp_head = knots[i]
                tmp_tail = knots[i + 1]
                knots[i + 1] = update_tail(tmp_head, tmp_tail)

            # visualize(knots)
            part1.add(knots[1])
            part2.add(knots[-1])
    return len(part1), len(part2)


part1, part2 = solve(moves)
print("part1:", part1)
print("part2:", part2)
