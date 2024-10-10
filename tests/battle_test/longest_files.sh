#!/bin/bash

# Navigate to the repos folder
cd repos || exit

# Create the log file in the repos folder
log_file="longest_files.log"
> "$log_file"

# Create a directory to copy the top files into
output_dir="top_files"
mkdir -p "$output_dir"

# Find all *.cls and *.trigger files, count lines, and sort them by line count
find . -type f \( -name "*.cls" -o -name "*.trigger" \) \
  -exec wc -l {} + | sort -nr | head -n 250 | while read -r line; do
    # Extract the line count and file path
    line_count=$(echo "$line" | awk '{print $1}')
    file_path=$(echo "$line" | awk '{$1=""; print $0}' | xargs) # Remove line count to get file path

    # Ensure the file path is valid and exists
    if [[ -f "$file_path" ]]; then
      # Print the line count and file path into the log file
      echo "$line_count $file_path" >> "$log_file"

      # Copy the file to the output directory
      cp "$file_path" "$output_dir/"
    fi
done

# Print the results
echo "Top 200 longest .cls and .trigger files by line count saved to $log_file"
echo "Files copied to $output_dir/"

