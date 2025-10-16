#!/usr/bin/env bash

set -euo pipefail
cd "$(dirname "$0")/e2e"

if command -v fd &> /dev/null; then
	FD_CMD="fd"
elif command -v fdfind &> /dev/null; then
	FD_CMD="fdfind"
else
	echo "Error: 'fd' or 'fdfind' not found. Please install fd." >&2
	exit 1
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

echo ""
echo -e "Some tests failed: ❌"
CODE_LENGTH=0
for CODE in "${FAILED_FILES[@]}"; do
        (( ${#CODE} > CODE_LENGTH )) && CODE_LENGTH=${#CODE}
done
for FILE in "${!FAILED_FILES[@]}"; do
        printf '\t\e[0;31m[%*d]\e[0m - %s\n' "$CODE_LENGTH" "${FAILED_FILES[$FILE]}" "$FILE"
done
exit 1

