
uniqueLenghts = {'1':2,'7':3,'4':4,'8':7}
          # 0         1     2      3      4       5     6          7     8         9
digits = ['abcdeg','cf','acdeg','abcdf','abcef','bcdef','bcdefg','abd','abcdefg','abcdef']

output = 0

def get_key(val):
    for key, value in uniqueLenghts.items():
         if val == value:
             return key


with open('Digits.txt') as my_file:
    for line in my_file:
        line = line.strip('\n').split('|')
        usp = list(map(lambda x: "".join(sorted(x)),line[0].split(' '))) # Unique Sygnal Patterns
        fduv = list(map(lambda x: "".join(sorted(x)),line[1].split(' ')))[1:] #four digit output value

        presentDig = ['']*10

        for out in usp:
            #print(out)
            if(len(out) in uniqueLenghts.values()):
                presentDig[int(get_key(len(out)))] = out
                #print(get_key(len(out)))
        print(presentDig)
        print('output:', fduv,'\n')


        top = presentDig[7].replace(presentDig[1], '')
        mid = 

        presentDig[3] = 
                

#print(output)