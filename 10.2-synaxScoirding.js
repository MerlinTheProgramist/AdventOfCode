const { DH_CHECK_P_NOT_SAFE_PRIME } = require('constants')
const fs = require('fs')

const open = {'<':'>','(':')','[':']','{':'}'}
const points = {')':1,']':2,'}':3,'>':4}


function print(str){
    console.log(str)
}

function ifValid(arr){
    var linePoints = 0
    var last = []
    for(const char of arr){
        //print(char)
        //let points = 0

        if(open[char])
            last.push(char)
        else if(open[last[last.length-1]] == char)
            last.pop()
        else 
            return 0
        
    }
    if (last.length == 0) return 0
    //print(last)
    for (const l of last.reverse()) {
            expected  = open[l]
            print(l)
            linePoints *= 5
            linePoints += points[expected]
    }
    print(linePoints)
    return linePoints
}

fs.readFile('synax.txt', (err, data) => {
    let lines =  data.toString().split("\n");
    var scores = []
    for(const line of lines){
        const ar = line.split("");
        var out = ifValid(ar)
        if (out != 0)
            scores.push(out)
    }
    scores = scores.sort(function(a, b){return a - b})
    print(scores)
    print(scores.length)
    print(scores[(scores.length-1)/2])
});

//356874314