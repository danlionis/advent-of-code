import sys


def parse_line(line):
    line = line.strip()
    signal_patterns, output_value = line.split(" | ")

    signal_patterns = signal_patterns.split(" ")
    output_value = output_value.split(" ")

    return (signal_patterns, output_value)

lines = list(map(parse_line, sys.stdin))

def part1():
    # segments of values 1, 4, 7, 8
    lens = [2, 4, 3, 7]
    res = 0
    for (sig, out) in lines:
        tmp = list(filter(lambda x: len(x) in lens, out))
        res += len(tmp)
    
    print("part1:", res)


def part2():
    letters = "abcdefg"
    
    for (sig, out) in lines:
    # setup segments
        segments = dict()
        fix = dict()
        
        for x in letters:
            segments[x] = set(letters)
            fix[x] = None

        ones = list(map(set, filter(lambda x: len(x) == 2, sig)))
        print(ones)

        for one in ones:
            segments["a"].intersection_update(one)
            segments["b"].intersection_update(one)
            segments["c"].difference_update(one)
            segments["d"].difference_update(one)
            segments["e"].difference_update(one)
            segments["f"].difference_update(one)
            segments["g"].difference_update(one)

        sevens = list(map(set, filter(lambda x: len(x) == 3, sig)))

        for seven in sevens:
            segments["a"].intersection_update(seven)
            segments["b"].intersection_update(seven)
            segments["c"].difference_update(seven)
            segments["d"].intersection_update(seven)
            segments["e"].difference_update(seven)
            segments["f"].difference_update(seven)
            segments["g"].difference_update(seven)

        fours = list(map(set, filter(lambda x: len(x) == 4, sig)))

        for four in fours:
            segments["a"].intersection_update(four)
            segments["b"].intersection_update(four)
            segments["c"].difference_update(four)
            segments["d"].difference_update(four)
            segments["e"].intersection_update(four)
            segments["f"].intersection_update(four)
            segments["g"].difference_update(four)

        eights = list(map(set, filter(lambda x: len(x) == 7, sig)))

        for eight in eights:
            segments["a"].intersection_update(eight)
            segments["b"].intersection_update(eight)
            segments["c"].intersection_update(eight)
            segments["d"].intersection_update(eight)
            segments["e"].intersection_update(eight)
            segments["f"].intersection_update(eight)
            segments["g"].intersection_update(eight)
        print(segments)


        for (k, v) in segments.items():
            if len(v) == 1:
                fix[k] = list(v)[0]
        print(fix)

part1()
part2()
