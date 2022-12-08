import sys
import functools

trees = [[int(tree) for tree in treeline]
         for treeline in sys.stdin.read().strip().split("\n")]

len_y = len(trees)
len_x = len(trees[0])


def get_horizontal_line(trees, y):
    return trees[y]


def get_vertical_line(trees, x):
    return [line[x] for line in trees]


# ------------- PART 1 -----------------

part1 = set()

for y in range(len_y):
    last_left = -1
    last_right = -1

    for x in range(len_x):
        tree_left = trees[y][x]
        if tree_left > last_left:
            part1.add((x, y))
            last_left = tree_left

        tree_right = trees[y][len_x - x - 1]
        if tree_right > last_right:
            part1.add((len_x - x - 1, y))
            last_right = tree_right

for x in range(len_x):
    last_left = -1
    last_right = -1

    for y in range(len_y):
        tree_left = trees[y][x]
        if tree_left > last_left:
            part1.add((x, y))
            last_left = tree_left

        tree_right = trees[-(y + 1)][x]
        if tree_right > last_right:
            part1.add((x, len_y - y - 1))
            last_right = tree_right


print("part1:", len(part1))


# ------------- PART 2 -----------------

def distance_visible(self_height, tree_list: list[int]):
    try:
        # return the index of the first tree that has a greater height than we do
        return next(i for i, tree in enumerate(tree_list) if tree >= self_height) + 1
    except:
        # if we can see through to the end return the length of the list
        return len(tree_list)


part2 = [[0] * len_x for _ in range(len_y)]

for y in range(1, len_y - 1):
    for x in range(1, len_x - 1):
        tree_left = trees[y][x]

        top = distance_visible(
            tree_left, get_vertical_line(trees, x)[:y][::-1])
        down = distance_visible(tree_left, get_vertical_line(trees, x)[y + 1:])

        right = distance_visible(
            tree_left, get_horizontal_line(trees, y)[x + 1:])
        left = distance_visible(
            tree_left, get_horizontal_line(trees, y)[:x][::-1])

        views = [top, down, right, left]
        score = functools.reduce(lambda x, y: x * y, views)
        part2[y][x] = score

part2 = [max(x) for x in part2]
print("part2:", max(part2))
