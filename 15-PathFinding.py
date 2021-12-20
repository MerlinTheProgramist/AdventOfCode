import numpy as np
from operator import attrgetter
import time as t


TILESIZE = 100
TILES = 5

SIZE = 100#TILESIZE * TILES
class Node():
    parent = None
    neighbors = None
    pathCost = 999
    def __init__(self, val):
        self.val = val
        pathCost = val

    def __str__(self):
        return str(self.val)

    def __int__(self):
        return self.val

    def path(self):
        return self.parent.pathCost + self.val if self.parent else 0
        #return self.parent.path() + self.val if self.parent else 0

def GetNeighbors(x,y):
    out = []
    if x-1 >0:
        out.append(grid[x-1,y])
    if x+1 <SIZE:
        out.append(grid[x+1,y])

    if y-1 >0:
        out.append(grid[x,y-1])
    if y+1 <SIZE:
        out.append(grid[x,y+1])

    return out

def Draw():
    for y in range(SIZE):
        for x in range(SIZE):
            print(grid[y,x], end=' ')
        print()

grid = np.empty((SIZE,SIZE),dtype=Node)

with open('pathFinding.txt','r') as f:
    y=0
    for line in f:
        x=0
        for char in line.strip('\n'):
            grid[y,x] = Node(int(char))
            x+=1
        y+=1
    
    # bump = lambda x: x+1 if x+1<=9 else 1
    # for yt in range(0,SIZE,TILESIZE):
    #     for xt in range(0,SIZE,TILESIZE):
    #         # mini grid
            
            
    #         if(xt!=0 or yt!=0):
    #             if(xt==0):
    #                 for y in range(TILESIZE):
    #                     for x in range(TILESIZE):
    #                         # print(yt+y-TILESIZE,xt+x)
    #                         grid[yt+y,xt+x] = Node(bump(int(grid[yt+y-TILESIZE,xt+x].val)))
    #             else:
    #                 for y in range(TILESIZE):
    #                     for x in range(TILESIZE):
    #                         # print(yt,' ',y,' ',xt+x-TILESIZE)
    #                         grid[yt+y,xt+x] = Node(bump(int(grid[yt+y,xt+x-TILESIZE].val)))
Draw()

for y in range(SIZE):
    for x in range(SIZE):
        grid[y,x].neighbors = GetNeighbors(y,x)


# A* Algorithm

OPEN = list()
CLOSED = list()
OPEN.append(grid[0,0])
target = grid[SIZE-1,SIZE-1]

current = None

while OPEN:
    # st=t.time_ns()
    OPEN.sort(key=lambda x: x.pathCost, reverse=True)
    current = OPEN.pop()
    #print(current)

    CLOSED.append(current)

    if(current==target):
        break
    

    current.pathCost = current.path()
    for neighbor in current.neighbors:
        if(not neighbor in CLOSED and neighbor.pathCost > current.pathCost or neighbor not in OPEN):

                neighbor.pathCost = current.pathCost
                neighbor.parent = current
                if(neighbor not in OPEN):
                    OPEN.append(neighbor)
    # if(t.time_ns()-st>10):
    #     print(t.time_ns()-st)
    #     print(list(map(lambda x: int(x), current.neighbors)))
    #     print(grid.whe(current))
            
outSum = 0 
while True:
    outSum+=current.val
    print(current.val)
    if(current.parent):
        current = current.parent
    else: break

print('SUM: ',outSum-grid[0,0].val)


#441