DAY = 5

with open(f"./input/{DAY:02}.txt", "r") as f:
    INPUT = f.read().strip()

EXAMPLE_INPUT = """
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"""


def apply_mapping(mapping, num):
    dst, src, r = mapping
    if num >= src and num < src + r:
        diff = num - src
        return dst + diff

    return num


def find_ranges(range1, range2):
    start1, end1 = range1
    start2, end2 = range2

    # Check for intersection
    if end1 < start2 or end2 < start1:
        return None  # No intersection

    # Calculate intersection
    intersection_start = max(start1, start2)
    intersection_end = min(end1, end2)

    left_start = min(start1, start2)

    return (intersection_start, intersection_end)


def apply_mapping_range(mapping, seeds):
    dst, src, r = mapping
    start_seed, seed_range = seeds
    if num >= src and num < src + r:
        diff = num - src
        return dst + diff
    return num


def parse_single_mapping(line: str):
    dst, src, r = map(int, line.strip().split())
    return dst, src, r


def parse_seeds(line: str):
    seeds = list(map(int, line[7:].strip().split()))
    return seeds


def parse_maps(all: str):
    lines = all.splitlines()
    mappings = [parse_single_mapping(line) for line in lines[1:]]
    return mappings


def parse_input(IN: str):
    parts = IN.strip().split("\n\n")
    seeds = parse_seeds(parts[0])

    maps = [parse_maps(lines) for lines in parts[1:]]
    return seeds, maps


def part1():
    seeds, maps = parse_input(INPUT)

    seed_live = [[seed] for seed in seeds]

    for m in maps:
        for s in seed_live:
            for mapping in m:
                current = s[-1]
                new = apply_mapping(mapping, current)
                if new != current:
                    s.append(new)
                    break

    res = min(s[-1] for s in seed_live)
    print("part1:", res)


def part2():
    seeds, maps = parse_input(INPUT)

    seed_chunks = [tuple(seeds[i : i + 2]) for i in range(0, len(seeds), 2)]
    print(seed_chunks)

    seed_live = [[seed] for seed in seeds]

    # for m in maps:
    #     for s in seed_live:
    #         for mapping in m:
    #             current = s[-1]
    #             new = apply_mapping(mapping, current)
    #             if new != current:
    #                 s.append(new)
    #                 break
    #
    # res = min(s[-1] for s in seed_live)
    # print("part1:", res)


part1()
part2()
