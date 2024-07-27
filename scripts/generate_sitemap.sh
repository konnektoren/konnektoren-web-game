#!/bin/bash

DOMAIN="https://konnektoren.help"
CURRENT_DATE=$(date +%Y-%m-%d)
PAGES=("/" "/map" "/about" "/challenge/:id" "/leaderboard" "/profile" "/results/:id" "/challenge/articles-1"
 "/challenge/reflexivpronoun-1" "/challenge/personal_pronouns-1" "/challenge/konnektoren-1")
LANGS=("ar" "cn" "de" "en" "es" "pl" "tr" "ua")

# Create the header for the sitemap.xml file
cat <<EOF > sitemap.xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
EOF

# Loop through each page and generate the corresponding sitemap entries
for PAGE in "${PAGES[@]}"; do
  cat <<EOF >> sitemap.xml
  <url>
    <loc>${DOMAIN}${PAGE}</loc>
    <lastmod>${CURRENT_DATE}</lastmod>
    <xhtml:link rel="alternate" hreflang="en" href="${DOMAIN}${PAGE}"/>
EOF

  for LANG in "${LANGS[@]}"; do
    cat <<EOF >> sitemap.xml
    <xhtml:link rel="alternate" hreflang="${LANG}" href="${DOMAIN}${PAGE}?lang=${LANG}"/>
EOF
  done

  echo "  </url>" >> sitemap.xml
done

# Close the urlset tag
echo '</urlset>' >> sitemap.xml
