import sys
from functools import reduce

questions = list(map(str.splitlines, sys.stdin.read().split("\n\n")))


def intersection(a, b): return [x for x in a if x in b]


questions_anyone = [len(set("".join(q))) for q in questions]
questions_everyone = [len(reduce(intersection, q)) for q in questions]

print("part1: " + str(sum(questions_anyone)))
print("part2: " + str(sum(questions_everyone)))
