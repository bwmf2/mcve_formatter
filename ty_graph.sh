#!/bin/sh

(cat src/veryl_grammar_trait.rs \
    | sed -e 's/\/\/.*//g' -e 's/\/\*.*\*\///g' -e 's/#.*//g' \
    | grep -Eo 'pub (.*) {|Box<(.*)>|Vec<(.*)>|\((.*)\)' \
    | sed \
        -e 's/Box<//g' -e 's/Vec<//g' -e 's/(//g' \
        -e 's/)//g' -e 's/>//g' -e 's/ {//g' \
        -e 's/pub.*/& -> {/g' -e 's/pub \(struct\|enum\) /}\n/g' \
        -e '0,/}/ s//digraph type {/';
echo '}}') > ty_graph.gv;

cat ty_graph.gv | dot -Tsvg > ty_graph.svg;
cat ty_graph.gv | tred | dot -Tsvg > ty_tred.svg
