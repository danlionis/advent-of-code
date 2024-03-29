import sys

nums = list(map(int, sys.stdin))

print("part1:", sum([1 if b > a else 0 for a, b in zip(nums, nums[1:])]))

res = 0
for i in range(1, len(nums)):
    a, b = nums[i - 1], nums[i]
    if b > a:
        res += 1
print("part1:", res)

res = 0
for i in range(3, len(nums)):
    a = sum(nums[i - 3: i])
    b = sum(nums[i - 2: i + 1])
    if b > a:
        res += 1
print("part2:", res)
