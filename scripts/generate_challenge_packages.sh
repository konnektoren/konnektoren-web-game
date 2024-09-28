#!/bin/bash

set -e  # Exit immediately if a command exits with a non-zero status.
set -x  # Print commands and their arguments as they are executed.

CHALLENGE_DIR="./challenge_packages"
OUTPUT_DIR="$BUILD_DIR/challenge_packages"

# Create output directory and check if it exists
mkdir -p "$OUTPUT_DIR"
if [ ! -d "$OUTPUT_DIR" ]; then
    echo "Failed to create output directory: $OUTPUT_DIR"
    exit 1
fi

# Check if we have write permissions in the output directory
if [ ! -w "$OUTPUT_DIR" ]; then
    echo "No write permission in output directory: $OUTPUT_DIR"
    exit 1
fi

# Check if zip is installed
if ! zip --version >/dev/null 2>&1; then
    echo "zip could not be found. Please install zip."
    exit 1
fi

# Loop through each challenge directory
for challenge in "$CHALLENGE_DIR"/*; do
    echo "Processing challenge: $challenge"
    if [ -d "$challenge" ]; then
        challenge_name=$(basename "$challenge")
        output_zip="$OUTPUT_DIR/${challenge_name}.zip"

        echo "Processing challenge: $challenge_name"
        echo "Output zip file: $output_zip"

        # Remove existing zip file if it exists
        rm -f "$output_zip"

        # Create zip file using absolute path for output_zip
        (cd "$challenge" && zip -r "$PWD/../../$output_zip" . -x "*/.*")

        # Check if zip file was created successfully
        if [ -f "$output_zip" ]; then
            echo "Created: $output_zip"
            ls -l "$output_zip"
        else
            echo "Failed to create: $output_zip"
        fi
    fi
done

echo "Challenge packages generated in $OUTPUT_DIR"
ls -la "$OUTPUT_DIR"
