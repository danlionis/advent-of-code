import sys

data = sys.stdin.read().splitlines();

# data = """R 4
# U 4
# L 3
# D 1
# R 4
# D 1
# L 5
# R 2
# """.splitlines()

moves = [(direction, int(steps))
         for direction, steps in map(str.split, data)]

position_head = (0, 0)
position_tail = (0, 0)

move_deltas = {"U": (0, 1), "R": (1, 0), "D": (0, -1), "L": (-1, 0)}


def sign(x):
    return 1 if x >= 0 else -1


def update_tail(head, tail):
    if head == tail:
        return tail

    tail_x, tail_y = tail
    head_x, head_y = head

    delta_x = head_x - tail_x
    delta_y = head_y - tail_y

    # print(head, tail, delta_x, delta_y, sign(delta_x))

    match (abs(delta_x), abs(delta_y)):
        case 0, 0: return tail
        case 1, 0: return tail
        case 0, 1: return tail
        case 1, 1: return tail
        case 2, 0: return (tail_x + sign(delta_x), tail_y)
        case 0, 2: return (tail_x, tail_y + sign(delta_y))
        case 2, 1: return (tail_x + sign(delta_x), tail_y + sign(delta_y))
        case 1, 2: return (tail_x + sign(delta_x), tail_y + sign(delta_y))

    print("invalid state", head, tail, delta_x, delta_y)

part1 = set()

for direction, amount in moves:
    for i in range(amount):
        ds = move_deltas[direction]
        position_head = (position_head[0] + ds[0], position_head[1] + ds[1])
        position_tail = update_tail(position_head, position_tail)
        part1.add(position_tail)

print(len(part1))


print(update_tail((2, 1), (0, 0)))
