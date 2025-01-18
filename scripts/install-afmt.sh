#!/usr/bin/env bash
#
# Install script for 'afmt' CLI.
# Usage: curl -sL https://example.com/install-afmt.sh | bash

set -euo pipefail

REPO="xixiaofinland/afmt"
API_URL="https://api.github.com/repos/${REPO}/releases/latest"
TMP_DIR="$(mktemp -d)"
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="afmt"

# ------------------------------------------------------------------------------
# 1. OS + ARCH DETECTION
# ------------------------------------------------------------------------------
detect_os() {
  case "$(uname -s)" in
    Linux*)   echo "unknown-linux-musl" ;;
    Darwin*)  echo "apple-darwin" ;;
    *)        echo "unsupported" ;;
  esac
}

detect_arch() {
  case "$(uname -m)" in
    x86_64)  echo "x86_64" ;;
    amd64)   echo "x86_64" ;;  # handle "amd64" as x86_64
    arm64)   echo "aarch64" ;; # handle "arm64" as aarch64
    aarch64) echo "aarch64" ;;
    *)       echo "unsupported" ;;
  esac
}

# ------------------------------------------------------------------------------
# 2. DEPENDENCY CHECK
# ------------------------------------------------------------------------------
check_dependencies() {
  local deps=("curl" "tar")
  for dep in "${deps[@]}"; do
    if ! command -v "$dep" >/dev/null 2>&1; then
      echo "Error: '$dep' is not installed. Please install it and retry."
      exit 1
    fi
  done
}

# ------------------------------------------------------------------------------
# 3. MAIN INSTALLATION LOGIC
# ------------------------------------------------------------------------------
main() {
  echo "Detecting OS and architecture..."
  local os_type arch_type
  os_type="$(detect_os)"
  arch_type="$(detect_arch)"

  if [[ "$os_type" == "unsupported" || "$arch_type" == "unsupported" ]]; then
    echo "Error: Unsupported OS or architecture."
    echo "This installer supports Linux/macOS on x86_64/aarch64."
    exit 1
  fi

  # Ensure install dir exists
  mkdir -p "$INSTALL_DIR"

  echo "OS detected: $os_type"
  echo "Arch detected: $arch_type"
  echo "Fetching latest release from GitHub ($REPO)..."
  local release_data
  release_data="$(curl -s "$API_URL")"

  # Construct a pattern to match the relevant asset name
  # e.g., "afmt-v0.4.0-x86_64-apple-darwin.tar.gz"
  local pattern="${arch_type}-${os_type}"

  echo "Looking for an asset matching pattern: $pattern"
  # Extract the first matching browser_download_url from release data
  local download_url
  download_url="$(echo "$release_data" \
    | grep -i "browser_download_url" \
    | grep "$pattern" \
    | cut -d '"' -f 4 \
    | head -n 1)"

  if [[ -z "$download_url" ]]; then
    echo "Error: No matching asset found for $pattern."
    exit 1
  fi

  echo "Downloading from: $download_url"
  local filename
  filename="$(basename "$download_url")"

  # Download the asset
  curl -sL "$download_url" -o "${TMP_DIR}/${filename}"

  # Extract the tar.gz into TMP_DIR
  echo "Extracting $filename..."
  tar -xf "${TMP_DIR}/${filename}" -C "$TMP_DIR"

  # Move the afmt binary into INSTALL_DIR
  # (Assume the extracted file is named 'afmt'; adjust if needed)
  if [[ -f "${TMP_DIR}/${BINARY_NAME}" ]]; then
    mv "${TMP_DIR}/${BINARY_NAME}" "$INSTALL_DIR/"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
  else
    echo "Error: Could not find the '$BINARY_NAME' executable in the archive."
    exit 1
  fi

  # Clean up
  rm -rf "$TMP_DIR"

  echo "============================================================"
  echo "afmt installed to: $INSTALL_DIR/$BINARY_NAME"
  echo ""

  # Suggest adding INSTALL_DIR to PATH if it isn't there
  if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "Note: $INSTALL_DIR is not in your PATH."
    echo "Add the following line to your shell profile (e.g. ~/.bashrc or ~/.zshrc):"
    echo ""
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    echo ""
    echo "Then, reload your shell or open a new terminal session."
  fi

  echo "Installation complete! Run 'afmt --help' to get started."
}

# ------------------------------------------------------------------------------
# 4. RUN
# ------------------------------------------------------------------------------
check_dependencies
main
