import sys

datastream = sys.stdin.read().strip()

def solve(data, n):
    res = 0
    for i in range(len(datastream)):
        s = datastream[i:i+n]

        if len(set(s)) == n:
            res = i + n
            break
    return res

print("part1:", solve(datastream, 4))
print("part2:", solve(datastream, 14))

# oneliners
print("part1:", [len(set(datastream[i:i+4])) for i in range(len(datastream))].index(4) + 4)
print("part2:", [len(set(datastream[i:i+14])) for i in range(len(datastream))].index(14) + 14)
