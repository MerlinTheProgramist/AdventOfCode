import numpy as np
import time as t

import copy

WIDTH = 10
HEIGHT = 9

def draw_grid(grid):
    print('------------------------------------')
    for row in grid:
        print(' '.join(row))

def get_position(pos,move) -> tuple:
    ret = [pos[0]+move[0],pos[1]+move[1]]

    if(pos[0]+move[0]<0): ret[0] = HEIGHT-1
    elif(pos[0]+move[0]>=HEIGHT): ret[0] = 0
    
    if(pos[1]+move[1]<0): ret[1] = WIDTH-1
    elif(pos[1]+move[1]>=WIDTH): ret[1] = 0

    return tuple(ret)

def result_flow(pos,mem=[]) -> bool:
    if(pos in mem): return False         #flow is loppring so 
    if(grid[pos]=='.'):return True
    
    new_pos = []
    if(grid[pos]=='>'):
        new_pos = get_position(pos,(0,1))
    elif(grid[pos]=='v'):
        new_pos = get_position(pos,(1,0))
    
    mem.append(pos)

    if(result_flow(new_pos,mem)):
        new_grid[new_pos] = grid[pos]
        grid[pos] = '.'
        return True
    
    return False
    

grid = [] #np.full((HEIGHT,WIDTH),'.')
with open('seaCucumbers.txt','r') as f:
    for line in f:
        line = line.strip('\n')
        grid.append([char for char in line])

print(grid)
HEIGHT,WIDTH = len(grid),len(grid[0])
print(HEIGHT, WIDTH)

draw_grid(grid)
print("initial state")

# new grid stores new state of grid
new_grid = grid.copy()

iteration = 1
changes = True

while changes:
    changes = False

    new_grid = copy.deepcopy(grid)

    # fist move east
    for y in range(HEIGHT):
        for x in range(WIDTH):
            
            if(grid[y][x]=='>'):
                pos = get_position((y,x), (0,1))
                if(grid[pos[0]][pos[1]]=='.'):
                    new_grid[pos[0]][pos[1]]='>'
                    new_grid[y][x]='.'
                    changes = True


    grid = copy.deepcopy(new_grid)

    # second move south
    for y in range(HEIGHT):
        for x in range(WIDTH):
            if(new_grid[y][x]=='v'):
                pos = get_position((y,x), (1,0))
                #print(f"pos {[y,x]}\n pos down {pos}")
                if(new_grid[pos[0]][pos[1]]=='.'):
                    grid[pos[0]][pos[1]]='v'
                    grid[y][x]='.'
                    changes = True

    
    # draw_grid(grid)
    # draw_grid(new_grid)
    print('iteration: ',iteration)
    iteration+=1



            # pos = ()
            # if(grid[y,x]=='>'):
            #     pos = get_position((y,x),(0,1))
            # elif(grid[y,x]=='v'):
            #     pos = get_position((y,x),(1,0))
                
            # # if right is empty just move
            # if grid[pos]=='.':
            #     grid[pos] = grid[y,x]
            #     grid[y,x] = '.'
            #     continue
            
            # result_flow(pos)