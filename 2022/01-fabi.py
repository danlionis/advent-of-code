kalorien = open('input/01.txt', 'r')
kalorien = kalorien.read()


numbers = kalorien.split("\n\n")

for i in range(len(numbers)):
    tmp = numbers[i].split()
    tmp = [int(x) for x in tmp]
    numbers[i] = sum(tmp)


print("part1: ", max(numbers))

numbers = sorted(numbers)

print("part2: ", sum(numbers[-3:]))



##Tag 1.1
#x = 0
#y = 0
#for number in numbers:
#    if (number == ""):
#        if(x>y):
#            y = x
#        x = 0
#        continue
#    x += int(number)

#print(y)

##Tag 1.2
#s = [y, 0, 0]
#x = 0
#y = 0
#z = 0
#for a in range(2):
#    for number in numbers:
#        if (number == ""):
#            if(x>y and s[a]>x):
#                y = x
#            x = 0
#            continue
#        x += int(number)
#    x = 0
#    s[a+1] = y
#    y = 0
#    print(s)


#for i in s:
#    z += i
#print(z)

