import sys

instructions = list(map(lambda l: (l[0], int(l[1:-1])), sys.stdin))


turn_right = {"E": "S", "S": "W", "W": "N", "N": "E"}
turn_left = {"E": "N", "S": "E", "W": "S", "N": "W"}
def rotate_w_right(x, y): return y, -x
def rotate_w_left(x, y): return -y, x


facing = "E"
x1, y1 = 0, 0
x2, y2 = 0, 0
w_x, w_y = 10, 1

for instr in instructions:
    i = instr[0]
    amount = instr[1]

    if i == "E":
        x1 += amount
        w_x += amount
    elif i == "S":
        y1 += -amount
        w_y += -amount
    elif i == "W":
        x1 += -amount
        w_x += -amount
    elif i == "N":
        y1 += amount
        w_y += amount
    elif i == "F":
        x2 += amount * w_x
        y2 += amount * w_y
        if facing == "E":
            x1 += amount
        elif facing == "S":
            y1 += -amount
        elif facing == "W":
            x1 += -amount
        elif facing == "N":
            y1 += amount
    elif i == "R":
        for _ in range(amount // 90):
            facing = turn_right[facing]
            w_x, w_y = rotate_w_right(w_x, w_y)
    elif i == "L":
        for _ in range(amount // 90):
            facing = turn_left[facing]
            w_x, w_y = rotate_w_left(w_x, w_y)

print("part1:", abs(x1) + abs(y1))
print("part2:", abs(x2) + abs(y2))
