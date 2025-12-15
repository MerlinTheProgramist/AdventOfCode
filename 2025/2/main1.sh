#!/bin/bash
function invalid(){
  len=${#1}
  hlen=$((len / 2))
  # echo "${1:hlen}" "," "${1: $((-hlen))}"
  [[ $((len%2)) -eq 0 && ${1:0:hlen} == "${1:hlen}" ]]
}

ans=0
while read line; do
  for range in  ${line//,/ }; do
    first=${range%-*}    
    last=${range#*-}    
    # echo $first "," $last
    for ((i=first; i<=last; i++)); do
      if invalid "$i"; then
        # echo $i
        (( ans+=i ))
      fi;
    done;
  done;
done; 3<&0 
echo "$ans"

