import sys
from collections import defaultdict

data = sys.stdin.read().strip().splitlines()

elves = set()

for row in range(len(data)):
    for col in range(len(data[row])):
        if data[row][col] == "#":
            elves.add((row, col))

adj_around = [(1, 0), (0, 1), (1, 1), (-1, 0), (0, -1), (-1, -1), (1, -1), (-1, 1)]
adj_north = [(-1, x) for x in [-1, 0, 1]]
adj_south = [(1, x) for x in [-1, 0, 1]]
adj_east = [(y, 1) for y in [-1, 0, 1]]
adj_west = [(y, -1) for y in [-1, 0, 1]]

adj = {"N": adj_north, "S": adj_south, "E": adj_east, "W": adj_west}

delta_moves = {"N": (-1, 0), "S": (1, 0), "E": (0, 1), "W": (0, -1)}

def visualize(elves):
    min_x = min(x for _, x in elves) 
    max_x = max(x for _, x in elves) 
    min_y = min(y for y, _ in elves) 
    max_y = max(y for y, _ in elves) 

    for row in range(min_y - 3, max_y + 3):
        line = ""
        for col in range(min_x - 3, max_x + 3):
            if (row, col) in elves:
                line += "#"
            else:
                line += "."
        print(line)

round = 0 

directions = ["N", "S", "W", "E"]

while True:
    move_counts = defaultdict(lambda: 0)
    proposed_moves = {}

    if round == 10:
        min_x, *_, max_x = sorted(x for _, x in elves) 
        min_y, *_, max_y = sorted(y for y, _ in elves) 
        part1 = (max_x - min_x + 1) * (max_y - min_y + 1) - len(elves)
        print("part1:", part1)


    for row, col in elves:
        # check if there are no elves around
        no_elves_around = not any([(row + y, col + x) in elves for y, x in adj_around])
        if no_elves_around:
            continue


        # check the directions in order 
        for direction in directions:
            considered = not any([(row + y, col + x) in elves for y, x in adj[direction]])
            if considered:
                dy, dx = delta_moves[direction]
                new_pos = (row + dy, col + dx)
                move_counts[new_pos] += 1
                proposed_moves[(row, col)] = new_pos
                break

    moved = 0
    
    for old_pos, new_pos in proposed_moves.items():
        if move_counts[new_pos] > 1:
            continue

        # print("move from:", old_pos, "to", new_pos)
        elves.remove(old_pos)
        elves.add(new_pos)
        moved += 1

    directions.append(directions.pop(0))
    round += 1

    if moved == 0:
        break


# visualize(elves)
print("part2", round)
