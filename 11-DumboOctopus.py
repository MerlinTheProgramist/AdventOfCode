
SIZE = 10
STEPS = 100

arr = [0]*SIZE
blinks = 0

def DrawBoard(grid):
    #print(grid)
    print(" "," * --- "*3, end="*\n")
    for x in range(SIZE):
        print("  | ", end="")
        print('\033[0m', end='')
        print(*list(map(lambda x: '\033[32m'+str(x) if x == 0 else '\033[0m'+str(x), grid[x])), end=" |\n")
        print('\033[0m', end='')
        
    print(" "," * --- "*3, end="*\n")

with open('octopus.txt','r') as f:
    for i,line in zip(range(SIZE),f):
        arr[i]= list(map(lambda x: int(x) ,[c for c in line.strip('\n')]))

def Blink(y,x):
    #print(f'{y},{x} in mem: {[y,x] in mem}')
    
    global blinks
    if(not(0 <= x < SIZE and 0 <= y < SIZE) or arr[y][x] == 0): return
    #print(f'{y},{x}')

    if(arr[y][x]!=10):
        arr[y][x] += 1
        if(arr[y][x]!=10):
            return
    #print(arr[y][x])
    #print(mem)
    # mem.append(f'{y},{x}')
    arr[y][x] = 0
    blinks+=1

    Blink(y-1,x-1)
    Blink(y,x-1)
    Blink(y+1,x-1)

    Blink(y-1,x)
    Blink(y+1,x)

    Blink(y-1,x+1)
    Blink(y,x+1)
    Blink(y+1,x+1)

        
def AllBlink():
    for y in range(SIZE):
        for x in range(SIZE):
            if(arr[y][x]!=0): return False
    return True

ite =0
while True:
    #print(1+ite,'. step')
    # 1.
    for y in range(SIZE):
        for x in range(SIZE):
            arr[y][x] +=1
    
    # 2.
    for y in range(SIZE):
        for x in range(SIZE):
            if(arr[y][x] == 10):
                Blink(y,x)
    DrawBoard(arr)
    if(ite==10):exit()

    if(AllBlink()):
        DrawBoard(arr)
        print(ite+1)
        exit()

    ite+=1
print(blinks)