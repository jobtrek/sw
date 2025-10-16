#/usr/bin/env bash

set -euo pipefail
cd "$(dirname "$0")/.."

cleanup() {
	git restore test
}

trap cleanup EXIT

DIFF_TOOL="${DIFF_TOOL:-diff}"

cargo run -- test --fd-bin-path "$FD_CMD"
DIFF="$($DIFF_TOOL test expected)"
cleanup
if [ -n "$DIFF" ]; then
	echo "Differences found between test and expected:"
	echo "$DIFF" | sed 's/^/\t/'
	exit 1
else
	echo "No differences found between test and expected."
fi
