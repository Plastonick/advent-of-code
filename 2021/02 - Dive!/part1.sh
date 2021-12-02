#!/bin/bash

awk '{ if ($1 == "forward") { f = f + $2 }; if ($1 == "up") { d = d - $2 }; if ( $1 == "down" ) { d = d + $2 }; { print f * d } }' input
