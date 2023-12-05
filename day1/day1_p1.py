
digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ]
with open("in.txt") as f:
    ans = 0
    for line in f:
        first = None
        last = None
        for i,c in enumerate(line):
            if c.isnumeric():
                dig = int(c)
            else:
                dig = None
                for j,num in enumerate(digits):
                    if i+len(num) < len(line) and line[i:(i+len(num))] == num:
                        dig = j+1
                        break
                if not dig:
                    continue 
            if not first:
                first = dig
            else:
                last = dig
        print(first, last)
        if not last:
            ans += first*11
        else:
            ans += first*10 + last
    print(ans)  
            