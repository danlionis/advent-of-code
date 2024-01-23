DAY = 2

with open(f"./input/{DAY:02}.txt", "r") as f:
    INPUT = f.read().strip()

EXAMPLE_INPUT = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"""


def parse_round(round: str):
    cubes = [c.strip().split() for c in round.split(",")]

    r, g, b = 0, 0, 0

    for numStr, col in cubes:
        num = int(numStr)
        match col:
            case "red":
                r = num
            case "green":
                g = num
            case "blue":
                b = num

    return [r, g, b]


def parse_game(line: str):
    gameStr, cubes = line.split(":", 1)
    game = int(gameStr.split(" ")[-1])
    rounds = cubes.split(";")
    rounds = [parse_round(r) for r in rounds]

    return game, rounds


def parse_input(i: str):
    lines = i.splitlines()
    games = [parse_game(line) for line in lines]

    return games


def part1():
    games = parse_input(INPUT)

    res = []
    for id, rounds in games:
        possible = True
        for r, g, b in rounds:
            if r > 12 or g > 13 or b > 14:
                possible = False

        if possible:
            res.append(id)

    print("part1:", sum(res))


def part2():
    games = parse_input(INPUT)

    res = 0

    for id, game in games:
        rs = max([g[0] for g in game])
        gs = max([g[1] for g in game])
        bs = max([g[2] for g in game])

        res += rs * gs * bs

    print("part2:", res)


part1()
part2()
