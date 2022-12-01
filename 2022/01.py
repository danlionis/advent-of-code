import sys

elves = sys.stdin.read()
elves = [sum(map(int, x.strip().split("\n"))) for x in elves.split("\n\n")]
elves = sorted(elves)

print("part1: ", max(elves))
print("part2: ", sum(elves[-3:]))

