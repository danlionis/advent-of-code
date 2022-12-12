import sys


def get_height_value(letter):
    if letter == "S":
        letter = "a"
    if letter == "E":
        letter = "z"

    return ord(letter) - ord("a")


# top, right, bottom, left
deltas = [(0, -1), (1, 0), (0, 1), (-1, 0)]

data = sys.stdin.read().splitlines()
heightmap = [[get_height_value(letter) for letter in line] for line in data]

# find start and end locations
start = (0, 0)
end = (0, 0)
for y in range(len(data)):
    for x in range(len(data[0])):
        if data[y][x] == "S":
            start = (x, y)
        if data[y][x] == "E":
            end = (x, y)

visited = [[0] * len(heightmap[0]) for _ in range(len(heightmap))]


def get_neighbors(x, y):

    adjecent = []

    for dx, dy in deltas:
        next_x, next_y = x + dx, y + dy

        # skip if out of bounds
        if next_x < 0 or next_x >= len(heightmap[0]) or next_y < 0 or next_y >= len(heightmap):
            continue
        adjecent.append((next_x, next_y))

    return adjecent


queue = []
queue.append(end)  # go from end to start to make the whole process one-pass

part1, part2 = 0, 0

while len(queue) > 0:
    x, y = queue.pop(0)
    cur_height = heightmap[y][x]

    if (x, y) == start:
        part1 = visited[y][x]

    if cur_height == 0:
        if part2 == 0:  # only set once on first occurence of height "a"
            part2 = visited[y][x]

    for next_x, next_y in get_neighbors(x, y):
        next_height = heightmap[next_y][next_x]

        if visited[next_y][next_x] == 0:  # if not visited
            if next_height >= cur_height - 1:
                queue.append((next_x, next_y))
                visited[next_y][next_x] = visited[y][x] + 1

print("part1:", part1)
print("part2:", part2)
