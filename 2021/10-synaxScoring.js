const { DH_CHECK_P_NOT_SAFE_PRIME } = require('constants')
const fs = require('fs')

const open = {'<':'>','(':')','[':']','{':'}'}


const points = {')':3,']':57,'}':1197,'>':25137}


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
        else if(open[ last[last.length-1] ] == char)
        {
            last.pop()
        }
        else{//if invalid
            print(last)
            print('Expected '+ open[last[last.length-1]] +' but found '+ char +' instead.')
            print(points[char])
            return points[char]
            last.pop()
        }
    }
    return 0
}

fs.readFile('synax.txt', (err, data) => {
    let lines =  data.toString().split("\n");
    var out = 0
    for(const line of lines){
        const ar = line.split("");
        o
        out += ifValid(ar)
    }
    print('output:'+out)
});

