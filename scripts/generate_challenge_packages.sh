#!/bin/bash

CHALLENGE_DIR="./challenge_packages"
OUTPUT_DIR="$BUILD_DIR/challenge_packages"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Loop through each challenge directory
for challenge in "$CHALLENGE_DIR"/*; do
    if [ -d "$challenge" ]; then
        challenge_name=$(basename "$challenge")
        (cd "$challenge" && zip -r "$OUTPUT_DIR/${challenge_name}.zip" *)
    fi
done

echo "Challenge packages generated in $OUTPUT_DIR"
