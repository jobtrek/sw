#/usr/bin/env bash

cd "$(dirname "$0")/.."

DIFF_TOOL="${DIFF_TOOL:-diff}"

cargo run -- test --fd-bin-path "$FD_CMD"
DIFF="$($DIFF_TOOL test expected)"
git restore test
if [ -n "$DIFF" ]; then
	echo "Differences found between test and expected:"
	echo "$DIFF" | sed 's/^/\t/'
	exit 1
else
	echo "No differences found between test and expected."
fi
