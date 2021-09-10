#!/bin/sh
tmux new -d -s reaction-roles
tmux send -t reaction-roles cargo SPACE run ENTER
exit 0