#!/bin/bash

for DAY in $(fdfind "\d\d.py" | sed -e "s/\.py//g")
do 
    echo "DAY $DAY:" $(cat input/$DAY.txt | wc -l) lines
    python3 $DAY.py < input/$DAY.txt

    echo ""
done
