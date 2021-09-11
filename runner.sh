#!/bin/bash
b=target/release/y20d
s=1
e=25
for ((i = $s ; i <= $e ; i++)); do
  echo "DAY $i"
  ./$b$i
done