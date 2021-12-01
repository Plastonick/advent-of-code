#!/bin/bash

# The file input includes an ordered set of "depths". How many times does the depth increase from one to the next?

awk '{if (prev > 0 &&  $1 > prev) { print $1;  print 1 } else { print $1; print 0}; { prev = $1 }}' input | grep -E '^1$' | wc -l
