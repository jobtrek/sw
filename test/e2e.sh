#!/usr/bin/env bash

set -euo pipefail
cd "$(dirname "$0")/e2e"

if command -v fdfind &> /dev/null; then
	FD_CMD="fdfind"
else
	FD_CMD="fd"
fi
export FD_CMD

declare -A FAILED_FILES=()
while read -r FILE; do
	echo "Running $FILE:"
	# Run the script and print its output with 1 indentation level
	if ! bash "$FILE" 2>&1 | sed 's/^/\t/'; then
		FAILED_FILES["$FILE"]=${PIPESTATUS[0]}
	fi
done < <($FD_CMD -t f -e sh)

if [ ${#FAILED_FILES[@]} -eq 0 ]; then
	echo "All tests passed! ✅"
	exit 0
fi

echo -e "\nSome tests failed:"
for FILE in "${!FAILED_FILES[@]}"; do
	echo -e "\t$FILE failed with exit code ${FAILED_FILES[$FILE]} ❌"
done
exit 1

