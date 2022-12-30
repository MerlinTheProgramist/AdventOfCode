import numpy as np
from operator import attrgetter
import time as t

import sys

class recursionlimit:
    def __init__(self, limit):
        self.limit = limit

    def __enter__(self):
        self.old_limit = sys.getrecursionlimit()
        sys.setrecursionlimit(self.limit)

    def __exit__(self, type, value, tb):
        sys.setrecursionlimit(self.old_limit)


TILESIZE = 10
TILES = 5#5

SIZE = TILESIZE * TILES




grid = np.zeros((SIZE,SIZE))
grid_path = np.zeros((SIZE,SIZE))

with open('pathFinding.txt','r') as f:
    y=0
    for line in f:
        x=0
        for char in line.strip('\n'):
            grid[y,x] = int(char)
            x+=1
        y+=1

bump = lambda x,y: x+y if x+y<=9 else y+x-9
for yt in range(TILES):
    for xt in range(TILES):
        if(xt!=0 or yt!=0):
            for y in range(TILESIZE):
                for x in range(TILESIZE):
                    grid[y+yt*(TILESIZE),x+xt*(TILESIZE)] = int(bump(grid[y,x] ,yt+xt))

#Draw()

def shortest_to_target(y,x, mem={}):
    if(str([y,x]) in mem): return mem[str([y,x])]

    if(x>=SIZE or y>=SIZE): return float("inf")
    if(x+1==SIZE and y+1==SIZE): return grid[y,x]

    a =[shortest_to_target(y, x+1, mem), shortest_to_target(y+1, x, mem)]
    mem[str([y,x])] =grid[y,x] + min(a)
    if a[0]<a[1]: grid_path[y,x+1]=1 
    else: grid_path[y+1,x]=1 
    return mem[str([y,x])]

with recursionlimit(30000):
    shortest = shortest_to_target(0, 0)


Draw()
print('SUM: ',int(shortest-grid[0,0]))


#441