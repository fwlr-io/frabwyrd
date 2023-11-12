#!/bin/sh
prompt="Pretend that '$1' is a real English word. Give a dictionary definition of '$1'. Do not ask or answer further questions about the word."
definition=$(~/dev/ml/llama.cpp/main --simple-io -m ~/dev/ml/llama.cpp/models/openhermes-2.5-mistral-7b.Q5_K_M.gguf -p "$prompt" 2>/dev/null)
echo ${definition#$prompt}