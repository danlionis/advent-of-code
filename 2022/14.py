import sys

data = [[edge for edge in line.split(" -> ")]
        for line in sys.stdin.read().splitlines()]

rocks = set()

for line in data:
    for a, b in zip(line, line[1:]):
        x1, y1 = map(int, a.split(","))
        x2, y2 = map(int, b.split(","))

        x1, x2 = sorted([x1, x2])
        y1, y2 = sorted([y1, y2])

        for x in range(x1, x2 + 1):
            for y in range(y1, y2 + 1):
                rocks.add((x, y))



max_y = max(map(lambda e: e[1], rocks))

def is_free(x, y):
    if y >= max_y + 2:
        return False

    return (x, y) not in rocks

i = 0

part1 = None
part2 = None

while part1 == None or part2 == None:
    x, y = 500, 0

    while True:
        if y > max_y:
            if part1 == None:
                part1 = i

        if is_free(x, y + 1): # down
            y += 1
        elif is_free(x - 1, y + 1): # diagonal left
            x -= 1
            y += 1
        elif is_free(x + 1, y + 1): # diagonal right
            x += 1
            y += 1
        else: # stuck
            if (x, y) == (500, 0) and part2 == None:
                part2 = i

            rocks.add((x, y))
            break

    i += 1

print("part1:", part1)
print("part2:", part2 + 1) # off by one error
