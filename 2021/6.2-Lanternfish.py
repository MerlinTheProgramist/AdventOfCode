import copy
file = open('Lanternfish.txt','r')
fish = [0]*10


for f in file.readlines()[0].strip('\n').split(','):
    fish[int(f)]+=1


input(fish)

#input(fish)

#fish=[3,4,3,1,2]

for _ in range(256):
    
    
    for key in range(10):
        if(key==0):
            fish[9]+=fish[key]
            fish[7]+=fish[key]
            fish[key] = 0
        else:
            #input(key)
            fish[key-1] =fish[key]
            fish[key] = 0
    
    #print(f"After {_+1} day: {fish}")
    print(fish)
total = 0
for f in fish:
    total += f
print(total)