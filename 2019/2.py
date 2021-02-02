prog = [1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 13, 1, 19, 1, 19, 10, 23, 2, 10, 23, 27, 1, 27, 6, 31, 1, 13, 31, 35, 1, 13, 35, 39, 1, 39, 10, 43, 2, 43, 13, 47, 1, 47, 9, 51, 2, 51, 13, 55, 1, 5, 55, 59, 2, 59, 9, 63, 1, 13, 63, 67,
        2, 13, 67, 71, 1, 71, 5, 75, 2, 75, 13, 79, 1, 79, 6, 83, 1, 83, 5, 87, 2, 87, 6, 91, 1, 5, 91, 95, 1, 95, 13, 99, 2, 99, 6, 103, 1, 5, 103, 107, 1, 107, 9, 111, 2, 6, 111, 115, 1, 5, 115, 119, 1, 119, 2, 123, 1, 6, 123, 0, 99, 2, 14, 0, 0]


def run(mem):
    i = 0
    while (op := mem[i]) != 99:

        p1 = mem[i+1]
        p2 = mem[i+2]
        p3 = mem[i+3]
        if op == 1:
            mem[p3] = mem[p1] + mem[p2]

        if op == 2:
            mem[p3] = mem[p1] * mem[p2]

        i += 4

    return mem[0]


# part 1
mem1 = prog.copy()
mem1[1] = 12
mem1[2] = 2
print("part1:", run(mem1))


# part 2
for noun in range(0, 100):
    for verb in range(0, 100):
        mem2 = prog.copy()
        mem2[1] = noun
        mem2[2] = verb
        if run(mem2) == 19690720:
            print("part2: noun= {} verb= {} res= {}".format(
                noun, verb, 100 * noun + verb))
