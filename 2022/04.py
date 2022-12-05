import sys

assignments = [[list(map(int, y.split("-")))
                for y in x.strip().split(",")] for x in sys.stdin.readlines()]


part1 = 0
part2 = 0
for a in assignments:
    range1, range2 = a

    if (range1[0] <= range2[0] and range1[1] >= range2[1]) or (range2[0] <= range1[0] and range2[1] >= range1[1]):
        part1 += 1

    # if (range1[0] >= range2[0] and range1[0] <= range2[1]) or (range1[1] >= range2[0] and range1[1] <= range2[1]) or (range2[0] >= range1[0] and range2[0] <= range1[1]) or (range2[1] >= range1[0] and range2[1] <= range1[1]):
    #     part2 += 1
    # optimization: https://stackoverflow.com/a/325964/5284327
    if range1[1] >= range2[0] and range1[0] <= range2[1]:
        part2 += 1


print("part1:", part1)
print("part2:", part2)
