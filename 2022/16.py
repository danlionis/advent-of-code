import sys

def process_line(line):
    parts = line.split()
    valve = parts[1]
    flow_rate = int(parts[4][5:-1])
    tunnels = [tunnel.strip(",") for tunnel in parts[9:]]
    return (valve, {"v": valve, "f": flow_rate, "t": tunnels})


valves = dict(map(process_line, sys.stdin.read().strip().splitlines()))

max_score = 0

def not_opened(opened):
    return [x for x in valves.keys() if x not in opened]

def best_possible_score(opened, minute):
    vs = not_opened(opened)
    flowrates = sorted([valves[v]["f"] for v in vs], reverse=True)

    return sum([a * b for a, b in zip(flowrates, range(minute, 0, -2))])


def move(valve: str, minute: int, score: int, opened: list):
    # print(31 - minute, valve, opened)
    global max_score
    flow = valves[valve]["f"]
    tunnels = valves[valve]["t"]


    if minute <= 0 or len(opened) == len(valves):
        # print(opened, score, max_score)
        max_score = max(score, max_score)
        return

    if best_possible_score(opened, minute) + score <= max_score:
        # print("not feasable", minute, opened)
        return

    # open valve

    if flow > 0 and valve not in opened:
        new_score = score + (minute - 1) * flow
        new_opened = opened + [valve]
    
        for tunnel in tunnels:
            move(tunnel, minute - 2, new_score, new_opened)

    for tunnel in tunnels:
        move(tunnel, minute - 1, score, opened)

move("AA", 30, 0, [])
print("part1:", max_score)
