import sys

trees = [[[int(tree), False] for tree in treeline] for treeline in sys.stdin.read().strip().split("\n")]

tree_dict = {}

for treeline in trees:
    last = -1
    for k, (tree, visible) in enumerate(treeline):
        if tree > last:
            treeline[k][1] = True
            last = tree
    last = -1
    for k, (tree, visible) in enumerate(treeline[::-1]):
        if tree > last:
            treeline[-(k + 1)][1] = True
            last = tree

trees = [list(t) for t in zip(*trees)]

for treeline in trees:
    last = -1
    for k, (tree, visible) in enumerate(treeline):
        if tree > last:
            treeline[k][1] = True
            last = tree
    last = -1
    for k, (tree, visible) in enumerate(treeline[::-1]):
        if tree > last:
            treeline[-(k + 1)][1] = True
            last = tree

part1 = [tree for treeline in trees for tree in treeline if tree[1]]

print("part1:", len(part1))



