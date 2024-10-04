#!/bin/bash

# Get the absolute path of the current script's directory
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

REPO_LIST="$SCRIPT_DIR/repos.txt"
TARGET_DIR="$SCRIPT_DIR/repos"


# Create target directory if it doesn't exist
mkdir -p $TARGET_DIR

# Loop over each repository in repos.txt
while IFS= read -r REPO_URL; do
    # Extract repo name from URL
    REPO_NAME=$(basename -s .git "$REPO_URL")

    # Clone the repository
    echo "Cloning $REPO_URL into $TARGET_DIR/$REPO_NAME"
    git clone "$REPO_URL" "$TARGET_DIR/$REPO_NAME"
done < "$REPO_LIST"
