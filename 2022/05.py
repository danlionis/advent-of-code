import sys
from copy import deepcopy

crates, moves = sys.stdin.read().split("\n\n")

crates = crates.split("\n")[:-1]
crates = [c[1::4] for c in crates]
# transpose and filter https://stackoverflow.com/a/6473724/5284327
crates.reverse()
crates = [list(filter(lambda x: x != " ", t)) for t in zip(*crates)]

moves = [list(map(int, x.split()[1::2])) for x in moves.strip().split("\n")]

part1 = deepcopy(crates)
part2 = deepcopy(crates)
x = len(moves)
i = 0
for m in moves:
    i += 1
    amount, move_from, move_to = m

    part1[move_to-1].extend(part1[move_from-1][-amount:][::-1])
    part1[move_from-1] = part1[move_from-1][:-amount]

    part2[move_to-1].extend(part2[move_from-1][-amount:])
    part2[move_from-1] = part2[move_from-1][:-amount]

print("part1:", "".join(list(map(lambda stack: stack[-1], part1))))
print("part2:", "".join(list(map(lambda x: x[-1], part2))))

