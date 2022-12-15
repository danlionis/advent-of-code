import sys

data = sys.stdin.read().strip().splitlines()

sensors = {}
beacons = set()

def manhattan(x1, y1, x2, y2):
    return abs(x1 - x2) + abs(y1 - y2)

for line in data:
    d = line.split()
    sx = int(d[2][2:].strip(","))
    sy = int(d[3][2:].strip(":"))

    bx = int(d[8][2:].strip(","))
    by = int(d[9][2:].strip(":"))

    sensors[(sx, sy)] = manhattan(sx, sy, bx, by)
    beacons.add((bx, by))

print(beacons)
print(sensors)

all_pos = list(sensors.keys()) + list(beacons)

min_x = min([s[0] for s in all_pos])
max_x = max([s[0] for s in all_pos])
print(min_x, max_x)


row = 10
row = 2000000

part1 = 0

for col in range(min_x - 1000000, max_x + 1000000):
    for (sx, sy), dist in sensors.items():
        if manhattan(col, row, sx, sy) <= dist and (col, row) not in beacons and (col, row) not in sensors:
            # print(col, max_x, "#")
            part1 += 1
            break

print(part1)


