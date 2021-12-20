STEPS = 40

polymer = ''
rules = {}
count = {}

def tail(n, word):
    while True:                     # Change recursion to a while loop
        if n == 0:
            return word 
        off = 0
        pol = word
        for i in range(len(word)):
            if (word[i:i+2] in rules):
                pol = pol[:i+off]+rules[word[i:i+2]].join(word[i:i+2])+pol[i+2+off:]
                off +=1
        word = pol
        n-=1

with open('polymer.txt') as my_file:
    for line in my_file:
        line = line.strip('\n')
        if('->'in line):
            line = line.split('->')
            rules[line[0].replace(' ', '')] = line[1].replace(' ', '')

        elif(len(line)>0 ):
            polymer = line

    polymer = tail(STEPS,polymer)

    for letter in polymer:
        if letter in count:
            count[letter] +=1
        else:
            count[letter] = 1
    print(max(count.values())-min(count.values()))
