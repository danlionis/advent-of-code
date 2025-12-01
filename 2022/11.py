import sys


monkes_data = [
    list(map(str.strip, monke.splitlines())) for monke in sys.stdin.read().split("\n\n")
]


def product(l):
    res = 1
    for li in l:
        res *= li
    return res


def parse_monke(monke_str):
    monke_id = int(monke_str[0][7])
    starting_items = [int(item) for item in monke_str[1][16:].split(",")]
    operation = monke_str[2].split("=")[1].split()
    test = int(monke_str[3].split()[3])
    if_true = int(monke_str[4].split()[5])
    if_false = int(monke_str[5].split()[5])

    return (monke_id, starting_items, operation, test, if_true, if_false)


def apply_operation(old, operation):
    match operation:
        case "old", "+", "old":
            return old + old
        case "old", "*", "old":
            return old * old
        case "old", "+", d:
            return old + int(d)
        case "old", "*", d:
            return old * int(d)

    return 0  # unreachable


def print_monkes(monkes):
    print("monkes:")
    for monke in monkes:
        print(monke[0], monke[1])


monkes = [parse_monke(monke) for monke in monkes_data]
inspections = [0] * len(monkes)

for round in range(20):
    for monke in monkes:
        for i in range(len(monke[1])):
            item = monke[1].pop(0)
            new = apply_operation(item, monke[2]) // 3
            inspections[monke[0]] += 1

            if new % monke[3] == 0:
                if_true = monke[4]
                monkes[if_true][1].append(new)
            else:
                if_false = monke[5]
                monkes[if_false][1].append(new)

print("part1:", product(sorted(inspections)[-2:]))

inspections = [0] * len(monkes)


test = product([monke[3] for monke in monkes])
monkes = [parse_monke(monke) for monke in monkes_data]

for round in range(10000):
    for monke in monkes:
        for i in range(len(monke[1])):
            item = monke[1].pop(0)
            new = apply_operation(item, monke[2]) % test
            inspections[monke[0]] += 1

            throw_monke = monke[4] if new % monke[3] == 0 else monke[5]
            monkes[throw_monke][1].append(new)

print("part2:", product(sorted(inspections)[-2:]))
