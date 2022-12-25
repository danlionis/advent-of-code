import sys
import z3

def parse_monke(line):
    monke = line[:4]
    operation = line[6:]
    try:
        operation = int(operation)
    except:
        operation = operation.split()
    return (monke, operation)

ops = {"+": int.__add__, "-": int.__sub__, "*": int.__mul__, "/": int.__truediv__}

monkes_original = dict(map(parse_monke, sys.stdin.read().splitlines()))
monkes = monkes_original.copy()

def resolve_monke(monke):
    operation = monkes[monke]
    
    if type(operation) is int:
        return operation

    a, op, b = operation
    a = resolve_monke(a)
    b = resolve_monke(b)

    res = ops[op](a, b)


    return int(res)

print("part1:", resolve_monke("root"))


solver = z3.Solver()

# Real is important here!
vars = dict((name, z3.Real(name)) for name in monkes_original.keys())

ops = {"+": z3.ArithRef.__add__, "-": z3.ArithRef.__sub__, "*": z3.ArithRef.__mul__, "/": z3.ArithRef.__truediv__}


for name_str, operation in monkes_original.items():
    name = vars[name_str]

    if name_str == "humn":
        continue

    if name_str == "root":
        solver.add(vars[operation[0]] == vars[operation[2]])
        continue

    if type(operation) is int:
        solver.add(name == operation)
        continue

    rhs, op, lhs = operation

    rhs = vars[rhs]
    lhs = vars[lhs]

    solver.add(name == ops[op](rhs, lhs))

sat = solver.check()
# print(sat)
part2 = solver.model().eval(vars["humn"])
print("part2:", part2)
