import sys

def get_height_value(letter):
    if letter == "S": letter = "a"
    if letter == "E": letter = "z"

    return ord(letter) - ord("a")

data = sys.stdin.read().splitlines()
heightmap = [[get_height_value(letter) for letter in line] for line in data]
# print(heightmap)

start = (0, 0)
end = (0, 0)

for y in range(len(data)):
    for x in range(len(data[0])):
        if data[y][x] == "S":
            start = (x, y)
        if data[y][x] == "E":
            end = (x, y)

steps = [[0] * len(heightmap[0]) for _ in range(len(heightmap))]
# print(steps)

# top, right, bottom, left
deltas = [(0, -1), (1, 0), (0, 1), (-1, 0)]


for y in range(len(heightmap)):
    for x in range(len(heightmap[0])):
        if heightmap[y][x] == 1000:
            start = (x, y)
        if heightmap[y][x] == -1:
            end = (x, y)

def get_adjecent(current_pos):
    cur_x, cur_y = current_pos

    adjecent = []

    for dx, dy in deltas:
        next_x, next_y = cur_x + dx , cur_y + dy

        # skip if out of bounds
        if next_x < 0 or next_x >= len(heightmap[0]) or next_y < 0 or next_y >= len(heightmap):
            continue
        adjecent.append((next_x, next_y))

    return adjecent

queue = []
queue.append(start)
while len(queue) > 0:
    current_pos = queue.pop(0)
    cur_x, cur_y = current_pos
    cur_height = heightmap[cur_y][cur_x]

    if current_pos == end:
        print("part1:", steps[cur_y][cur_x])
        break

    for next_x, next_y in get_adjecent(current_pos):
        next_height = heightmap[next_y][next_x]

        if steps[next_y][next_x] == 0:
            if next_height <= cur_height + 1:
                queue.append((next_x, next_y))
                steps[next_y][next_x] = steps[cur_y][cur_x] + 1


# same thing but backwards, we just look for the first elevation of "a" doing a BFS from the end
steps = [[0] * len(heightmap[0]) for _ in range(len(heightmap))]
queue = []
queue.append(end)
while len(queue) > 0:
    current_pos = queue.pop(0)
    cur_x, cur_y = current_pos
    cur_height = heightmap[cur_y][cur_x]

    if cur_height == 0:
        print("part2:", steps[cur_y][cur_x])
        break

    for next_x, next_y in get_adjecent(current_pos):
        next_height = heightmap[next_y][next_x]

        if steps[next_y][next_x] == 0:
            if next_height >= cur_height - 1:
                queue.append((next_x, next_y))
                steps[next_y][next_x] = steps[cur_y][cur_x] + 1
