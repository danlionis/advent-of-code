import sys
from copy import deepcopy

crates, moves = sys.stdin.read().split("\n\n")

crates = crates.split("\n")[:-1]
crates = [c[1::4] for c in crates]
# transpose and filter https://scipython.com/book/chapter-4-the-core-python-language-ii/examples/the-matrix-transpose-by-list-comprehension/) 
crates.reverse()
crates = [[row[i] for row in crates if row[i] != " "] for i in range(len(crates) + 1)]  

moves = [list(map(int, x.split()[1::2])) for x in moves.strip().split("\n")]

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
