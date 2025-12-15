function main()
    dial = 50
    zeros = 0

    for line in eachline(stdin)
        if length(line)<2
            break
        end
        dir = line[1]
        num = parse(Int32, line[2:end])
        if dir == 'L'
            dial -= num
        else # dir == 'R'
            dial += num
        end     
        dial = mod(dial, 100)

        # println(dial)
        if dial == 0
            zeros += 1
        end
    end
    println(zeros)
end

main()


