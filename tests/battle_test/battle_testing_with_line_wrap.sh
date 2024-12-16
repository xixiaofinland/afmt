#!/bin/bash

# Get the absolute path of the current script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_LIST="$SCRIPT_DIR/repos.txt"
TARGET_DIR="$SCRIPT_DIR/repos"
# FORMATTER_BINARY="$SCRIPT_DIR/../../target/release/afmt"
FORMATTER_BINARY="$SCRIPT_DIR/../../target/debug/afmt"
LOG_FILE="$SCRIPT_DIR/format_errors.log"  # Log file for errors
LONG_LINES_LOG_FILE="$SCRIPT_DIR/long_lines.log"

LINE_LENGTH=85

# Create target directory if it doesn't exist
mkdir -p $TARGET_DIR

# Check if the formatter binary exists, if not, run cargo build --release
if [ ! -f "$FORMATTER_BINARY" ]; then
    echo "Formatter binary not found, building it with cargo..."
    (cd "$SCRIPT_DIR/../.." && cargo build --release)
    if [ $? -ne 0 ]; then
        echo "Cargo build failed, exiting."
        exit 1
    fi
fi

while IFS= read -r REPO_URL; do
    REPO_NAME=$(basename -s .git "$REPO_URL")
    echo "Cloning $REPO_URL into $TARGET_DIR/$REPO_NAME"
    git clone "$REPO_URL" "$TARGET_DIR/$REPO_NAME"
done < "$REPO_LIST"

# Clear the log file at the start
> "$LOG_FILE"
> "$LONG_LINES_LOG_FILE"

# Function to format files and log errors with clear info
format_files() {
    local FILE_PATH="$1"

    echo "Processing file: $FILE_PATH"

    OUTPUT=$($FORMATTER_BINARY "$FILE_PATH" 2>&1)
    EXIT_CODE=$?

    if [ $EXIT_CODE -ne 0 ]; then
        if echo "$OUTPUT" | grep -qE "snippet: %%{2,3}"; then
            :  # Skip logging for %% cases as they are from managed package templating
        elif echo "$OUTPUT" | grep -q "%%%"; then
            :  # Same as above
        # elif echo "$OUTPUT" | grep -q "/scripts/"; then
        #     :  # Same as above
        # elif echo "$OUTPUT" | grep -q "Parent node kind: class_body,"; then
        #     :  # Same as above
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

    # Extract lines exceeding the LINE_LENGTH and append to long_lines.log
    echo "$OUTPUT" | grep '^.\{81,\}$' >> "$LONG_LINES_LOG_FILE"
}

export -f format_files
export FORMATTER_BINARY
export LOG_FILE
export LONG_LINES_LOG_FILE
export LINE_LENGTH

# Record the start time
START_TIME=$(date +%s)

# Find all .cls and .trigger files and process them in parallel
find "$TARGET_DIR" \( -type d \( -name ".sfdx" -o -name "scripts" \) \) -prune -o -type f \( -name "*.cls" -o -name "*.trigger" \) -print0 | \
    parallel -0 -j+0 format_files

# find "$TARGET_DIR" -path "$TARGET_DIR/.sfdx" -prune -o -type f \( -name "*.cls" -o -name "*.trigger" \) -print0 | \
#     parallel -0 -j+0 format_files

# Check if any errors were logged
if [ -s "$LOG_FILE" ]; then
    echo "Errors occurred during formatting. Check $LOG_FILE for details."
else
    echo "All files processed successfully."
fi

if [ -s "$LONG_LINES_LOG_FILE" ]; then
    echo "Some files contain lines exceeding $LINE_LENGTH characters. Check $LONG_LINES_LOG_FILE for details."
else
    echo "No lines exceeding $LINE_LENGTH characters were found."
fi

# Record the end time and calculate the elapsed time
END_TIME=$(date +%s)
ELAPSED_TIME=$((END_TIME - START_TIME))

# Print the time taken
echo "Script execution time: $ELAPSED_TIME seconds"
