#!/bin/bash

# File: konnektoren-web-game/scripts/indexnow_submit.sh

DOMAIN="https://konnektoren.help"
BUILD_DIR=${BUILD_DIR:-"./dist"}
INDEXNOW_KEY_FILE="indexnow_key.txt"

# Generate IndexNow key if it doesn't exist
if [ ! -f "$INDEXNOW_KEY_FILE" ]; then
    INDEXNOW_KEY=$(openssl rand -hex 16)
    echo $INDEXNOW_KEY > $INDEXNOW_KEY_FILE
else
    INDEXNOW_KEY=$(cat $INDEXNOW_KEY_FILE)
fi

# Create key file in the build directory
echo $INDEXNOW_KEY > "$BUILD_DIR/$INDEXNOW_KEY.txt"

# Check if sitemap.txt exists
if [ ! -f "sitemap.txt" ]; then
    echo "sitemap.txt not found. Please run generate_sitemap.sh first."
    exit 1
fi

# Prepare URL list for submission
URL_LIST=$(cat sitemap.txt | sed 's/^/"/;s/$/"/' | tr '\n' ',' | sed 's/,$//')

# Submit URLs to Bing's IndexNow API
curl -X POST "https://www.bing.com/IndexNow" \
     -H "Content-Type: application/json" \
     -d "{
         \"host\": \"${DOMAIN}\",
         \"key\": \"${INDEXNOW_KEY}\",
         \"urlList\": [${URL_LIST}]
     }"

# Submit to Yandex's IndexNow API
curl -X POST "https://yandex.com/indexnow" \
    -H "Content-Type: application/json" \
    -d "{
        \"host\": \"${DOMAIN}\",
        \"key\": \"${INDEXNOW_KEY}\",
        \"urlList\": [${URL_LIST}]
    }"

echo "IndexNow submission completed."
