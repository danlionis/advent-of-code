import sys
from functools import reduce
from collections import defaultdict
import itertools
import re

lines = list(map(str.strip, sys.stdin))


bitmask_bits = {"X": 1, "1": 0, "0": 0}
value_bits = {"X": 0, "1": 1, "0": 0}


def and_mask(m):
    return reduce(lambda acc, x: acc << 1 | bitmask_bits[x], m, 0)


def or_mask(m):
    return reduce(lambda acc, x: acc << 1 | value_bits[x], m, 0)


def parse_mask(m):
    bitmask = and_mask(m)
    value = or_mask(m)
    return bitmask, value


def wildcards(m):
    return reduce(lambda acc, x: acc << 1 | bitmask_bits[x], m, 0)


def decompose(num):
    n = 0
    res = []
    while num > 0:
        x = num % 2
        if x == 1:
            res.append(1 << n)
        num //= 2

        n += 1
    return res


def subset(l):
    res = []

    for i in range(len(l) + 1):
        res = res + list(map(list, itertools.combinations(l, i)))

    return list(map(lambda s: reduce(lambda a, b: a | b, s, 0), res))


mem_regex = re.compile(r"mem\[(?P<mem>\d+)\] = (?P<num>\d+)")

mem = defaultdict()
mask = None

for instr in lines:
    if instr.startswith("mask"):
        mask = parse_mask(instr[7:])
    if instr.startswith("mem"):
        mem_match = mem_regex.match(instr)
        (mem_addr, num) = mem_match.groups()
        mem_addr = int(mem_addr)
        num = int(num)

        tmp = num & mask[0]
        tmp = mask[1] | tmp
        mem[mem_addr] = tmp

print("part1:", sum(mem.values()))

mem = defaultdict()
mask = None

for instr in lines:
    if instr.startswith("mask"):
        mask = instr[7:]
    if instr.startswith("mem"):
        mem_match = mem_regex.match(instr)
        (mem_addr, num) = mem_match.groups()
        mem_addr = int(mem_addr)
        num = int(num)

        mem_addr = mem_addr | or_mask(mask)
        mem_addr = mem_addr & ~and_mask(mask)

        pows = decompose(wildcards(mask))
        subsets = subset(pows)

        for s in subsets:
            mem[mem_addr | s] = num

print("part2:", sum(mem.values()))
