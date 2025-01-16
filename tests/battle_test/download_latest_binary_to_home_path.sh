#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Variables
REPO="xixiaofinland/afmt"
API_URL="https://api.github.com/repos/$REPO/releases/latest"
DOWNLOAD_DIR="/tmp/afmt_download"
INSTALL_DIR="$HOME"  # You can change this to "$HOME/afmt" if preferred

# Function to detect OS
detect_os() {
    OS_TYPE="$(uname -s)"
    case "$OS_TYPE" in
        Linux*)     OS="linux";;
        Darwin*)    OS="macos";;
        *)          OS="unsupported";;
    esac
    echo "$OS"
}

# Function to check dependencies
check_dependencies() {
    local dependencies=("curl")
    for cmd in "${dependencies[@]}"; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            echo "Error: '$cmd' is not installed. Please install it and retry."
            exit 1
        fi
    done

    # Check extraction tools based on file extension
    case "$EXT" in
        zip)
            if ! command -v unzip >/dev/null 2>&1; then
                echo "Error: 'unzip' is not installed. Please install it and retry."
                exit 1
            fi
            ;;
        gz|tgz|bz2)
            if ! command -v tar >/dev/null 2>&1; then
                echo "Error: 'tar' is not installed. Please install it and retry."
                exit 1
            fi
            ;;
    esac
}

# Function to add INSTALL_DIR to PATH (Optional)
add_to_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        SHELL_RC=""
        if [ -n "$BASH_VERSION" ]; then
            SHELL_RC="$HOME/.bashrc"
        elif [ -n "$ZSH_VERSION" ]; then
            SHELL_RC="$HOME/.zshrc"
        else
            SHELL_RC="$HOME/.profile"
        fi

        echo "Adding $INSTALL_DIR to PATH in $SHELL_RC"
        echo "" >> "$SHELL_RC"
        echo "# Added by afmt installer" >> "$SHELL_RC"
        echo "export PATH=\$PATH:$INSTALL_DIR" >> "$SHELL_RC"
        echo "Please restart your terminal or run 'source $SHELL_RC' to apply the changes."
    fi
}

# Detect Operating System
echo "Detecting operating system..."
OS=$(detect_os)

if [ "$OS" == "unsupported" ]; then
    echo "Error: Unsupported operating system. This script supports Linux and macOS only."
    exit 1
fi

echo "Operating system detected: $OS"

# Create a temporary download directory
mkdir -p "$DOWNLOAD_DIR"

echo "Fetching the latest release information from GitHub..."

# Fetch the latest release data from GitHub API
RELEASE_DATA=$(curl -s "$API_URL")

# Extract the download URL for the current OS asset
# This assumes that the asset name contains 'linux' or 'macos' (case-insensitive)
DOWNLOAD_URL=$(echo "$RELEASE_DATA" | grep -i "browser_download_url" | grep -i "$OS" | cut -d '"' -f 4 | head -n 1)

# Check if a download URL was found
if [ -z "$DOWNLOAD_URL" ]; then
    echo "Error: Could not find a $OS download asset in the latest release."
    exit 1
fi

echo "Download URL found: $DOWNLOAD_URL"

# Extract the filename from the URL
FILENAME=$(basename "$DOWNLOAD_URL")

echo "Downloading $FILENAME..."

# Download the asset using curl
curl -L "$DOWNLOAD_URL" -o "$DOWNLOAD_DIR/$FILENAME"

echo "Download completed."

# Determine the file extension and extract accordingly
EXT="${FILENAME##*.}"

echo "Checking dependencies..."
check_dependencies

echo "Extracting the downloaded file..."

cd "$DOWNLOAD_DIR"

case "$EXT" in
    zip)
        unzip "$FILENAME"
        ;;
    gz|tgz)
        tar -xzf "$FILENAME"
        ;;
    bz2)
        tar -xjf "$FILENAME"
        ;;
    *)
        echo "Error: Unsupported file extension '$EXT'. Cannot extract."
        exit 1
        ;;
esac

echo "Extraction completed."

# Locate the extracted directory
EXTRACTED_DIR=$(find "$DOWNLOAD_DIR" -mindepth 1 -maxdepth 1 -type d)

if [ -z "$EXTRACTED_DIR" ]; then
    echo "Error: Extraction failed or no directory found."
    exit 1
fi

echo "Extracted directory: $EXTRACTED_DIR"

# Path to the 'afmt' executable inside the extracted directory
AFMT_PATH="$EXTRACTED_DIR/afmt"

# Check if 'afmt' exists and is a file
if [ -f "$AFMT_PATH" ]; then
    echo "Found 'afmt' executable."
else
    echo "Error: 'afmt' executable not found in $EXTRACTED_DIR."
    exit 1
fi

# Create the installation directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Move the 'afmt' executable to the installation directory
mv "$AFMT_PATH" "$INSTALL_DIR/"

echo "Moved 'afmt' to $INSTALL_DIR."

# Set execute permissions on the 'afmt' executable
chmod +x "$INSTALL_DIR/afmt"

echo "Execute permission set for 'afmt'."

# Optional: Add the installation directory to PATH if not already present
# Uncomment the following line if you want to add INSTALL_DIR to your PATH
# add_to_path

# Clean up the temporary download directory
rm -rf "$DOWNLOAD_DIR"

echo "Installation of afmt is complete!"
echo "You can run 'afmt' from your terminal."
echo "If you did not add $INSTALL_DIR to your PATH, you can execute it using the full path: $INSTALL_DIR/afmt"
