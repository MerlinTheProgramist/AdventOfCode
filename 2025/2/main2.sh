#!/bin/bash
function invalid(){
  len=${#1}
  for (( l=len/2 ; l>= 1 ; l--)); do
    if [[ $((len%l)) -ne 0 ]]; then
      continue
    fi
    sub=${1:0:l}
    for (( j=l; j<len; j+=l)); do
      # echo $j":" ${1:j:l}
      if [[ $sub != ${1:j:l} ]]; then
        continue 2
      fi
    done;
    return 0
  done;
  return 1
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

