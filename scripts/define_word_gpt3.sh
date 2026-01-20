#!/bin/bash

# get key
apikey=$(<~/dev/ml/openai.key)

# make call
curl -s https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $apikey" \
  -d '{
     "model": "gpt-3.5-turbo",
     "messages": [{"role": "user", "content": "Pretend that '$1' is a real English word. Give a dictionary definition of '$1'."}],
     "temperature": 0.7
   }' \
   | jq '.choices[0].message.content'