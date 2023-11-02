
definition=$(~/dev/ml/llama.cpp/main --simple-io -m ~/dev/ml/llama.cpp/models/mistral-7b-instruct-v0.1.Q4_K_M.gguf -p "Pretend that '$1' is a real English word. Give a dictionary definition of '$1'." 2>/dev/null)
echo ${definition#*:}