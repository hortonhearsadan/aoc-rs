#!/bin/bash
b=target/release/y20d

for ((i = 1 ; i <= 25 ; i++)); do
  echo "DAY $i"
  ./$b$i
done