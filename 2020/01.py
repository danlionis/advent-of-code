import sys

nums = list(map(int, sys.stdin))

res = [x * y for x in nums for y in nums if x + y == 2020]
print("part1:", str(res[0]))


l = len(nums)

for i in range(l):
    for j in range(i, l):
        for k in range(j, l):
            x, y, z = nums[i], nums[j], nums[k]
            if x + y + z == 2020:
                print("part2:", x * y * z)
                break
        else:
            continue
        break
    else:
        continue
    break


# res = [nums[x] * nums[y] * nums[z] for x in range(len(nums)) for y in range(
#    x, len(nums)) for z in range(y, len(nums)) if nums[x] + nums[y] + nums[z] == 2020]
