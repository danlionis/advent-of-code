import sys


def process_line(line):
    parts = line.split()
    valve = parts[1]
    flow_rate = int(parts[4][5:-1])
    tunnels = [tunnel.strip(",") for tunnel in parts[9:]]
    return (valve, {"v": valve, "f": flow_rate, "t": tunnels})


data = dict(map(process_line, sys.stdin.read().strip().splitlines()))
print(data)

print(sorted([(k, v) for k, v in data.items() if v["f"] > 0]))

opened = set()
current_valve = "AA"

# for i in range(30, 0, -1):

#     should_open =
