#!/bin/bash

DOMAINS=("https://konnektoren.help" "https://konnektoren.app")
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

# Function to generate title from URL
get_title() {
    local page=$1
    if [ "$page" = "/" ]; then
        echo "Home"
    else
        echo "$page" | sed 's/-/ /g;s/\// - /g' | awk '{for(i=1;i<=NF;i++){ $i=toupper(substr($i,1,1)) substr($i,2) }}1'
    fi
}

for DOMAIN in "${DOMAINS[@]}"; do
    DOMAIN_NAME=$(echo $DOMAIN | sed 's/https:\/\///')

    # Create the header for the sitemap.xml file
    cat <<EOF > sitemap_${DOMAIN_NAME}.xml
<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
EOF

    # Create the header for the atom.xml file
    cat <<EOF > atom_${DOMAIN_NAME}.xml
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

    > sitemap_${DOMAIN_NAME}.txt

    # Loop through each page and generate the corresponding sitemap and atom entries
    for PAGE in "${PAGES[@]}"; do
      URL_SUFFIX=$([ "$PAGE" = "/" ] && echo "" || echo "?page=${PAGE}")
      TITLE=$(get_title "$PAGE")

      # Sitemap entry
      cat <<EOF >> sitemap_${DOMAIN_NAME}.xml
  <url>
    <loc>${DOMAIN}${URL_SUFFIX}</loc>
    <lastmod>${CURRENT_DATE}</lastmod>
    <xhtml:link rel="alternate" hreflang="en" href="${DOMAIN}${URL_SUFFIX}"/>
EOF

      echo "${DOMAIN}${URL_SUFFIX}" >> sitemap_${DOMAIN_NAME}.txt

      for LANG in "${LANGS[@]}"; do
        LANG_SUFFIX=$([ "$URL_SUFFIX" = "" ] && echo "?lang=${LANG}" || echo "${URL_SUFFIX}&lang=${LANG}")
        cat <<EOF >> sitemap_${DOMAIN_NAME}.xml
    <xhtml:link rel="alternate" hreflang="${LANG}" href="${DOMAIN}${LANG_SUFFIX}"/>
EOF

        echo "${DOMAIN}${LANG_SUFFIX}" >> sitemap_${DOMAIN_NAME}.txt
      done

      echo "  </url>" >> sitemap_${DOMAIN_NAME}.xml

      # Atom entry
      cat <<EOF >> atom_${DOMAIN_NAME}.xml
  <entry>
    <title>${TITLE}</title>
    <link href="${DOMAIN}${URL_SUFFIX}"/>
    <id>${DOMAIN}${URL_SUFFIX}</id>
    <updated>${CURRENT_DATE_TIME}</updated>
    <summary>Learn about ${TITLE} on ${DOMAIN_NAME}</summary>
  </entry>
EOF
    done

    # Add challenge pages
    for CHALLENGE in "${CHALLENGES[@]}"; do
      URL_SUFFIX="?page=challenge&id=${CHALLENGE}"
      TITLE=$(get_title "Challenge ${CHALLENGE}")

      # Sitemap entry
      cat <<EOF >> sitemap_${DOMAIN_NAME}.xml
  <url>
    <loc>${DOMAIN}${URL_SUFFIX}</loc>
    <lastmod>${CURRENT_DATE}</lastmod>
    <xhtml:link rel="alternate" hreflang="en" href="${DOMAIN}${URL_SUFFIX}"/>
EOF

      echo "${DOMAIN}${URL_SUFFIX}" >> sitemap_${DOMAIN_NAME}.txt

      for LANG in "${LANGS[@]}"; do
        LANG_SUFFIX="${URL_SUFFIX}&lang=${LANG}"
        cat <<EOF >> sitemap_${DOMAIN_NAME}.xml
    <xhtml:link rel="alternate" hreflang="${LANG}" href="${DOMAIN}${LANG_SUFFIX}"/>
EOF

        echo "${DOMAIN}${LANG_SUFFIX}" >> sitemap_${DOMAIN_NAME}.txt
      done

      echo "  </url>" >> sitemap_${DOMAIN_NAME}.xml

      # Atom entry
      cat <<EOF >> atom_${DOMAIN_NAME}.xml
  <entry>
    <title>${TITLE}</title>
    <link href="${DOMAIN}${URL_SUFFIX}"/>
    <id>${DOMAIN}${URL_SUFFIX}</id>
    <updated>${CURRENT_DATE_TIME}</updated>
    <summary>Learn about ${TITLE} on ${DOMAIN_NAME}</summary>
  </entry>
EOF
    done

    # Close the urlset tag in sitemap.xml
    echo '</urlset>' >> sitemap_${DOMAIN_NAME}.xml

    # Close the feed tag in atom.xml
    echo '</feed>' >> atom_${DOMAIN_NAME}.xml
done
