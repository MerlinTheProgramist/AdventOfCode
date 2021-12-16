numbers = []

arrays = []

board = [[0]*5]*5

firstBoard = [0,0]

def getCol(matrix, i):
    return [row[i] for row in matrix]

def DrawBoard(grid):
    #print(grid)
    print(" "," * ---- "*4, end="*\n")
    for x in range(5):
        print("  | ", end="")

        print(*list(map(lambda x: str(x)+" "*(5-len(str(x))), grid[x])), end=" |\n")

        if((x+1)%5==0):
            print(" "," * ---- "*4, end="*\n")

def CheckBoard(board):
    global numbers

    for iter in range(len(numbers)): #Iteration of bingo numbers

        for x in range(5):
            for y in range(5):
                
                if (int(board[x][y]) == int(numbers[iter])):
                    #print(board[x][y]==int(numbers[iter]))
                    board[x][y] = True
        
        #print(board)
        #print(numbers[iter])
        #input('')
        if(CheckForBingo(board)):
            DrawBoard(board)
            points = calcPoints(board) * numbers[iter]
            print(points)
            return iter, points

    return 999,0
        
def calcPoints(board):
    boardPoints = 0
    for x in range(5):
        for y in range(5):
            if(not board[x][y] is True):
                boardPoints +=board[x][y]
    
    return boardPoints

def CheckForBingo(board):

    for x in range(5):
            if(board[x]==[True]*5):
                return True
    
            if(getCol(board,x)==[True]*5):
                return True





# Load Data
with open('GiantSquid.txt') as my_file:
    i = -1
    for line in my_file:
        if(i==-1):
            numbers = list(map(lambda x: int(x), line.strip('\n').split(',')))
            i = 0
        elif(len(line) > 1):
            board[i] = list(map(lambda x: int(x), line.strip('\n').split()))
            
            i +=1
            if(i==5):
                print(board)
                check = CheckBoard(board)
                if(firstBoard[0]< check[0]):
                    firstBoard = check
                i = 0
        else:
            i=0

print(firstBoard[1])
    

