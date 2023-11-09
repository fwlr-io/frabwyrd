#!/bin/sh

logging=true

# llm model locations
hermes=~/dev/ml/llama.cpp/models/openhermes-2.5-mistral-7b.Q5_K_M.gguf
mistral=~/dev/ml/llama.cpp/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf
llama=~/dev/ml/llama.cpp/models/llama-2-7b-chat.Q4_K_M.gguf

# api keys

# 

# determine which llm to use

use_local_model=false
use_api_model=false

case $1 in
  mistral)
    llm='mistral'
    model=$mistral
    use_local_model=true
    ;;

  hermes)
    llm='hermes'
    model=$hermes
    use_local_model=true
    ;;

  llama)
    llm='llama'
    model=$llama
    use_local_model=true
    ;;

  gpt3)
    llm='gpt3'
    api=""
    apikey=""
    use_api_model=true
    ;;

  gpt4)
    llm='gpt4'
    api=""
    apikey=""
    use_api_model=true
    ;;

  claude)
    llm='claude'
    api=""
    apikey=""
    use_api_model=true
    ;;

  grok)
    llm='grok'
    api=""
    apikey=""
    use_api_model=true
    ;;

  *)
    echo "unrecognised LLM option given: $1"
    exit 1
    ;;
esac

if $logging; then
  echo "using llm: $llm"

  if $use_local_model; then
    echo "using local model at $model"
  elif $use_api_model; then
    echo "using api at $api"
  else
    echo "local/api not specified, exiting"
    exit 1
  fi

fi


# get next word to define from db, based on llm and what's already defined

word=$(sqlite3 -init /dev/null -batch ../frabwyrd.db "select word from list where (keep = 1) and word not in (select dict.word from dict where (dict.word = word) and (dict.llm = '$llm')) limit 1;")

if $logging; then
  echo "word to define: $word"
fi




# capture result...

if $use_local_model; then
  # run the model
  raw_definition=$(~/dev/ml/llama.cpp/main --simple-io -m $model -p "Pretend that '$word' is a real English word. Give a dictionary definition of '$word'." 2>/dev/null)

elif $use_api_model; then
  raw_definition="nothing yet"
  echo "api calls not implemented yet"
  exit 1

else
  echo "local/api not specified, exiting"
  exit 1
fi


definition=$(echo ${raw_definition#*:} | sed "s/'/''/g")

if $logging; then
  echo 
  echo "definition for $word, as provided by $llm:"
  echo "$definition"
fi



# ...and insert safely into sqlite
# all that needs to be escaped for sqlite is single quotes
# achieved with 
#   sed "s/'/''/g"

echo "  (  and here is where we would insert '$word', '$llm', '$definition'  )"
# sqlite3 -init /dev/null -batch ../frabwyrd.db ""