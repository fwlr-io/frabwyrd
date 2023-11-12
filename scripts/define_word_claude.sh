#!/bin/bash

# get key
apikey=$(<~/dev/ml/anthropic.key)

# make call
curl --silent \
     --request POST \
     --url https://api.anthropic.com/v1/complete \
     --header 'accept: application/json' \
     --header 'anthropic-version: 2023-06-01' \
     --header 'content-type: application/json' \
     --header "x-api-key: $apikey" \
     --data '
{
  "prompt": "\n\nHuman: Pretend that '$1' is a real English word. Give a single dictionary definition of '$1'.\n\nAssistant:",
  "model": "claude-2",
  "max_tokens_to_sample": 192,
  "stream": false
}
' | jq '.completion'