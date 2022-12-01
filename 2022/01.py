import sys

nums = sys.stdin.read()

nums = [sum(map(int, x.strip().split("\n"))) for x in nums.split("\n\n")]


nums = sorted(nums)
print("part1: ", max(nums))

print("part2: ", sum(nums[-3:]))
