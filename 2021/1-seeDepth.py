"""
Made by MerlinTheProgramist
(github)[https://github.com/MerlinTheProgramist]
"""


increment = 0

with open('seeDepth.txt') as my_file:
    file = list(map(lambda x: int(x), my_file.readlines()))
#print(file)

for i in range(0, len(file)-2):
    print(file[i:i+4])
    window1 = sum(file[i:i+3])
    window2 = sum(file[i+1:i+4])
    if int(window1<window2):
        increment+=1

print(increment)


#1709