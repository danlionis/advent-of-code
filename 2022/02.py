import sys


# A, X: Rock
# B, Y: Paper
# C, Z: Scissors

points = {"X": 1, "Y": 2, "Z": 3}
draw_hand = {"A": "X", "B": "Y", "C": "Z"}
winning_hand = {"A": "Y", "B": "Z", "C": "X"}
loosing_hand = {"A": "Z", "B": "X", "C": "Y"}

winning = ["AY", "BZ", "CX"]
draws = ["AX", "BY", "CZ"]

def play(game):

    enemy, me = game
    res = points[me]

    if (enemy + me) in draws:
        res += 3

    if (enemy + me) in winning:
        res += 6

    return res

def play_to_win(game):
    enemy, end = game
    me = ""

    if end == "X":
        me = loosing_hand[enemy]
    elif end == "Y":
        me = draw_hand[enemy] 
    elif end == "Z":
        me = winning_hand[enemy]

    return play([enemy, me])

games = [game.strip().split() for game in sys.stdin.readlines()]

print("part1:", sum(map(play,games)))
print("part2:", sum(map(play_to_win,games)))
