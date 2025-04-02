#!/bin/bash

# -----------------------------------------------------------------------------
# Script to format files in repos with improved git error handling
# -----------------------------------------------------------------------------

# Exit on error, but with proper error handling
set -eo pipefail

# Get the absolute path of the current script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_LIST="$SCRIPT_DIR/repos.txt"
TARGET_DIR="$SCRIPT_DIR/repos"
FORMATTER_BINARY="$SCRIPT_DIR/../../target/debug/afmt"
LOG_FILE="$SCRIPT_DIR/format_errors.log"

# Enhanced error handling function
handle_error() {
    local exit_code=$?
    echo "Error occurred in script at line $1, exit code: $exit_code"
    exit $exit_code
}

trap 'handle_error ${LINENO}' ERR

# Create target directory if it doesn't exist
mkdir -p "$TARGET_DIR"

# Check if the formatter binary exists
if [ ! -f "$FORMATTER_BINARY" ]; then
    echo "Formatter binary not found, building it with cargo..."
    (cd "$SCRIPT_DIR/../.." && cargo build) || {
        echo "Cargo build failed, exiting."
        exit 1
    }
fi

# Clear the log file at the start
> "$LOG_FILE"

# Function to check if a repository is accessible
check_repo_availability() {
    local repo_url="$1"
    # Try to do a lightweight ls-remote to check repo accessibility
    if git ls-remote --quiet --exit-code "$repo_url" HEAD &>/dev/null; then
        return 0
    else
        return 1
    fi
}

# Clone repositories with better error handling
while IFS= read -r REPO_URL || [ -n "$REPO_URL" ]; do
    # Skip empty lines and comments
    [[ -z "$REPO_URL" || "$REPO_URL" =~ ^# ]] && continue

    REPO_NAME=$(basename -s .git "$REPO_URL")
    REPO_PATH="$TARGET_DIR/$REPO_NAME"

    echo "Checking availability of $REPO_URL..."

    if ! check_repo_availability "$REPO_URL"; then
        echo "Warning: Repository $REPO_URL appears to be inaccessible, skipping..."
        echo "Failed to access repository: $REPO_URL" >> "$LOG_FILE"
        continue
    fi

    echo "Cloning $REPO_URL into $REPO_PATH"

    if [ -d "$REPO_PATH" ]; then
        echo "Directory already exists, removing it..."
        rm -rf "$REPO_PATH"
    fi

    # Clone with retries and proper error handling
    for i in {1..3}; do
        if git clone --depth 1 --single-branch "$REPO_URL" "$REPO_PATH" 2>> "$LOG_FILE"; then
            break
        else
            if [ $i -eq 3 ]; then
                echo "Failed to clone $REPO_URL after 3 attempts"
                continue 2
            fi
            echo "Attempt $i failed, retrying..."
            sleep 2
        fi
    done
done < "$REPO_LIST"

# Clear the log files at the start
> "$LOG_FILE"
# > "$LONG_LINES_LOG_FILE"

# Check for idempotent mode flag
IDEMPOTENT_MODE=false
if [ "$1" == "--idempotent" ]; then
    IDEMPOTENT_MODE=true
    echo "Idempotent testing mode activated."
fi

# Function to format files and log errors with clear info
format_files() {
    local FILE_PATH="$1"

    # echo "Processing file: $FILE_PATH"

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

    # Log long lines
    # PATTERN="^.{$((LINE_LENGTH + 1)),}$"
    # echo "$OUTPUT" | grep -E "$PATTERN" >> "$LONG_LINES_LOG_FILE"
}

idempotent_test() {
    local FILE_PATH="$1"
    TMP1=$(mktemp)
    TMP2=$(mktemp)

    # Format once and save output to TMP1
    $FORMATTER_BINARY "$FILE_PATH" > "$TMP1" 2>/dev/null

    # Format the result of the first formatting and save to TMP2
    $FORMATTER_BINARY "$TMP1" > "$TMP2" 2>/dev/null

    # Capture detailed diff output
    DIFF_OUTPUT=$(diff "$TMP1" "$TMP2")
    if [ -n "$DIFF_OUTPUT" ]; then
        echo "Idempotency test failed for $FILE_PATH" >> "$LOG_FILE"
        echo "Diff details:" >> "$LOG_FILE"
        echo "$DIFF_OUTPUT" >> "$LOG_FILE"
        echo "Difference found in idempotency test for: $FILE_PATH"
    else
        echo "Idempotency test passed for $FILE_PATH"
    fi

    rm -f "$TMP1" "$TMP2"
}

export -f format_files
export -f idempotent_test
export FORMATTER_BINARY
export LOG_FILE
# export LONG_LINES_LOG_FILE
# export LINE_LENGTH

# Record the start time
START_TIME=$(date +%s)

# Find all .cls and .trigger files and process them in parallel
find "$TARGET_DIR" \( -type d \( -name ".sfdx" -o -name "scripts" \) \) -prune -o -type f \( -name "*.cls" -o -name "*.trigger" \) -print0 | \
    parallel -0 -j+0 format_files

# Run idempotent testing if mode is activated
if [ "$IDEMPOTENT_MODE" = true ]; then
    echo "Running idempotency tests..."
    find "$TARGET_DIR" \( -type d \( -name ".sfdx" -o -name "scripts" \) \) -prune -o -type f \( -name "*.cls" -o -name "*.trigger" \) -print0 | \
        parallel -0 -j+0 idempotent_test
fi

# find "$TARGET_DIR" -path "$TARGET_DIR/.sfdx" -prune -o -type f \( -name "*.cls" -o -name "*.trigger" \) -print0 | \
#     parallel -0 -j+0 format_files

# Record the end time and calculate the elapsed time
END_TIME=$(date +%s)
ELAPSED_TIME=$((END_TIME - START_TIME))

# Check if any errors were logged
if [ -s "$LOG_FILE" ]; then
    echo "Errors occurred during formatting. Check $LOG_FILE for details."
    echo "Script execution time: $ELAPSED_TIME seconds"
    exit 1
else
    echo "All files processed successfully."
    echo "Script execution time: $ELAPSED_TIME seconds"
    exit 0
fi

