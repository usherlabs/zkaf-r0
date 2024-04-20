#!/bin/bash

OPERATION=$1
shift

echo "Pulling output from remote server at $OPERATION...";

rsync -azP -e ssh $OPERATION:~/zkaf/output/ ./output/
