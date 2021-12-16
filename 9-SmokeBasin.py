import math as m

WIDTH = 100
HEIGHT = 100

arr = [0]*HEIGHT

output = 0

with open('smoke.txt','r') as f:
    for i,line in zip(range(HEIGHT),f):
        arr[i]= list(map(lambda x: int(x) ,[c for c in line.strip('\n')]))
print(arr)


"""
TASK 1
"""
# for y in range(100):
#     for x in range(100):
#         out = True
#         curr = arr[y][x]
#         if(y-1<0 or arr[y-1][x]>curr):
#             if(y+1>99 or arr[y+1][x]>curr):
#                 if(x-1<0 or arr[y][x-1]>curr):
#                     if(x+1>99 or arr[y][x+1]>curr):
#                         output +=curr+1

# print(output)

def findBasin(x,y,been=[]):
    
    if arr[x][y]==9 or [x,y] in been: return 0
    print(f"x: {x} y: {y} is {arr[x][y]}")
    been.append([x,y])
    surr = 1
    if( y+1<WIDTH):#  and arr[x][y] == arr[x][y+1]-1):
        surr += findBasin(x,y+1,been)
    if( y-1>=0):#     and arr[x][y] == arr[x][y-1]-1):
        surr += findBasin(x,y-1,been)
    if( x+1<HEIGHT):# and arr[x][y] == arr[x+1][y]-1):
        surr += findBasin(x+1,y,been)
    if( x-1>=0):#     and arr[x][y] == arr[x-1][y]-1):
        surr += findBasin(x-1,y,been)

    return surr



"""
TASK 2
"""
bestout = [0,0,0]

for y in range(HEIGHT):
    for x in range(WIDTH):
        print("::::::::::::::::::")
        bas = findBasin(y,x)
        bestout.sort()
        for i in range(3):
            if(bestout[i]<bas):
                bestout[i]=bas
                break
        print("end   ------")

print(bestout)
print(m.prod(bestout))


