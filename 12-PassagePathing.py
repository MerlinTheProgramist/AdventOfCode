# class Cave():
#     def __init__(self,name,small):
#         self.small = small
#         self.cons = []
#         self.__name__ = ''
#     def addConn(self,conn):
#         self.cons.append(conn)


caves = {}


with open('passage.txt','r') as f:
    for line in f:
        line = line.strip('\n').split('-')
        fr = line[0]
        to = line[1]
        if(fr in caves):
            caves[fr].append(to)
        else:
            caves[fr] = [to]

        if(to in caves):
            caves[to].append(fr)
        else:
            caves[to] = [fr]

print(caves)

def hasDubls(arr):
    arr = [x for x in arr if x.islower()]
    return len(set(arr)) != len(arr) 

def routes(curr,conns=[],mem={}):
    if(curr == 'end'): return 1,[['end']]
    #if(curr in mem): return mem[curr]
    #print(caves[curr])
    tot = 0
    paths = []
    for way in caves[curr]:
        if(way!='start' and(way.isupper() or (not way in conns or not hasDubls(conns)))):
            out = routes(way,conns+[way],mem)
            tot += out[0]
            paths.extend([[curr, *i] for i in out[1]])
    #mem[curr] = tot
    return tot, paths

out = routes('start')
# for route in out[1]:
#     print(','.join(route))
print(out[0])
