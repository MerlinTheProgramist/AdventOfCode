from statistics import *
import math as m
file = open('Whales.txt','r')

arr = list(map(lambda x:int(x) ,file.readlines()[0].strip('\n').split(',')))
# print(arr)
# med = median(arr)

# fuel = 0

# for item in arr:
#     fuel += abs( med - item)
# print("first: ",fuel)
#arr = [16,1,2,0,4,2,7,1,2,14]

men = m.floor( mean(arr))
print(mean(arr))
print(men)
fuel = 0
for item in arr:
    if item != men: 
        #input(abs(men - item))
        for i in range(1,abs(men - item)+1):
            #input(i)
            fuel += i
print('second:',fuel)

#89791190
"""
1 1
2 2
3 3
4 4
5 
"""
#1 + 2 + 3 + 4 + 5