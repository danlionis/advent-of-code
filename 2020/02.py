import re
import sys

valid1, valid2 = 0, 0
lines = map(str.strip, sys.stdin)
r = re.compile(r"[\-\s:]+")

for line in lines:
    parts = r.split(line)
    a, b, letter, password = int(parts[0]), int(parts[1]), parts[2], parts[3]

    letter_count = sum(1 for x in password if x == letter)

    if letter_count <= b and letter_count >= a:
        valid1 += 1

    if (password[a - 1] == letter) != (password[b - 1] == letter):
        valid2 += 1


print("part1: " + str(valid1))
print("part2: " + str(valid2))
