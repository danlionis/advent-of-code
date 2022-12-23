import sys
import re

board, instr = sys.stdin.read().strip("\n").split("\n\n")
board = board.splitlines()
# print(board[0])
# print(instr)

row, col = 0,  board[0].index(".")
print(col, row)

def turn(direction, turn):
    match turn, direction:
        case "R", (1, 0): return (0, 1)
        case "R", (0, 1): return (-1, 0)
        case "R", (-1, 0): return (0, -1)
        case "R", (0, -1): return (1, 0)
        case "L", (1, 0): return (0, -1)
        case "L", (0, 1): return (1, 0)
        case "L", (-1, 0): return (0, 1)
        case "L", (0, -1): return (-1, 0)
    return dir

def facing(dir):
    match dir:
        case (1, 0): return 0
        case (0, 1): return 1
        case (-1, 0): return 2
        case (0, -1): return 3
    return 0

def vis_dir(dir):
    match dir:
        case (1, 0): return ">" 
        case (0, 1): return "v" 
        case (-1, 0): return "<" 
        case (0, -1): return "^" 
    return "X" 

def board_get(row, col):
    try:
        return board[row % mod_y][col % mod_x] 
    except:
        return " "



dir = (1, 0) # r -> (0, 1) -> r -> (-1, 0) -> r -> (0, -1)
pos = (col, row)

mod_x = max(len(x) for x in board)
mod_y = len(board)

print("mods:", mod_x, mod_y)

moves = list(map(int, re.split(r"[RL]", instr)))

turns = [t for t in instr if t == "R" or t == "L"]

vis = [list(l) for l in board]

for i in range(len(moves)):
    move = moves[i]
    x, y = pos

    for _ in range(move):
        print("dir=", dir, "move=", move, "pos=", x, y)
        x, y = [sum(x) for x in zip(pos, dir)]
        y = y % mod_y
        x = x % mod_x
        print("newpos=", x, y)
        while board_get(y, x) == " ":
            print("wrap", x, y)
            x, y = [sum(x) for x in zip((x, y), dir)]
            y = y % mod_y
            x = x % mod_x
            print("wrap_new", x, y)

            # wrap
        if board_get(y, x) == "#":
            print("rock at:", x, y, "staying at", pos)
            break

        pos = (x, y)
        try:
            vis[y][x] = vis_dir(dir)
        except:
            pass

    try:
        t = turns[i]
        dir = turn(dir, t)
    except:
        break



    print(pos)

part1 = 1000 * (pos[1] + 1) + 4 * (pos[0] + 1) + facing(dir)
print("part1:", part1)
print("\n".join(["".join(l) for l in vis]))
