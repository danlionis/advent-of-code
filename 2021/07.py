import sys

nums = sorted(list(map(int, sys.stdin.readline().split(","))))

def part1():
    res = [0] * nums[-1]
    
    for i in range(nums[-1]):
        for b in nums:
            if i == b:
                continue
    
            res[i] += abs(i - b) 
    
    print("part1:", min(res))

def part2():
    res = [0] * nums[-1]
    
    for i in range(nums[-1]):
        for b in nums:
            if i == b:
                continue
    
            n = abs(i - b)
            res[i] += (n**2 + n) // 2 # gauß
    
    print("part2:", min(res))

part1()
part2()

