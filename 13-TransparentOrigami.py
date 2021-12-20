WIDTH = 2000
HEIGHT = 2000
grid = [[0]*WIDTH for _ in range(HEIGHT)]

def DrawBoard():
    print('------------------------------------')
    for row in range(HEIGHT):
        print()
        print(' '.join(list(map(lambda x: '#' if x else '.',grid[row][:WIDTH]))))

with open('folds.txt') as my_file:
    for line in my_file:
        line = line.strip('\n')
        if ',' in line:
            grid[int(line.split(',')[1])][int(line.split(',')[0])] = 1
        elif '=' in line:
            splt = line.split(' ')[-1].split('=')
            if(splt[0]=='x'):
                xplt = int(splt[1])
                for y in range(HEIGHT):
                    for x in range(0, xplt):
                        grid[y][x] = grid[y][x] or grid[y][2*xplt-x]
                        grid[y][2*xplt-x] = 0
                WIDTH = xplt
            elif(splt[0]=='y'):
                yplt = int(splt[1])
                for y in range(0, yplt):
                    for x in range(WIDTH):
                        grid[y][x] = grid[y][x] or grid[2*yplt-y][x]
                        grid[2*yplt-y][x] = 0
                HEIGHT = yplt
                
            
            DrawBoard()
        else:
            DrawBoard()
            
    out = 0
    for row in grid:
        out+=row.count(1)
    print(out) 

