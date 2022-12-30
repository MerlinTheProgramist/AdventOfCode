from ast import literal_eval
from functools import cmp_to_key

"""
-1 (LESS)
 0 (EQUAL)
 1 (GREATER)
"""


def item_compare(a, b):
    res = 0
    if type(a) == type(b) == int:
        if(a < b):
            return -1
        elif(a > b):
            return 1
        else:
            return 0
    elif type(a) == type(b) == list:
        n = len(a)
        m = len(b)
        res = 0
        for i in range(min(n, m)):
            res = item_compare(a[i], b[i])
            if res:  # if not eq
                break
        if res == 0:
            if n < m:
                return -1
            elif n > m:
                return 1
            else:
                return 0
    elif type(a) == int:
        res = item_compare([a], b)
    else:
        res = item_compare(a, [b])
    return res


with open("input.txt", 'r') as f:
    left = f.readline()

    to_sort = [[[2]], [[6]]]
    out = 0

    id: int = 1
    while(1):
        if(not left):
            break
        right = literal_eval(f.readline())
        left = literal_eval(left)
        to_sort.append(right)
        to_sort.append(left)
        if(type(left) != list or type(right) != list):
            print("ERROR ERROR ERROR")
            break
        _ = f.readline()

        # print(left)
        # print(right)

        # if(compare(left, right) == -1):
        # out += id
        id += 1
        left = f.readline()

    to_sort = sorted(to_sort, key=cmp_to_key(item_compare))
    print()
    for ar in to_sort:
        print(ar)
    print((to_sort.index([[2]])+1) * (1+to_sort.index([[6]])))
