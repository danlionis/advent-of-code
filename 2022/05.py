'''
                    [L]     [H] [W]
                [J] [Z] [J] [Q] [Q]
[S]             [M] [C] [T] [F] [B]
[P]     [H]     [B] [D] [G] [B] [P]
[W]     [L] [D] [D] [J] [W] [T] [C]
[N] [T] [R] [T] [T] [T] [M] [M] [G]
[J] [S] [Q] [S] [Z] [W] [P] [G] [D]
[Z] [G] [V] [V] [Q] [M] [L] [N] [R]
 1   2   3   4   5   6   7   8   9 
'''

import sys
from copy import deepcopy

moves = [x.split() for x in sys.stdin.readlines()]
moves = [list(map(int, [x[1], x[3], x[5]])) for x in moves]

crates = [
    list("ZJNWPS"),
    list("GST"),
    list("VQRLH"),
    list("VSTD"),
    list("QZTDBMJ"),
    list("MWTJDCZL"),
    list("LPMWGTJ"),
    list("NGMTBFQH"),
    list("RDGCPBQW"),
]

part1 = deepcopy(crates)

for m in moves:
    amount, move_from, move_to = m

    for a in range(amount):
        tmp = part1[move_from-1].pop()
        part1[move_to-1].append(tmp)

print("part1:", "".join(list(map(lambda stack: stack[-1], part1))))


part2 = deepcopy(crates)

for m in moves:
    amount, move_from, move_to = m

    part2[move_to-1].extend(part2[move_from-1][-amount:])
    part2[move_from-1] = part2[move_from-1][:-amount]

print("part2:", "".join(list(map(lambda x: x[-1], part2))))