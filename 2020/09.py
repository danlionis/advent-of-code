import sys
import collections

lines = list(map(int, sys.stdin))

preambleLen = 25

part1 = None
for i in range(preambleLen, len(lines)):
    preamble = lines[i-preambleLen:i]
    el = lines[i]
    if el not in [preamble[x] + preamble[y] for x in range(0, len(preamble)) for y in range(x, len(preamble))]:
        part1 = el
        break


print("part1:", part1)


part2 = collections.deque([])
for i in range(len(lines)):

    while sum(part2) > part1:
        part2.popleft()

    if sum(part2) == part1:
        break

    part2.append(lines[i])


largest, smallest = max(part2), min(part2)
print("part2:", largest + smallest)
