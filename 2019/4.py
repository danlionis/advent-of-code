import itertools
low, high = 234208, 765869
# low, high = 134564, 585159  # fabi

part1 = 0
part2 = 0


def groups(pw):

    if len(pw) == 0:
        return []

    el = pw[0]
    i = 0
    while i < len(pw) and pw[i] == el:
        i += 1

    return [pw[0:i]] + groups(pw[i:])


for pw in range(low, high):

    d = [int(x) for x in str(pw)]
    increasing = all(d[i] <= d[i+1] for i in range(len(d) - 1))
    # groups = [sum(1 for _ in x[1]) for x in itertools.groupby(d)]
    groupLenghts = [len(g) for g in groups(d)]

    if increasing and max(groupLenghts) >= 2:
        part1 += 1
    if increasing and 2 in groupLenghts:
        part2 += 1

print("part1:", part1)
print("part2:", part2)
