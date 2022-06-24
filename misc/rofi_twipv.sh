#!/bin/bash

if [[ "$@" != "" ]]
then
	streamer=$(echo "$@" | cut -d" " -f1)
	coproc streamlink twitch.tv/$streamer best --player mpv 2>&1
	exit 1;
else
    $HOME/.cargo/bin/twipv rofi
fi
