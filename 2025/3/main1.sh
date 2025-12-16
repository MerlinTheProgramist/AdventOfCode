#!/bin/bash
ans=0
while read -r line; do
  readarray -t bank < <(fold -w1 <<< "$line")
  len=${#bank[@]}
  tens=$(printf '%s\n' "${bank[@]::len-1}" | awk 'NR==1{m=$1} $1>m{m=$1} END{print m}')

  # get the index of tens
  tens_i=0
  for ((i=0;i<$len;i++)); do
    if [[ $tens -eq "${bank[$i]}" ]]; then
      tens_i=$i
      break
    fi
  done

  ones=$(printf '%s\n' "${bank[@]:tens_i+1}" | awk 'NR==1{m=$1} $1>m{m=$1} END{print m}')
  max="$tens$ones"
  echo "max:" $max
  ((ans+=max))
done
echo "$ans"
