"""
Made by MerlinTheProgramist
(github)[https://github.com/MerlinTheProgramist]
"""

horizontal = 0
deph = 0
aim = 0


with open('Dive.txt') as my_file:
    for line in my_file:
        comm = line.split()[0]
        num = int(line.split()[1])
        #print(comm,num)
        if(comm=="down"):
            aim += num
        elif(comm=="up"):
            aim -= num
        elif(comm=="forward"):
            horizontal += num
            deph += num * aim

print("horizontal", horizontal)
print("deph",deph)
print("anwser:",deph*horizontal)

#print(file)