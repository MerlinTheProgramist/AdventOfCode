from pandas import *
#import numpy as np

grid = [ [0]*1000 for i in range(1000) ]#np.zeros((10,10))
print(grid)

def rangeR(a,b):
    if(a>b):
        return range(b,a+1)
    else:
        return range(b,a-1,-1)


with open('HydrothermalVenture.txt') as my_file:
    for line in my_file:
        pos1, pos2 = line.strip('\n').split('->')
        pos1 = [int(pos1.split(',')[0]),int(pos1.split(',')[1])]
        pos2 = [int(pos2.split(',')[0]),int(pos2.split(',')[1])]
        
        
        print(line)
        print(pos1)
        print(pos2)
        if(pos1[1]==pos2[1]): # HORIZONTAL Line

            for x in rangeR(pos1[0],pos2[0]):
                grid[x][pos1[1]] +=1


        elif(pos1[0]==pos2[0]): # Vertical
            for y in rangeR(pos1[1],pos2[1]):
                grid[pos1[0]][y] +=1

        else: #Diagnal
            print(pos1)
            print(pos2)
            for x,y in zip(rangeR(pos1[0],pos2[0]), rangeR(pos1[1],pos2[1])):
                grid[x][y] +=1
                print(f'{x}|{y}')
            #input()
output = 0

print(grid)
for x in range(0,len(grid)):
    for y in range(0,len(grid)):
        if(grid[x][y]>=2):
            output+=1

print(output)
#977000