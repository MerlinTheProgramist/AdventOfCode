STEPS = 10

polymer = ''

rules = {}

count = {}

with open('polymer.txt') as my_file:
    for line in my_file:
        line = line.strip('\n')
        if('->'in line):
            line = line.split('->')
            rules[line[0].replace(' ', '')] = line[1].replace(' ', '')

        elif(len(line)>0 ):
            polymer = line

    print(rules)
    for _ in range(STEPS):
        off = 0
        pol = polymer
        for i in range(len(polymer)):
            if (polymer[i:i+2] in rules):
                pol = pol[:i+off]+rules[polymer[i:i+2]].join(polymer[i:i+2])+pol[i+2+off:]
                off +=1
        polymer = pol

    # Counting
    for letter in polymer:
        if letter in count:
            count[letter] +=1
        else:
            count[letter] = 1
    print(max(count.values())-min(count.values()))

