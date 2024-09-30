#!/bin/bash

DOMAIN="https://konnektoren.help"
CURRENT_DATE=$(date +%Y-%m-%d)
CURRENT_DATE_TIME=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
PAGES=("/" "/map" "/about" "/challenges" "/leaderboard" "/profile" "/marketplace" "/challenge/articles-1"
 "/challenge/reflexivpronoun-1" "/challenge/personal_pronouns-1" "/challenge/konnektoren-1")
LANGS=("ar" "cn" "de" "en" "es" "pl" "tr" "ua")

# Function to generate title from URL
get_title() {
    local page=$1
    if [ "$page" = "/" ]; then
        echo "Home"
    else
        echo "$page" | sed 's/^\///;s/-/ /g;s/\// - /g' | awk '{for(i=1;i<=NF;i++){ $i=toupper(substr($i,1,1)) substr($i,2) }}1'
    fi
}

# Create the header for the sitemap.xml file
cat <<EOF > sitemap.xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
EOF

# Create the header for the atom.xml file
cat <<EOF > atom.xml
<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Konnektoren.help</title>
  <link href="${DOMAIN}/atom.xml" rel="self"/>
  <link href="${DOMAIN}"/>
  <updated>${CURRENT_DATE_TIME}</updated>
  <id>${DOMAIN}/</id>
  <author>
    <name>Konnektoren.help</name>
  </author>
EOF

> sitemap.txt

# Loop through each page and generate the corresponding sitemap and atom entries
for PAGE in "${PAGES[@]}"; do
  TITLE=$(get_title "$PAGE")

  # Sitemap entry
  cat <<EOF >> sitemap.xml
  <url>
    <loc>${DOMAIN}${PAGE}</loc>
    <lastmod>${CURRENT_DATE}</lastmod>
    <xhtml:link rel="alternate" hreflang="en" href="${DOMAIN}${PAGE}"/>
EOF

  echo "${DOMAIN}${PAGE}" >> sitemap.txt

  for LANG in "${LANGS[@]}"; do
    cat <<EOF >> sitemap.xml
    <xhtml:link rel="alternate" hreflang="${LANG}" href="${DOMAIN}${PAGE}?lang=${LANG}"/>
EOF

    echo "${DOMAIN}${PAGE}?lang=${LANG}" >> sitemap.txt
  done

  echo "  </url>" >> sitemap.xml

  # Atom entry
  cat <<EOF >> atom.xml
  <entry>
    <title>${TITLE}</title>
    <link href="${DOMAIN}${PAGE}"/>
    <id>${DOMAIN}${PAGE}</id>
    <updated>${CURRENT_DATE_TIME}</updated>
    <summary>Learn about ${TITLE} on Konnektoren.help</summary>
  </entry>
EOF
done

# Close the urlset tag in sitemap.xml
echo '</urlset>' >> sitemap.xml

# Close the feed tag in atom.xml
echo '</feed>' >> atom.xml
