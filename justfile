# justfile

# Set default values for environment variables
export BUILD_DIR := env_var_or_default("BUILD_DIR", "dist")
export DOMAIN := env_var_or_default("DOMAIN", "https://konnektoren.help")
export SITEMAP := env_var_or_default("SITEMAP", "sitemap.txt")
export REPORTS_DIR := env_var_or_default("REPORTS_DIR", "reports")

# Default recipe to display help information
default:
    @just --list

# Install required dependencies
setup:
    cargo install trunk
    cargo install wasm-pack
    rustup target add wasm32-unknown-unknown

# Start development server
serve:
    trunk serve

# Build the project for release
build:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building with BUILD_DIR=${BUILD_DIR}"
    echo "Using DOMAIN=${DOMAIN}"

    # Create build directory if it doesn't exist
    mkdir -p ${BUILD_DIR}

    # Main build
    trunk build --release

    # Generate additional files
    BUILD_DIR=${BUILD_DIR} ./scripts/generate_challenge_packages.sh
    BUILD_DIR=${BUILD_DIR} ./scripts/generate_pages_index.sh

    # Generate sitemaps
    BUILD_DIR=${BUILD_DIR} \
    DOMAIN=${DOMAIN} \
    ./scripts/generate_sitemap.sh

# Build for specific environment
build-env env="development":
    #!/usr/bin/env bash
    case "{{env}}" in
        "production")
            export DOMAIN="https://konnektoren.help"
            ;;
        "staging")
            export DOMAIN="https://staging.konnektoren.help"
            ;;
        "development")
            export DOMAIN="http://localhost:8080"
            ;;
        *)
            echo "Unknown environment: {{env}}"
            exit 1
            ;;
    esac
    just build

# Run all tests
test: test-cargo test-wasm test-i18n

# Run cargo tests
test-cargo:
    cargo test

# Run wasm tests in Firefox
test-wasm:
    wasm-pack test --headless --firefox

# Run i18n completeness check
test-i18n:
    #!/usr/bin/env bash
    chmod +x ./scripts/i18n_report.sh
    ./scripts/i18n_report.sh

# Generate i18n report
i18n-report:
    #!/usr/bin/env bash
    ./scripts/i18n_report.sh

# Submit URLs to search engines using IndexNow
submit-indexnow domain=DOMAIN:
    #!/usr/bin/env bash
    BUILD_DIR=${BUILD_DIR} \
    DOMAIN=${domain} \
    SITEMAP=${SITEMAP} \
    ./scripts/indexnow_submit.sh

# Clean build artifacts and reports
clean:
    rm -rf ${BUILD_DIR}
    rm -rf reports
    cargo clean

# Format code
fmt:
    cargo fmt

# Check code formatting
fmt-check:
    cargo fmt --check

# Run clippy lints
lint:
    cargo clippy -- -D warnings

# Show current configuration
config:
    @echo "Current configuration:"
    @echo "BUILD_DIR: ${BUILD_DIR}"
    @echo "DOMAIN: ${DOMAIN}"
    @echo "SITEMAP: ${SITEMAP}"
