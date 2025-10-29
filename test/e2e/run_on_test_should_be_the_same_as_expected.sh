#!/usr/bin/env bash

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

cleanup() {
	git restore test
}

trap cleanup EXIT

DIFF_TOOL="${DIFF_TOOL:-diff -r}"

./sw test --fd-bin-path "$FD_CMD"
if DIFF=$($DIFF_TOOL test expected); then
	echo "No differences found between test and expected."
elif [ $? -eq 1 ]; then
	echo "Differences found between test and expected:"
	echo "$DIFF" | sed 's/^/\t/'
	exit 1
else
	echo "Error occurred while comparing test and expected." >&2
	exit 2
fi
