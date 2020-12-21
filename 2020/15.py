from collections import defaultdict


puzzle_input = [0, 13, 16, 17, 1, 10, 6]
# puzzle_input = [0, 3, 6]

turn = 1

spoken = defaultdict(list) 

last = -1

for p in puzzle_input:
    spoken[p].append(turn)
    last = p
    turn += 1


# while turn != 2021:
while turn != 30000001:
    if turn == 2021:
        print("part1", last)
    if len(spoken[last]) == 1:
        spoken[0].append(turn)
        last = 0
    else:
        diff = spoken[last][-1] - spoken[last][-2]
        spoken[diff] = spoken[diff][-1:] + [turn]
        last = diff
    turn += 1

print("part2", last)
