#!/bin/bash

awk '{ if ($1 == "forward") { f = f + $2; d = d + (a * $2)}; if ($1 == "up") { a = a - $2 }; if ( $1 == "down" ) { a = a + $2 }; { print f * d } }' input
