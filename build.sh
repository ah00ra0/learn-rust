#!/usr/bin/env bash
# ساخت کتاب در پوشه‌ی docs (برای GitHub Pages) + sitemap.xml و robots.txt
set -euo pipefail
cd "$(dirname "$0")"

SITE="https://ah00ra0.github.io/learn-rust"

mdbook build
touch docs/.nojekyll

TODAY=$(date +%F)
{
  echo '<?xml version="1.0" encoding="UTF-8"?>'
  echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">'
  echo "  <url><loc>$SITE/</loc><lastmod>$TODAY</lastmod><priority>1.0</priority></url>"
  for f in docs/*.html; do
    name=$(basename "$f")
    case "$name" in
      index.html|intro.html|print.html|404.html|toc.html) continue ;;
    esac
    echo "  <url><loc>$SITE/$name</loc><lastmod>$TODAY</lastmod><priority>0.8</priority></url>"
  done
  echo '</urlset>'
} > docs/sitemap.xml

cat > docs/robots.txt <<EOF
User-agent: *
Allow: /

Sitemap: $SITE/sitemap.xml
EOF

echo "OK: $(grep -c '<loc>' docs/sitemap.xml) صفحه در sitemap"
