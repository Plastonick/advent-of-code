#!/bin/bash

# The file input includes an ordered set of "depths". 
# This time, we want to consider the rolling sum of the three consecutive depths, how often does this rolling sum increase from the last? 

awk '{if (a > 0 && b > 0) { print a + b +$1 }; { b = a; a = $1 }}' input | awk '{if (prev > 0 && $1 > prev) { print 1 }; { prev = $1 }}' | wc -l
