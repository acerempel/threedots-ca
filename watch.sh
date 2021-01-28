#!/bin/bash
miniserve --index index.html --port 8000 build_local &
trap 'kill %1' EXIT
fd . -t f source | entr -d ./vendor/bin/jigsaw build --cache
