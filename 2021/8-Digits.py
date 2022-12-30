
uniqueLenghts = {'1':2,'7':3,'4':4,'8':7}
          # 0         1     2      3      4       5     6          7     8         9
digits = ['abcdeg','cf','acdeg','abcdf','abcef','bcdef','bcdefg','abd','abcdefg','abcdef']

output = 0

def get_key(val):
    for key, value in uniqueLenghts.items():
         if val == value:
             return key

def get_with_lenght(arr,n):
    return list(filter(lambda x: len(x)==n,arr))

output = 0
with open('Digits.txt') as my_file:
    itr=1
    for line in my_file:
        print(line)
        line = line.strip('\n').split('|')
        usp = list(map(lambda x: "".join(sorted(x)),line[0].split(' ')))[:-1] # Unique Sygnal Patterns
        fduv = list(map(lambda x: "".join(sorted(x)),line[1].split(' ')))[1:] #four digit output value

        presentDig = ['']*10

        for out in usp:
            #print(out)
            if(len(out) in uniqueLenghts.values()):
                presentDig[int(get_key(len(out)))] = ''.join(list(sorted([i for i in out])))
                #print(get_key(len(out)))
        


        presentDig[3] = [x for x in get_with_lenght(usp,5) if(set(presentDig[1]).issubset(set(x)))][0]

        presentDig[9] = ''.join(sorted(list(set(presentDig[4]) | set(presentDig[3]))))
        #usp.remove(list(sorted(presentDig[9])))
        
        top = presentDig[7].replace(presentDig[1], '')
        bottom = list(set(presentDig[9]).difference(set([top])|set(presentDig[4])))[0]
        mid = list(set(presentDig[3])-set(presentDig[1])-set([top,bottom]))[0]

        presentDig[0] = ''.join(list(set(presentDig[8]) - set(mid)))
        
        #print('DEF:',get_with_lenght(usp, 5))
        presentDig[5] = [i for i in set(get_with_lenght(usp, 5))-set([presentDig[3]]) if(len(set(i) - set(presentDig[4]))==2)][0]


        presentDig = list(map(lambda x: ''.join(sorted(x.split())),presentDig))

        left2 = set(usp) - set(presentDig)
        presentDig[2] = ''.join(sorted( get_with_lenght(left2, 5)[0]))
        presentDig[6] = ''.join(sorted( get_with_lenght(left2, 6)[0]))

        #print(presentDig)
        #print('output:', fduv,'\n')
        # print(itr)
        # print(usp)
        # print(presentDig)
        # print(fduv)
        out =''
        for dig in fduv:
            out+=str(presentDig.index(''.join(sorted(dig))))
        # print(out)
        output+=int(out)
        
        itr+=1
print(output)