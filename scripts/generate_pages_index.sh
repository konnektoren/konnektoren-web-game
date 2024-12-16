# generate_pages_index.sh
PAGES=("map/" "about/" "achievements/" "challenges/" "challenge/" "leaderboard/" "profile/" "marketplace/" "backup/")
CHALLENGES=(
    "articles-1" "articles-2" "articles-3" "articles-4" "articles-5" "articles-6"
    "reflexivpronoun-1" "reflexivpronoun-2"
    "personal_pronouns-1" "personal_pronouns-2"
    "konnektoren-1" "konnektoren-2" "konnektoren-3" "konnektoren-4" "konnektoren-5"
    "custom-verbs-1"
    "custom-perfect_tense-1"
    "custom-group-of-nouns"
    "custom-negation"
    "custom_haupt-neben-satze_exercise"
    "verbs_mit_dativ_akkusativ"
    "custom-konjunktiv2"
    "custom-zeitangaben"
    "custom-casus"
    "custom_past_tenses"
    "custom_verbs_prepositions"
    "custom_interrogative_particles"
    "custom-articles-1"
)
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

for challenge in "${CHALLENGES[@]}"; do
    mkdir -p "dist_temp/challenge/${challenge}"
    cp dist_temp/index.html "dist_temp/challenge/${challenge}/index.html"
done

# Remove old dist folder and rename dist_temp to dist
rm -rf $BUILD_DIR
mv dist_temp $BUILD_DIR
