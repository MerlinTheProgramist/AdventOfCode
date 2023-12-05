
digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", ]
with open("in.txt") as f:
    ans = 0
    for line in f:
        first = None
        last = None
        for c in line:
            if not c.isnumeric():
                continue
            if not first:
                first = int(c)
            else:
                last = int(c)
        if not last:
            ans += first*11
        else:
            ans += first*10 + last
    print(ans)  
            