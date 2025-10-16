#/usr/bin/env bash

cd "$(dirname "$0")/e2e"

FILES="$(fd -t f -e sh)"
declare -A FAILED_FILES

while read -r FILE; do
	echo "Running $FILE:"

	# Run the script and print its output with 1 indentation level
	bash "$FILE" 2>&1 | sed 's/^/\t/'
	LAST_EXIT_CODE=${PIPESTATUS[0]}

	if [ $LAST_EXIT_CODE -ne 0 ]; then
		FAILED_FILES["$FILE"]=$LAST_EXIT_CODE
	fi
done < <(echo "$FILES")

if [ ${#FAILED_FILES[@]} -eq 0 ]; then
	echo "All tests passed! ✅"
	exit 0
fi

echo -e "\nSome tests failed:"
for FILE in "${!FAILED_FILES[@]}"; do
	echo -e "\t$FILE failed with exit code ${FAILED_FILES[$FILE]} ❌"
done
exit 1

