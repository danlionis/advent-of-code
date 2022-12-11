import sys


ops = sys.stdin.read().splitlines()

acc = 1
accs = [acc]

for op in ops:
    match op.split():
        case ["noop"]:
            accs.append(acc)
        case "addx", num:
            accs.append(acc)
            acc += int(num)
            accs.append(acc)

print("part1:", sum([(a + 1) * b for a, b in list(enumerate(accs))[19::40]])) # 19 because index starts at 1 (19 is 20th element)

crt = [" "] * 40 * 6

for cycle in range(40 * 6):
    sprite_pos = accs[cycle]
    sprite_pos = [sprite_pos - 1, sprite_pos, sprite_pos + 1]

    crt[cycle] = "█" if (cycle % 40) in sprite_pos else " "

print("part2:")
for i in range(6):
    print("".join(crt[i*40:(i+1)*40]))
