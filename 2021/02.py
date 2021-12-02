import sys

instr = list(map(lambda x: x.strip().split(" "), sys.stdin))

p = sum(map(lambda x: int(x[1]), filter(lambda x: x[0] == "forward", instr)))
u = sum(map(lambda x: int(x[1]), filter(lambda x: x[0] == "up", instr)))
d = sum(map(lambda x: int(x[1]), filter(lambda x: x[0] == "down", instr)))

print("part1:", p * (d - u))

depth, p = 0, 0

for i in instr:
    ins = i[0]
    x = int(i[1])
    if ins == "forward":
        p += x
    elif ins == "down":
        depth += x 
    elif ins == "up":
        depth -= x 


print("part1:", depth * p)


depth, p, aim = 0, 0, 0

for i in instr:
    ins = i[0]
    x = int(i[1])
    if ins == "forward":
        p += x
        depth += aim * x
    elif ins == "down":
        aim += x 
    elif ins == "up":
        aim -= x 

print("part2:", depth * p)
