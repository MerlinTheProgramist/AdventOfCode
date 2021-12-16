file = open('BinaryDiagnostic.txt','r')
content = list(map(lambda x: x.strip('\n'),file.readlines()))
file.close()
height = len(content)
width = len(content[0].strip('\n'))

def bin_to_dec(binary):
    i,integer = 0,0
    size = len(binary)
    while i < len(binary):
        integer += int(binary[size - 1 - i])*pow(2,i)
        i+=1
    return integer           

oxygen = content.copy()
carbon = content.copy()

for i in range(width):
    print('bit:',i)
    print("len:",len(oxygen)/2)
    oxygenbit = 1 if sum([int(ox[i]) for ox in oxygen])>=(len(oxygen)/2) else 0
    #list(map(lambda x: 1 if int(x)>=(len(oxygen)/2) else 0, numberOfOnes(oxygen)))
    print(oxygenbit)
    print(oxygen)
    oxygen[:] = [line for line in oxygen if int(line[i])==int(oxygenbit)]

    if(len(oxygen)==1): break

for i in range(width):
    print('bit:', i)
    print("len:",len(carbon)/2)
    carbonbit = 1 if sum([int(ox[i]) for ox in carbon])<(len(carbon)/2) else 0
    print(carbonbit)
    print(carbon)
    carbon[:] = [line for line in carbon if int(line[i])==int(carbonbit)]

    if(len(carbon)==1): break
print('oxygen rating:',oxygen[0])
print('oxygen rating:',carbon[0])
print(bin_to_dec(oxygen[0])*bin_to_dec(carbon[0]))

# anwser 7440311