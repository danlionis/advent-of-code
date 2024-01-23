from collections import Counter
from functools import cmp_to_key

DAY = 7

with open(f"./input/{DAY:02}.txt", "r") as f:
    INPUT = f.read().strip()

EXAMPLE_INPUT = """
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"""

values = {
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
    "T": 10,
    "J": 11,
    "Q": 12,
    "K": 13,
    "A": 14,
}


def compare_hand(a, b, joker: bool):
    vs = values
    if joker:
        vs["J"] = -1
    for i in range(len(a)):
        ca = values[a[i]]
        cb = values[b[i]]
        if ca > cb:
            return 1
        if ca < cb:
            return -1

    return 0


def compare(a, b):
    if a[0] > b[0]:
        return 1
    if a[0] < b[0]:
        return -1
    return compare_hand(a[1], b[1], False)


def compare_joker(a, b):
    if a[0] > b[0]:
        return 1
    if a[0] < b[0]:
        return -1
    return compare_hand(a[1], b[1], True)


def parse_hand(hand: str, joker=False):
    cards = list(hand)

    # count the number of each card
    counts = Counter(cards)
    js = counts["J"]
    if joker and js > 0:
        counts.pop("J")
    common = counts.most_common(2)

    if len(common) == 0:
        return 6

    fst = common[0][1]
    snd = 0
    if len(common) == 2:
        snd = common[1][1]

    if joker:
        fst += js

    if fst == 5:  # five of kind
        return 6

    if fst == 4:  # four of kind
        return 5
    if fst == 3 and snd == 2:  # Full house
        return 4
    if fst == 3:  # three of kind
        return 3
    if fst == 2 and snd == 2:  # two pair
        return 2
    if fst == 2:  # one pair
        return 1
    if fst == 1:  # high card
        return 0

    return 0


def parse_line(line: str, joker=False):
    hand, bid = line.split()

    return parse_hand(hand, joker), hand, int(bid)


def parse_input(text: str, joker: bool):
    lines = text.strip().splitlines()

    return [parse_line(line, joker) for line in lines]


def part1():
    hands = parse_input(INPUT, False)

    sorted_hands = sorted(hands, key=cmp_to_key(compare))

    res = 0
    for i in range(len(sorted_hands)):
        _, _, bid = sorted_hands[i]
        res += bid * (i + 1)

    print("part1:", res)


def part2():
    hands = parse_input(INPUT, True)

    sorted_hands = sorted(hands, key=cmp_to_key(compare_joker))

    res = 0
    for i in range(len(sorted_hands)):
        _, _, bid = sorted_hands[i]
        res += bid * (i + 1)

    print("part2:", res)


part1()
part2()
