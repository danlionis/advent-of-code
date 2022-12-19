import sys


def get_coords(cube):
    x, y, z = map(int, cube.split(","))
    return (x, y, z)


cubes = set([get_coords(cube)
            for cube in sys.stdin.read().strip().splitlines()])

deltas = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]


def solve_part1(cubes):
    res = 0
    for x, y, z in cubes:
        for dx, dy, dz in deltas:
            if (x + dx, y + dy, z + dz) not in cubes:
                res += 1
    return res


part1 = solve_part1(cubes)
print("part1:", solve_part1(cubes))


max_x = max(x for x, _, _ in cubes)
min_x = min(x for x, _, _ in cubes)

max_y = max(y for _, y, _ in cubes)
min_y = min(y for _, y, _ in cubes)

max_z = max(z for _, _, z in cubes)
min_z = min(z for _, _, z in cubes)

# all the positions where there is no cube
negative = set([(x, y, z) for x in range(min_x, max_x + 1) for y in range(min_y, max_y + 1)
               for z in range(min_z, max_z + 1) if (x, y, z) not in cubes])


def remove_rec(x, y, z):
    # key could have been deleted in one of the previous recursion steps
    if (x, y, z) not in negative:
        return
    negative.remove((x, y, z))
    for dx, dy, dz in deltas:
        if (x + dx, y + dy, z + dz) in negative:
            remove_rec(x + dx, y + dy, z + dz)


# set recursion depth to maximum amount of possible cubes
sys.setrecursionlimit(max_x * max_y * max_z)


# remove every cube of air if it is reachable from any cube of air from the outside
for x, y, z in negative.copy():
    if x in [min_x, max_x] or y in [min_y, max_y] or z in [min_z, max_z]:
        remove_rec(x, y, z)


# calculate surface area of the air pocket(s) and substract from total surface area
part2 = part1 - solve_part1(negative)
print("part2:", part2)
