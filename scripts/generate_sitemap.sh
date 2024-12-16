#!/bin/bash

# Required environment variables
DOMAIN=${DOMAIN:-"https://konnektoren.help"}
BUILD_DIR=${BUILD_DIR:-"dist"}

# Other constants
CURRENT_DATE=$(date +%Y-%m-%d)
CURRENT_DATE_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
PAGES=("/" "map" "about" "challenges" "leaderboard" "profile" "marketplace")
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
LANGS=("ar" "cn" "de" "en" "es" "pl" "tr" "ua")

# Create build directory if it doesn't exist
mkdir -p "$BUILD_DIR"

# Function to generate title from URL
get_title() {
    local page=$1
    if [ "$page" = "/" ]; then
        echo "Home"
    else
        echo "$page" | sed 's/-/ /g;s/\// - /g' | awk '{for(i=1;i<=NF;i++){ $i=toupper(substr($i,1,1)) substr($i,2) }}1'
    fi
}

# Function to escape XML special characters
xml_escape() {
    echo "$1" | sed 's/&/\&amp;/g; s/</\&lt;/g; s/>/\&gt;/g; s/"/\&quot;/g; s/'"'"'/\&#39;/g'
}

DOMAIN_NAME=$(echo $DOMAIN | sed 's/https\?:\/\///')

# Create the header for the sitemap.xml file
cat <<EOF > "$BUILD_DIR/sitemap.xml"
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
EOF

# Create the header for the atom.xml file
cat <<EOF > "$BUILD_DIR/atom.xml"
<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>${DOMAIN_NAME}</title>
  <link href="${DOMAIN}/atom.xml" rel="self"/>
  <link href="${DOMAIN}"/>
  <updated>${CURRENT_DATE_TIME}</updated>
  <id>${DOMAIN}/</id>
  <author>
    <name>${DOMAIN_NAME}</name>
  </author>
EOF

# Create sitemap.txt file
> "$BUILD_DIR/sitemap.txt"

# Loop through each page and generate the corresponding sitemap and atom entries
for PAGE in "${PAGES[@]}"; do
    if [ "$PAGE" = "/" ]; then
        URL_SUFFIX="/"
    else
        URL_SUFFIX="/${PAGE}/"
    fi
    ESCAPED_URL_SUFFIX=$(xml_escape "$URL_SUFFIX")
    TITLE=$(get_title "$PAGE")

    # Sitemap entry
    cat <<EOF >> "$BUILD_DIR/sitemap.xml"
  <url>
    <loc>${DOMAIN}${ESCAPED_URL_SUFFIX}</loc>
    <lastmod>${CURRENT_DATE}</lastmod>
    <xhtml:link rel="alternate" hreflang="en" href="${DOMAIN}${ESCAPED_URL_SUFFIX}"/>
EOF

    echo "${DOMAIN}${URL_SUFFIX}" >> "$BUILD_DIR/sitemap.txt"

    for LANG in "${LANGS[@]}"; do
        LANG_SUFFIX="${URL_SUFFIX}?lang=${LANG}"
        ESCAPED_LANG_SUFFIX=$(xml_escape "$LANG_SUFFIX")
        cat <<EOF >> "$BUILD_DIR/sitemap.xml"
    <xhtml:link rel="alternate" hreflang="${LANG}" href="${DOMAIN}${ESCAPED_LANG_SUFFIX}"/>
EOF

        echo "${DOMAIN}${LANG_SUFFIX}" >> "$BUILD_DIR/sitemap.txt"
    done

    echo "  </url>" >> "$BUILD_DIR/sitemap.xml"

    # Atom entry
    cat <<EOF >> "$BUILD_DIR/atom.xml"
  <entry>
    <title>${TITLE}</title>
    <link href="${DOMAIN}${ESCAPED_URL_SUFFIX}"/>
    <id>${DOMAIN}${ESCAPED_URL_SUFFIX}</id>
    <updated>${CURRENT_DATE_TIME}</updated>
    <summary>Learn about ${TITLE} on ${DOMAIN_NAME}</summary>
  </entry>
EOF
done

# Add challenge pages
for CHALLENGE in "${CHALLENGES[@]}"; do
    URL_SUFFIX="/challenge/${CHALLENGE}/"
    ESCAPED_URL_SUFFIX=$(xml_escape "$URL_SUFFIX")
    TITLE=$(get_title "Challenge ${CHALLENGE}")

    # Sitemap entry
    cat <<EOF >> "$BUILD_DIR/sitemap.xml"
  <url>
    <loc>${DOMAIN}${ESCAPED_URL_SUFFIX}</loc>
    <lastmod>${CURRENT_DATE}</lastmod>
    <xhtml:link rel="alternate" hreflang="en" href="${DOMAIN}${ESCAPED_URL_SUFFIX}"/>
EOF

    echo "${DOMAIN}${URL_SUFFIX}" >> "$BUILD_DIR/sitemap.txt"

    for LANG in "${LANGS[@]}"; do
        LANG_SUFFIX="${URL_SUFFIX}?lang=${LANG}"
        ESCAPED_LANG_SUFFIX=$(xml_escape "$LANG_SUFFIX")
        cat <<EOF >> "$BUILD_DIR/sitemap.xml"
    <xhtml:link rel="alternate" hreflang="${LANG}" href="${DOMAIN}${ESCAPED_LANG_SUFFIX}"/>
EOF

        echo "${DOMAIN}${LANG_SUFFIX}" >> "$BUILD_DIR/sitemap.txt"
    done

    echo "  </url>" >> "$BUILD_DIR/sitemap.xml"

    # Atom entry
    cat <<EOF >> "$BUILD_DIR/atom.xml"
  <entry>
    <title>${TITLE}</title>
    <link href="${DOMAIN}${ESCAPED_URL_SUFFIX}"/>
    <id>${DOMAIN}${ESCAPED_URL_SUFFIX}</id>
    <updated>${CURRENT_DATE_TIME}</updated>
    <summary>Learn about ${TITLE} on ${DOMAIN_NAME}</summary>
  </entry>
EOF
done

# Close the urlset tag in sitemap.xml
echo '</urlset>' >> "$BUILD_DIR/sitemap.xml"

# Close the feed tag in atom.xml
echo '</feed>' >> "$BUILD_DIR/atom.xml"

echo "Generated sitemap.xml, atom.xml, and sitemap.txt in $BUILD_DIR for domain $DOMAIN"
