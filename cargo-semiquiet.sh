#!/usr/bin/env bash

# Runs a command that contains cargo build output while hiding the 'Compiling' lines from that output while still
# showing the other output (including the build progress bar/summary).

# cargo clean

set -o errexit -o pipefail -o nounset

cols="${COLUMNS:-$(tput cols)}"

line=
while read -r -N1 char; do
	if [ "$char" = $'\n' ]; then
		last_line="$line"
		line=
		if [[ "$last_line" = *'Compiling'* ]]; then
			printf "\r%${cols}s\r" ''
			continue
		fi
	fi
	printf '%s' "$char"
	line+="$char"
done < <(
	unbuffer cargo "$@"
)
