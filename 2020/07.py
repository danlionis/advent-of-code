import sys

lines = map(str.strip, sys.stdin.readlines())


def res_bag(bag):
    sp = bag.split(" ")
    if sp[0] == "no":
        return None

    return (" ".join(sp[1:3]), int(sp[0]))


def process_line(line):
    [bag_in, bags_out] = map(str.strip, line.split("contain"))
    bag_in = " ".join(bag_in.split(" ")[:2])

    bags_out = dict(filter(lambda x: x != None, map(
        res_bag, map(str.strip, bags_out.split(",")))))
    return (bag_in, bags_out)


def parent_bags(bag):
    res = set()
    for (bag_in, bags_out) in bags.items():
        if bag in bags_out:
            res.add(bag_in)
            res = res.union(parent_bags(bag_in))
    return res


def child_bags(bag):
    return sum(v + v * child_bags(k) for (k, v) in bags[bag].items())


bags = dict(map(process_line, lines))

part1 = parent_bags("shiny gold")
print("part1: " + str(len(part1)))

part2 = child_bags("shiny gold")
print("part2: " + str(part2))
