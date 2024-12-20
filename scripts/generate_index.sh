set -euo pipefail

# Default values and parameters
TEMPLATE_FILE="${TEMPLATE_FILE:-index.html.template}"
DOMAIN="${DOMAIN:-https://konnektoren.help}"

# Extract site name from domain
SITE_NAME=$(echo "${DOMAIN}" | sed -E 's|https?://||' | sed -E 's/\.[^.]+$//')

echo "Generating index.html for:"
echo "  Domain: ${DOMAIN}"
echo "  Site Name: ${SITE_NAME}"

# Function to validate template file
validate_template() {
    if [ ! -f "${TEMPLATE_FILE}" ]; then
        echo "Error: Template file ${TEMPLATE_FILE} not found!"
        exit 1
    fi
}

# Function to replace placeholders in template
generate_index() {
    local output_file="index.html"
    local temp_file="${output_file}.tmp"

    # Create a temporary file for processing
    cat "${TEMPLATE_FILE}" > "${temp_file}"

    # Replace all placeholders
    sed -i.bak \
        -e "s|\${DOMAIN}|${DOMAIN}|g" \
        -e "s|\${SITE_NAME}|${SITE_NAME}|g" \
        "${temp_file}"

    # Move temporary file to final location
    mv "${temp_file}" "${output_file}"

    # Clean up backup files
    rm -f "${temp_file}.bak"

    echo "Generated: ${output_file}"
}

# Main execution
main() {
    # Validate template existence
    validate_template

    # Generate index.html in project root
    generate_index

    echo "Index file generation completed successfully"
}

# Execute main function
main
