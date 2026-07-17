#!/bin/bash

# Usage:
# brew install universal-ctags
# pip3 install pytest
# ./tools/progress.sh > tmp/progress.txt && <tmp/progress.txt python3 tools/group.py > tmp/progress-grouped.txt && python3 -m pytest tools/group-test.py

header="$(pebble sdk include-path emery)/pebble.h"

for x in $(/opt/homebrew/bin/ctags -x --c-kinds=fp "$header" | awk '{print $1}'); do
  if grep -rq "sys::${x}\b" --include='*.rs' src/**; then
    found="🟩"
  else
    found="🟥"
  fi
  echo "${found} ${x}"
done
