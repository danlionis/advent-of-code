import sys
from collections import defaultdict

lines = sys.stdin.read().strip().split("\n")

directories = defaultdict(lambda: [])
current_dir = []

for l in lines:
    if l.startswith("$ cd"):
        _, dir = l.rsplit(" ", 1)
        if dir == "/":
            current_dir = ["root"]
        elif dir == "..":
            current_dir.pop()
        else:
            current_dir.append(dir)
    elif l.startswith("$ ls"):
        pass
    else:
        size, name = l.split()
        if size != "dir":
            directories["/".join(current_dir)].append((name, int(size)))

part1 = defaultdict(lambda: 0)

for key, v in directories.items():
    s = sum([x[1] for x in v])
    parts = key.split("/")
    for i in range(len(parts)):
        ps = "/".join(parts[:i + 1])
        part1[ps] += s

print("part1", sum([x for x in part1.values() if x <= 100000]))

unused = 70000000 - part1["root"]
needed = 30000000 - unused

res = min(filter(lambda x: x >= needed, part1.values()))
print("part2:", res)
