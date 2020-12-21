import sys
import re

lines = sys.stdin.read()


sections = lines.split("\n\n")

rules_raw = sections[0].split("\n")
rules_regex = re.compile(r"(.*): (\d+)\-(\d+) or (\d+)\-(\d+)")

rules = dict()


def dbg(x):
    print(x)
    return x


for rule in rules_raw:
    (name, first_low, first_high, second_low,
     second_high) = rules_regex.search(rule).groups()
    l1 = int(first_low)
    h1 = int(first_high)
    l2 = int(second_low)
    h2 = int(second_high)

    # rules[name] = lambda x: (x >= int(dbg(second_low)) and x <= int(second_high))
    rules[name] = lambda x, l1 = l1, h1 = h1, l2 = l2, h2 = h2: (
        x >= l1 and x <= h1) or (x >= l2 and x <= h2)


nearby_tickets = [list(map(int, ticket.split(",")))
                  for ticket in sections[2].split("\n")[1:-1]]

valid_tickets = []
invalid_fields = []
for ticket in nearby_tickets:
    all_valid = True
    for field in ticket:
        tmp = [rule(field) for (name, rule) in rules.items()]
        if not any(tmp):
            invalid_fields.append(field)
            all_valid = False
    if all_valid:
        valid_tickets.append(ticket)

print("part1:", sum(invalid_fields))
print(valid_tickets)
