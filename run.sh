#!/bin/sh

for n in $(seq 1 32); do
    echo $n;
    sed -e "s/Align; .*\]/Align; $n]/g" src/lib.rs -i;
    time -p sh -c 'timeout -s INT 77 cargo +stable build --release 2> /dev/null';
    # time -p sh -c 'timeout -s INT 77 cargo +nightly build --release 2> /dev/null';
done;
