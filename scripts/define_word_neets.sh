#!/bin/bash

# get key
apikey=$(<~/dev/ml/neets.key)
echo $apikey

# make call
curl -vv https://api.neets.ai/v1/chat/completions \
  -H "Authorization: Bearer $apikey" \
  -H "Content-Type: application/json" \
  -d '{
     "model": "Neets-7B",
     "max_tokens": 400,
     "messages": [{"role": "user", "content": "Pretend that '$1' is a real English word. Give a dictionary definition of '$1'."}]
   }' 
   # | jq '.choices[0].message.content'