import sys
import math

winds = [1 if w == ">" else -1 for w in list(sys.stdin.read().strip())]


def get_rock_parts(x, y, shape):
    match shape:
        case "-": return set([(x + dx, y) for dx in range(4)])
        case "+": return set([(x + 1, y), (x, y + 1), (x + 1, y + 1), (x + 2, y + 1), (x + 1, y + 2)])
        case "L": return set([(x + dx, y) for dx in range(3)] + [(x + 2, y + dy) for dy in range(1, 3)])
        case "I": return set([(x, y + dy) for dy in range(4)])
        case "S": return set([(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)])

    return set()


def get_max_y(x, y, shape):
    parts = get_rock_parts(x, y, shape)
    return max(y for _, y in parts)


rock_types = list("-+LIS")


def would_collide_x(pos_x, pos_y, shape):
    parts = get_rock_parts(pos_x, pos_y, shape)
    return any([x < 0 or x >= 7 for x, _ in parts]) or len(grid.intersection(parts)) > 0


def would_collide_y(pos_x, pos_y, shape):
    parts = get_rock_parts(pos_x, pos_y, shape)
    return pos_y < 0 or len(grid.intersection(parts)) > 0


grid = set()
max_y = -1

wind_index = 0

pos = (0, 0)
part1 = 0

# print(len(rock_types), len(winds), math.lcm(len(rock_types), len(winds)))

for rock_num in range(1000000000000):
    # print(rock_num, wind_index, rock_num % len(rock_types), wind_index % len(winds))
    if rock_num == 2022:
        part1 = max_y + 1
        break
    x, y = 2, max_y + 4
    shape = rock_types[rock_num % len(rock_types)]
    # print("new=", shape, "pos=", x, y)

    while True:
        # if rock_num % len(rock_types) == 0 and wind_index % len(winds) == 0:
        #     print(rock_num, wind_index)

        dx = winds[wind_index % len(winds)]
        wind_index += 1
        # print(x, y, dx)
        if not would_collide_x(x + dx, y, shape):
            x += dx

        if would_collide_y(x, y - 1, shape):
            # print("collide", x, y)
            grid = grid.union(get_rock_parts(x, y, shape))
            max_y = max(max_y, get_max_y(x, y, shape))
            break

        y += -1

print("part1:", max_y + 1)
