#!/bin/bash

# Step 1: Run everything inside the Nix environment as a subshell
nix develop ~/dotfiles-nix/#tree --command bash -c '
    # Step 2: Download the tree-sitter-sfapex repository
    REPO_URL="https://github.com/aheber/tree-sitter-sfapex"
    SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
    TARGET_DIR="$SCRIPT_DIR/tree-sitter-sfapex"
    if [ -d "$TARGET_DIR" ]; then
        echo "Removing existing $TARGET_DIR..."
        rm -rf "$TARGET_DIR"
    fi
    echo "Cloning the tree-sitter-sfapex repository..."
    git clone "$REPO_URL" || { echo "Failed to clone the repository"; exit 1; }

    # Step 3: generate c files and wasm package
    cd "$TARGET_DIR" || { echo "Failed to enter $TARGET_DIR"; exit 1; }
    echo "Generating parser files using tree-sitter..."
    tree-sitter generate ./apex/grammar.js || { echo "Failed to generate files"; exit 1; }
    cd apex
    tree-sitter build -w || { echo "Failed to build wasm"; exit 1; }
    cp tree-sitter-apex.wasm ../ || { echo "Failed to copy wasm pack"; exit 1; }
    cd ..

    # Step 4: Copy generated files to dep/
    echo "Copying parser.c, tree-sitter folder and wasm pack to dep/..."
    cp src/parser.c $SCRIPT_DIR || { echo "Failed to copy parser.c"; exit 1; }
    cp -r src/tree_sitter $SCRIPT_DIR || { echo "Failed to copy tree-sitter folder"; exit 1; }
    cp tree-sitter-apex.wasm $SCRIPT_DIR || { echo "Failed to copy wasm pack to dep/" exit 1; }

    # Step 5: Cleanup downloaded repository
    cd $SCRIPT_DIR || exit 1
    echo "Cleaning up downloaded tree-sitter-sfapex repository..."
    rm -rf "$TARGET_DIR"

    echo "Process completed successfully."
'
