#!/bin/bash

OPERATION=$1
shift

echo "Synchronising local repository to remote server at $OPERATION...";

rsync -vhra -e ssh ./ $OPERATION:~/zkaf/ --include='**.gitignore' --exclude='/.git' --filter=':- .gitignore' --delete-after
