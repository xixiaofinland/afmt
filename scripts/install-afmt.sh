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
    Linux*)  echo "linux" ;;
    Darwin*) echo "macos" ;;
    *)       echo "unsupported" ;;
  esac
}

detect_arch() {
  case "$(uname -m)" in
    x86_64|amd64)  echo "x86_64" ;;
    arm64|aarch64) echo "aarch64" ;;
    *)             echo "unsupported" ;;
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

  # Construct the pattern to match new naming: e.g. "linux-x86_64"
  local pattern="${os_type}-${arch_type}"

  echo "Looking for an asset matching: $pattern"
  # Extract the first matching browser_download_url
  local download_url
  download_url="$(echo "$release_data" \
    | grep -i "browser_download_url" \
    | grep "$pattern" \
    | grep ".tar.gz" \
    | cut -d '"' -f 4 \
    | head -n 1)"

  if [[ -z "$download_url" ]]; then
    echo "Error: No matching tar.gz asset found for $pattern."
    exit 1
  fi

  echo "Downloading from: $download_url"
  local filename
  filename="$(basename "$download_url")"

  # Download the asset
  curl -sL "$download_url" -o "${TMP_DIR}/${filename}"

  # Extract the tarball
  echo "Extracting $filename..."
  tar -xf "${TMP_DIR}/${filename}" -C "$TMP_DIR"

  # Move the afmt binary into INSTALL_DIR
  if [[ -f "${TMP_DIR}/${BINARY_NAME}" ]]; then
    mv "${TMP_DIR}/${BINARY_NAME}" "$INSTALL_DIR/"
    chmod +x "${INSTALL_DIR}/${BINARY_NAME}"
  else
    echo "Error: Could not find the '$BINARY_NAME' executable in the archive."
    exit 1
  fi

  # Clean up
  rm -rf "$TMP_DIR"

  # Define color codes
  GREEN='\e[32m'
  YELLOW='\e[33m'
  RESET='\e[0m'

  # Print completion message
  echo -e "${GREEN}Installed to: $INSTALL_DIR/$BINARY_NAME${RESET}"


# ----------------------------------------------------------------------------
  # 4. DETECT SHELL & PROMPT TO ADD PATH WITH ERROR HANDLING
  # ----------------------------------------------------------------------------

  # Check if INSTALL_DIR is already in PATH
  if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    # Detect user's shell from $SHELL
    current_shell="$(basename "$SHELL")"

    # Ask if user wants to update their shell profile automatically
    read -r -p "Would you like to add \"$INSTALL_DIR\" to your PATH now? (y/n) " answer
    if [[ "$answer" =~ ^[Yy]$ ]]; then
      case "$current_shell" in
        bash)
          PROFILE_FILE="$HOME/.bashrc"
          ;;
        zsh)
          PROFILE_FILE="$HOME/.zshrc"
          ;;
        fish)
          PROFILE_FILE="$HOME/.config/fish/config.fish"
          ;;
        *)
          PROFILE_FILE=""
          ;;
      esac

      if [[ -n "$PROFILE_FILE" ]]; then
        echo "Adding to $PROFILE_FILE ..."
        if [[ "$current_shell" == "fish" ]]; then
          # For fish shell, use fish_user_paths
          mkdir -p "$(dirname "$PROFILE_FILE")"
          echo "set -U fish_user_paths \$fish_user_paths $INSTALL_DIR" >> "$PROFILE_FILE" 2>/dev/null && \
            echo "Done! Open a new terminal or run 'exec fish' to refresh." || \
            { echo "Error: Failed to write to $PROFILE_FILE."; }
        else
          # For bash and zsh
          echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$PROFILE_FILE" 2>/dev/null && \
            echo "Done! Run 'source $PROFILE_FILE' or open a new terminal." || \
            { echo "Error: Failed to write to $PROFILE_FILE."; }
        fi
      else
        echo -e "${YELLOW}Unsupported shell detected: $current_shell.${RESET}"
        echo "Please add this line manually to your shell's config file:"
        echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
      fi
    else
      echo -e "\nYou can add this line to your shell profile manually (e.g. ~/.bashrc):"
      echo -e "${YELLOW}  export PATH=\"\$PATH:$INSTALL_DIR\"${RESET}\n"
    fi
  fi

  echo -e "\n=================== Completed! ====================\n"
  if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
      echo -e "${GREEN}Run: \"~/.local/bin/afmt -h\" to get started.${RESET}\n"
  else
      echo -e "${GREEN}Run: \"afmt -h\" to get started.${RESET}\n"
  fi
}

# ------------------------------------------------------------------------------
# 5. RUN
# ------------------------------------------------------------------------------
check_dependencies
main
