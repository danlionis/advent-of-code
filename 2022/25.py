import sys


digits = {"2": 2, "1": 1, "0": 0, "-": -1, "=": -2, 0: "0", 3: "=", 4: "-", 1: "1", 2: "2"}

def snafu_to_dec(snafu):
    res = 0
    for i in range(len(snafu)):
        pow = 5 ** i
        index = -(i + 1)
        digit = snafu[index]

        res += pow * digits[digit]

    return res

def dec_to_snafu(dec):

    res = "" 
    carry = False
    while dec != 0:
        mod = dec % 5
        if carry:
            mod += 1
        res += digits[mod]
        carry = mod >= 3

        dec //= 5
        
    if carry:
        res += "1"
    return res[::-1]

data = list(map(snafu_to_dec, sys.stdin.read().splitlines()))
print("part1:", dec_to_snafu(sum(data)))

