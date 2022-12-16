import sys

trees = [[int(tree) for tree in treeline]
         for treeline in sys.stdin.read().strip().split("\n")]

len_y = len(trees)
len_x = len(trees[0])


def get_horizontal_line(trees, y):
    return trees[y]


def get_vertical_line(trees, x):
    return [line[x] for line in trees]


# ------------- PART 1 -----------------


part1 = 0
for r in range(len(trees)):
    for c in range(len(trees[0])):
        tree = trees[r][c]
        vertical = get_vertical_line(trees, c)
        horizontal = get_horizontal_line(trees, r)

        left = all([x < tree for x in horizontal[:c]])
        right = all([x < tree for x in horizontal[c+1:]])
        top = all([x < tree for x in vertical[:r]])
        bottom = all([x < tree for x in vertical[r+1:]])

        if any([left, right, top, bottom]):
            part1 += 1

print("part1", part1)

# ------------- PART 2 -----------------


def distance_visible(self_height, tree_list: list[int]):
    try:
        # return the index of the first tree that has a greater height than we do
        return next(i for i, tree in enumerate(tree_list) if tree >= self_height) + 1
    except:
        # if we can see through to the end return the length of the list
        return len(tree_list)


part2 = [[0] * len_x for _ in range(len_y)]

for r in range(len_y):
    for c in range(len_x):
        tree_left = trees[r][c]

        top = distance_visible(
            tree_left, get_vertical_line(trees, c)[:r][::-1])
        down = distance_visible(tree_left, get_vertical_line(trees, c)[r + 1:])

        right = distance_visible(
            tree_left, get_horizontal_line(trees, r)[c + 1:])
        left = distance_visible(
            tree_left, get_horizontal_line(trees, r)[:c][::-1])

        part2[r][c] = top * down * right * left

part2 = [max(x) for x in part2]
print("part2:", max(part2))
