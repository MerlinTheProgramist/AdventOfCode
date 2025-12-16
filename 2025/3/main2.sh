#!/bin/bash
N=12
ans=0
line_n=0
while read -r line; do
  ((line_n+=1))
  readarray -t bank < <(fold -w1 <<< "$line")
  len=${#bank[@]}
  max=""
  for ((n=0;n<$N;n++)); do
    unit=$(printf '%s\n' "${bank[@]::len-N+1+n}" | awk 'NR==1{m=$1} $1>m{m=$1} END{print m}')

    # get the index of tens
    unit_i=0
    for ((i=0;i<$len;i++)); do
      if [[ $unit -eq "${bank[$i]}" ]]; then
        unit_i=$i
        bank[$i]=0
        break
      fi
      bank[$i]=0
    done
    max+=$unit
  done
  echo "$line_n~max:" $max
  ((ans+=max))
done
echo "$ans"
