import sys
from collections import defaultdict

lines = sys.stdin.read().strip().split("\n")

directories = defaultdict(lambda: [])

current_dir = []

for l in lines:
    match l.split():
        case "$", "cd", "..": current_dir.pop()
        case "$", "cd", "/": current_dir = ["root"]
        case "$", "cd", d: current_dir.append(d)
        case "$", "ls": pass
        case "dir", _: pass
        case size, name:  directories["/".join(current_dir)].append(int(size))


part1 = defaultdict(lambda: 0)

for file_name, sizes in directories.items():
    parts = file_name.split("/")
    for i in range(len(parts)):
        directory_prefix = "/".join(parts[:i + 1])
        part1[directory_prefix] += sum(sizes)

print("part1:", sum(x for x in part1.values() if x <= 100000))

unused = 70000000 - part1["root"]
needed = 30000000 - unused

print("part2:", min(x for x in part1.values() if x >= needed))
