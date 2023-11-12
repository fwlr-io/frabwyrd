#!/bin/bash

logging=true

# local model scripts
# hermes=~/dev/ml/frabwyrd/scripts/define_word_hermes.sh
mistral=~/dev/ml/frabwyrd/scripts/define_word_mistral.sh
llama=~/dev/ml/frabwyrd/scripts/define_word_llama.sh
# to run: . $hermes foo

# remote model scripts
gpt3=~/dev/ml/frabwyrd/scripts/define_word_gpt3.sh
gpt4=~/dev/ml/frabwyrd/scripts/define_word_gpt4.sh
claude=~/dev/ml/frabwyrd/scripts/define_word_claude.sh
# grok=
# to run: . $gpt3 foo

# determine which llm to use

case $1 in
  mistral)
    llm='mistral' ;;
  # hermes)
    # llm='hermes'  ;;
  llama)
    llm='llama'   ;;
  gpt3)
    llm='gpt3'    ;;
  gpt4)
    llm='gpt4'    ;;
  claude)
    llm='claude'  ;;
  # grok)
    # llm='grok'    ;;
  *)
    echo "unrecognised LLM option given: $1"
    exit 1
    ;;
esac

model=${!llm}

if $logging; then
  echo "using llm: $llm"
  echo "using script: $model"
fi


# get next word to define from db, based on llm and what's already defined

word=$(sqlite3 -init /dev/null -batch frabwyrd.db "select word from list where (keep = 1) and word not in (select dict.word from dict where (dict.word = word) and (dict.llm = '$llm')) limit 1;")

if $logging; then
  echo "word to define: $word"
fi


# capture result...

raw_definition=$(. $model $word)

# definition=$(echo ${raw_definition#*:} | sed "s/'/''/g")

if $logging; then
  echo 
  echo "definition for $word, as provided by $llm:"
  # echo "$definition"
  echo "$raw_definition"
fi



# ...and insert safely into sqlite
# all that needs to be escaped for sqlite is single quotes
# achieved with 
#   sed "s/'/''/g"

# echo "  (  and here is where we would insert '$word', '$llm', '$definition'  )"
echo "  (  and here is where we would insert '$word', '$llm', '$raw_definition'  )"
# sqlite3 -init /dev/null -batch frabwyrd.db ""