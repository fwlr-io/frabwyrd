while true; do
  echo "\n"
  word=$(sqlite3 -init /dev/null -batch frabwyrd.db 'select word from list where seen = 0 limit 1;')
  read -n 1 -s -p $word decision
  if [ $decision = "y" ]; then
    sqlite3 -init /dev/null -batch frabwyrd.db "update list set seen = true, keep = true where word = \"$word\";"
  elif [ $decision = "n" ]; then
    sqlite3 -init /dev/null -batch frabwyrd.db "update list set seen = true, keep = false where word = \"$word\";"
  else
    echo "\nquitting"
    break
  fi
done