
def binary_to_decimal(binary):
    i,integer = 0,0
    size = len(binary)
    while i < len(binary):
        integer += int(binary[size - 1 - i])*pow(2,i)
        i+=1
    return integer

gammaRate = 0 #last common bit
epsilionRate = 0 # most common bit

file = open('BinaryDiagnostic.txt','r')
content = file.readlines()
file.close()


height = len(content)
width = len(content[0].strip('\n'))

print(width)
numberOfOnes= [0]*width
for line in content:
    i = 0
    for bit in line.strip('\n'):
        numberOfOnes[i] += int(bit)
        i+=1
    
avrage = list(map(lambda x: 1 if x >height/2 else 0, numberOfOnes))
arvegeRev = list(map(lambda x: 1 if x==0 else 0, avrage))

print()
print(binary_to_decimal(avrage))
print(binary_to_decimal(avrage)*binary_to_decimal(arvegeRev))
