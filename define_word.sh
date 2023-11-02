#!/bin/sh

# get next word from sqlite to define based on llm 

bllm="mis''tral"
llm=$(echo $bllm | sed "s/'/''/g")
echo $llm

word=$(sqlite3 -init /dev/null -batch frabwyrd.db "select word from list where (keep = 1) and word not in (select dict.word from dict where (dict.word = word) and (dict.llm = '$llm')) limit 1;")
echo $word

# capture result and insert safely into sqlite