#!/bin/bash

# Get the absolute path of the current script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Set TARGET_DIR relative to the script's directory
TARGET_DIR="$SCRIPT_DIR/repos"  # Adjust this path if needed
FORMATTER_BINARY="$SCRIPT_DIR/../../target/debug/afmt"  # Adjust this path if needed
LOG_FILE="$SCRIPT_DIR/format_errors.log"  # Log file for errors

# Clear the log file at the start
> "$LOG_FILE"

# Function to format files and log errors with clear info
format_files() {
    local FILE_PATH="$1"
    
    # Print the file path being processed
    echo "Processing file: $FILE_PATH"
    
    # Run the formatter and capture both stdout and stderr
    OUTPUT=$($FORMATTER_BINARY -f "$FILE_PATH" 2>&1)
    EXIT_CODE=$?
    
    if [ $EXIT_CODE -ne 0 ]; then
        # Check if the error contains "snippet: %%" or "snippet: %%%" and skip logging if true
        # Managed package code has this templating code `%%%Name_space%%%`, which has nothing to do with Apex
        if echo "$OUTPUT" | grep -qE "snippet: %%{2,3}"; then
            :  # No operation, skip logging
        elif echo "$OUTPUT" | grep -q "%%%"; then
            :  # Skip logging for %% cases with any number of percent signs
        else
            {
                echo "========================================"
                echo "Error while formatting file: $FILE_PATH"
                echo "Exit code: $EXIT_CODE"
                echo "----------------------------------------"
                echo "$OUTPUT"
                echo "========================================"
            } | tee -a "$LOG_FILE"
            return 1
        fi
    fi
}

# Loop over each repository in repos folder
for REPO_DIR in "$TARGET_DIR"/*; do
    echo "Processing repository: $REPO_DIR"
    # Find all .cls and .trigger files
    find "$REPO_DIR" -path "$REPO_DIR/.sfdx" -prune -o -type f \( -name "*.cls" -o -name "*.trigger" \) -print | while read -r FILE; do
        format_files "$FILE"
    done
done

# Check if any errors were logged
if [ -s "$LOG_FILE" ]; then
    echo "Errors occurred during formatting. Check $LOG_FILE for details."
else
    echo "All files processed successfully."
fi

