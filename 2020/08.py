import sys


def parse_instruction(ins):
    [op, v] = ins.split(" ")
    return (op, int(v))


def run_prog(instr):
    executed = set()
    i, acc = 0, 0

    while i < len(instr):
        if i in executed:
            return (False, acc)

        executed.add(i)
        (op, v) = instr[i]

        if op == "acc":
            acc += v
            i += 1
        if op == "jmp":
            i += v
        if op == "nop":
            i += 1

    return (True, acc)


instructions = list(map(lambda l: parse_instruction(l.strip()), sys.stdin))

print("part1:",  run_prog(instructions)[1])

repl = {"jmp": "nop", "nop": "jmp"}
for i in range(len(instructions)):

    (op, v) = instructions[i]
    if op == "nop" or op == "jmp":
        new_instr = instructions.copy()
        new_instr[i] = (repl[op], v)
        (term, acc) = run_prog(new_instr)

        if term:
            print("part2: " + str(acc))
