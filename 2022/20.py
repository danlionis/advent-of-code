import sys

def get_nums(l):
    return [x[1] for x in l]

def sign(x):
    return 1 if x >= 0 else 0 

nums = list(map(int, sys.stdin.read().strip().splitlines()))
nums_indexed = list(enumerate(nums))
mod = len(nums)

for i in range(mod):
    move = nums[i]
    index_in_mix = [x[0] for x in nums_indexed].index(i)
    mix_elem = nums_indexed[index_in_mix]

    nums_indexed.pop(index_in_mix)

    # since we removed one element already we only wrap one less
    index_to_insert = (move + index_in_mix) % (mod - 1)

    nums_indexed.insert(index_to_insert, mix_elem)


index_zero = [x[1] for x in nums_indexed].index(0)

res1 = nums_indexed[(index_zero + 1000) % mod][1]
res2 = nums_indexed[(index_zero + 2000) % mod][1]
res3 = nums_indexed[(index_zero + 3000) % mod][1]

print("part1:", res1 + res2 + res3)

# ---------- PART 2 ----------

nums = [n * 811589153 for n in nums]
nums_indexed = list(enumerate(nums))
mod = len(nums)

for x in range(10):
    for i in range(mod):
        move = nums[i]
        index_in_mix = [x[0] for x in nums_indexed].index(i)
        mix_elem = nums_indexed[index_in_mix]

        nums_indexed.pop(index_in_mix)

        # since we removed one element already we only wrap one less
        index_to_insert = (move + index_in_mix) % (mod - 1)

        nums_indexed.insert(index_to_insert, mix_elem)



index_zero = [x[1] for x in nums_indexed].index(0)

res1 = nums_indexed[(index_zero + 1000) % mod][1]
res2 = nums_indexed[(index_zero + 2000) % mod][1]
res3 = nums_indexed[(index_zero + 3000) % mod][1]

print("part2:", res1 + res2 + res3)
