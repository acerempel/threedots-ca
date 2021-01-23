#! /bin/bash
serve --index index.html --port 8000 bite &
trap 'kill %1' EXIT
fd . -t f bontent | entr -d ./target/debug/threedots -o bite
