#!/bin/bash

TARGET_DIR="repos"
FORMATTER_BINARY="../../target/debug/afmt"  # Assuming the formatter is a binary
LOG_FILE="format_errors.log"            # Log file for errors

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
    
    # Check if an error occurred (non-zero exit code)
    if [ $EXIT_CODE -ne 0 ]; then
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
}

# Loop over each repository in repos folder
for REPO_DIR in "$TARGET_DIR"/*; do
    echo "Processing repository: $REPO_DIR"
    # Find all .cls and .trigger files
    find "$REPO_DIR" -type f \( -name "*.cls" -o -name "*.trigger" \) | while read -r FILE; do
        format_files "$FILE"
    done
done

# Check if any errors were logged
if [ -s "$LOG_FILE" ]; then
    echo "Errors occurred during formatting. Check $LOG_FILE for details."
else
    echo "All files processed successfully."
fi

