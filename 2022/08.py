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

part1 = [[False] * len_x for _ in range(len_y)]

def get_visible_mask(tree_list):
    last = -1
    mask = []

    for tree in tree_list:
        mask.append(tree > last)
        last = max(last, tree)

    return mask
            


for y in range(len_y):
    line = get_horizontal_line(trees, y)
    mask_left = get_visible_mask(line)
    mask_right = get_visible_mask(line[::-1])[::-1]
    mask = [a or b for a, b, in zip(mask_left, mask_right)]
    part1[y] = [a or b for a, b in zip(mask_left, part1[y])]
    part1[y] = [a or b for a, b in zip(mask_right, part1[y])]

for x in range(len_x):
    line = get_vertical_line(trees, x)
    mask_left = get_visible_mask(line)
    mask_right = get_visible_mask(line[::-1])[::-1]
    mask = [a or b for a, b, in zip(mask_left, mask_right)]

    for y in range(len_y): 
        part1[y][x] = mask[y] or part1[y][x]

part1 = [visible for line in part1 for visible in line if visible]
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
        tree = trees[y][x]

        top = distance_visible(tree, get_vertical_line(trees, x)[:y][::-1])
        down = distance_visible(tree, get_vertical_line(trees, x)[y + 1:])

        right = distance_visible(tree, get_horizontal_line(trees, y)[x + 1:])
        left = distance_visible(tree, get_horizontal_line(trees, y)[:x][::-1])

        views = [top, down, right, left]
        score = functools.reduce(lambda x, y: x * y, views)
        part2[y][x] = score

part2 = [max(x) for x in part2]
print("part2:", max(part2))
