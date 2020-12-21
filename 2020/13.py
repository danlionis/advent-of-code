import sys

timestamp = int(sys.stdin.readline().strip())
lines = sys.stdin.readline().strip().split(",")
input1 = list(map(int, filter(lambda l: l != "x", lines)))

minutes = [x - timestamp % x for x in input1]

(minutes, id) = min(zip(minutes, input1), key=lambda x: x[0])
print("part1", minutes * id)

input2 = list(map(lambda x: (int(x[0]), x[1]), filter(
    lambda x: x[0] != "x", zip(lines, range(len(lines))))))

min_id = min(map(lambda x: x[0], input2))

t = 0
found = False

while not found:
    t += min_id
    found = [(t + x[1]) % x[0] == 0 for x in input2]
    found = all(found)
    break

print("part2:", t)
