function main()
    dial = 50
    zeros = 0

    for line in eachline(stdin)
        # PARSING
        if length(line)<2
            break
        end
        dir = line[1]
        num = parse(Int32, line[2:end])
        times = div(num, 100)

        # LOGIC
        if dir == 'L'
            num *= -1
        end

        zeros += 
    end
    println(zeros)
end

main()


