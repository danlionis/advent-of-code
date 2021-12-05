import re
import sys
import numpy as np

nums = list(map(int, sys.stdin.readline().strip().split(",")))

# boards = list(map(lambda x: list(map(lambda y: list(map(int, re.split(r"\s+", y.strip()))), x.split("\n"))), sys.stdin.read().strip().split("\n\n")))
boards = list(map(lambda x: list(map(int, re.split(r"\s+", x.replace("\n", " ").strip()))), sys.stdin.read().strip().split("\n\n")))

def check_winning_board(board):
    board = np.array(board)
    board = np.array(np.split(board, 5))
    winning = any(all(e == -1 for e in l) for l in board)
    winning |= any(all(e == -1 for e in l) for l in board.T)
    return winning

def remaining(board):
    board = list(filter(lambda x: x != -1, board))
    return board
        
def part1():
    for n in nums:
        for b in boards:
            for i in range(len(b)):
                if b[i] == n:
                    b[i] = -1
        
            if check_winning_board(b):
                print("part1:", n * sum(remaining(b)))
                return

def part2():
    finished = [] 
    for n in nums:
        for b_i in range(len(boards)):
            if b_i in map(lambda x: x[0], finished):
                continue

            b = boards[b_i]
            for i in range(len(b)):
                if b[i] == n:
                    b[i] = -1
        
            if check_winning_board(b):
                finished.append((b_i, n * sum(remaining(b))))

    res = finished[-1][1]
    print("part2:", res)

part1()
part2()
