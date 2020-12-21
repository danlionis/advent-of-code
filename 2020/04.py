import sys
import re

lines = map(" ".join, map(str.splitlines, sys.stdin.read().split("\n\n")))
passports = [dict(e.split(":") for e in l.split(" ")) for l in lines]


def heigt_valid(hgt):
    val = hgt[:-2]
    unit = hgt[-2:]

    return (unit == "cm" and int(val) >= 150 and int(val) <= 193) or (unit == "in" and int(val) >= 59 and int(val) <= 76)



fields = {
    "byr": lambda byr: int(byr) >= 1920 and int(byr) <= 2002,
    "iyr": lambda iyr: int(iyr) >= 2010 and int(iyr) <= 2020,
    "eyr": lambda eyr: int(eyr) >= 2020 and int(eyr) <= 2030,
    "hgt": heigt_valid,
    "hcl": lambda pid: re.match(r"^#[0-9a-f]{6}$", pid) != None,
    "ecl": lambda ecl: ecl in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"],
    "pid": lambda pid: re.match(r"^\d{9}$", pid) != None
}


def has_all_fields(l): return all([f in l for f in fields])


part1 = list(filter(has_all_fields, passports))
print("part1: " + str(len(part1)))

part2 = list(filter(lambda p: all(fields[k](v)
                             for (k, v) in p.items() if k != "cid"), part1))
print("part2: " + str(len(part2)))
