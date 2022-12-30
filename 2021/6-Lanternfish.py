
file = open('Lanternfish.txt','r')
fish = {}
new_fish = {}

for f in file.readlines()[0].strip('\n').split(','):
    if(int(f) in fish):
        fish[int(f)]+=1
    else:
        fish[int(f)] = 1

input(fish)

#input(fish)

#fish=[3,4,3,1,2]

for _ in range(256):

    for key in list(fish):
        if(int(key)==0):
            #input(f'zero {key}')
            if(8 in fish):
                fish[str(8)]+=fish[key]
            else:
                fish[str(8)] = fish[key]

            fish[str(6)] = fish.pop(key)
        else:
            #input(key)
            fish[str(int(key)-1)] = fish.pop(key)
    
    #print(f"After {_+1} day: {fish}")
    print(fish)
total = 0
for f in fish:
    total += fish[f]
print(total)