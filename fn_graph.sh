#!/bin/sh

(cat src/veryl_walker.rs \
    | sed -e 's/\/\/.*//g' \
    | grep -Eo 'fn(.*)\(|self\.(.*)\(' \
    | sed \
        -e 's/(//g' -e 's/fn.*/& -> {/g' \
        -e 's/fn /}\n/g' -e 's/self\.//g' -e 's/r#//g' \
        -e '0,/}/ s//digraph trait {/';
echo '}}') > fn_graph.gv;

cat fn_graph.gv | dot -Tsvg > fn_graph.svg;
cat fn_graph.gv | tred | dot -Tsvg > fn_tred.svg
