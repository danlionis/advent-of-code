#!/bin/bash

for DAY in {1..25}
do 

    if (( $DAY < 10 ));then
        DAY=0$DAY
    fi

    if test -f ./$DAY.py
    then
        echo "DAY $DAY:" $(cat input/$DAY.txt | wc -l) lines
        python3 $DAY.py < input/$DAY.txt
        echo ""
    fi 

done
