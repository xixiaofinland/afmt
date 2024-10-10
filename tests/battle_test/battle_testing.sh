#!/bin/bash

# Record the start time
START_TIME=$(date +%s)

# Get the absolute path of the current script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_LIST="$SCRIPT_DIR/repos.txt"
TARGET_DIR="$SCRIPT_DIR/repos"
FORMATTER_BINARY="$SCRIPT_DIR/../../target/debug/afmt"
LOG_FILE="$SCRIPT_DIR/format_errors.log"  # Log file for errors

# Create target directory if it doesn't exist
mkdir -p $TARGET_DIR

while IFS= read -r REPO_URL; do
    REPO_NAME=$(basename -s .git "$REPO_URL")
    echo "Cloning $REPO_URL into $TARGET_DIR/$REPO_NAME"
    git clone "$REPO_URL" "$TARGET_DIR/$REPO_NAME"
done < "$REPO_LIST"

> "$LOG_FILE"

format_files() {
    local FILE_PATH="$1"

    echo "Processing file: $FILE_PATH"

    OUTPUT=$($FORMATTER_BINARY -f "$FILE_PATH" 2>&1)
    EXIT_CODE=$?

    if [ $EXIT_CODE -ne 0 ]; then
        if echo "$OUTPUT" | grep -qE "snippet: %%{2,3}"; then
            :  # managed package code can have `%%` `%%%` as templating code. Skip logging them
        elif echo "$OUTPUT" | grep -q "Error in node kind: ;"; then
           :
        else
            {
                echo "========================================"
                echo "Error while formatting file: $FILE_PATH"
                echo "Exit code: $EXIT_CODE"
                echo "----------------------------------------"
                echo "$OUTPUT"
                echo "========================================"
            } >> "$LOG_FILE"
            return 1
        fi
    fi
}

export -f format_files
export FORMATTER_BINARY
export LOG_FILE

# Find all .cls and .trigger files and process them in parallel
find "$TARGET_DIR" -path "$TARGET_DIR/.sfdx" -prune -o -type f \( -name "*.cls" -o -name "*.trigger" \) -print0 | \
    parallel -0 -j+0 format_files

# Check if any errors were logged
if [ -s "$LOG_FILE" ]; then
    echo "Errors occurred during formatting. Check $LOG_FILE for details."
else
    echo "All files processed successfully."
fi

# Record the end time and calculate the elapsed time
END_TIME=$(date +%s)
ELAPSED_TIME=$((END_TIME - START_TIME))

# Print the time taken
echo "Script execution time: $ELAPSED_TIME seconds"

