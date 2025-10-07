#!/usr/bin/env bash

### Description
# A commit with an executable, a normal file, a symlink and an untracked fifo.
# All of them get changed in the worktree.
set -eu -o pipefail

git init
seq 5 8 >file
seq 1 3 >executable && chmod +x executable
ln -s nonexisting-target link
mkfifo fifo-should-be-ignored

git add . && git commit -m "init"

seq 5 10 >file
seq 1 5 >executable

rm link
ln -s other-nonexisting-target link

