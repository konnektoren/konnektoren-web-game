# generate_pages_index.sh
PAGES=("map/" "about/" "achievements/" "challenges/" "leaderboard/" "profile/" "marketplace/" "backup/")
BUILD_DIR=${BUILD_DIR:-dist}

# Create temporary directory
mkdir -p dist_temp

# Copy all files from dist to dist_temp
cp -r $BUILD_DIR/* dist_temp/

# Create folders for each page and copy index.html
for page in "${PAGES[@]}"; do
    mkdir -p "dist_temp/${page%/}"  # Remove trailing slash for mkdir
    cp dist_temp/index.html "dist_temp/${page%/}/index.html"
done

# Remove old dist folder and rename dist_temp to dist
rm -rf $BUILD_DIR
mv dist_temp $BUILD_DIR
