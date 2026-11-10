#!/usr/bin/env bash
# ساخت کتاب چندزبانه.
#   ./build.sh            ساخت همه‌ی زبان‌ها + صفحه‌ی انتخاب زبان + sitemap + robots
#   ./build.sh test       اجرای mdbook test روی همه‌ی زبان‌ها
#   ./build.sh test fa    اجرای تست فقط یک زبان
#   ./build.sh build fa   ساخت فقط یک زبان (بدون sitemap)
set -euo pipefail
cd "$(dirname "$0")"

SITE="https://ah00ra0.github.io/learn-rust"
REPO="https://github.com/ah00ra0/learn-rust"
ORDER=(en fa es fr it ar)   # ترتیب نمایش در صفحه‌ی انتخاب زبان

active_langs() {
  for l in "${ORDER[@]}"; do
    [ -f "books/$l/src/SUMMARY.md" ] && echo "$l"
  done
}

# تولید book.toml و theme برای یک زبان از روی meta.conf
prepare() {
  local l="$1"
  # shellcheck disable=SC1090
  source "books/$l/meta.conf"
  mkdir -p "books/$l/theme"
  cp theme/base.css "books/$l/theme/base.css"
  local css='"theme/base.css"'
  if [ "$DIR" = "rtl" ]; then
    cp theme/rtl.css "books/$l/theme/rtl.css"
    css='"theme/base.css", "theme/rtl.css"'
  fi

  cat > "books/$l/book.toml" <<EOF
[book]
title = "$TITLE"
description = "$DESC"
authors = ["ah00ra0"]
language = "$l"
text-direction = "$DIR"

[rust]
edition = "2024"

[build]
build-dir = "../../docs/$l"

[output.html]
site-url = "/learn-rust/$l/"
git-repository-url = "$REPO"
edit-url-template = "$REPO/edit/main/books/$l/src/{path}"
additional-css = [$css]
default-theme = "light"
preferred-dark-theme = "navy"
smart-punctuation = false

[output.html.playground]
editable = false
copyable = true
runnable = true

[output.html.search]
enable = true
limit-results = 20
EOF

  local alt=""
  for o in $(active_langs); do
    alt+="<link rel=\"alternate\" hreflang=\"$o\" href=\"$SITE/$o/\">"$'\n'
  done
  [ -f books/en/src/SUMMARY.md ] && alt+="<link rel=\"alternate\" hreflang=\"x-default\" href=\"$SITE/en/\">"$'\n'

  cat > "books/$l/theme/head.hbs" <<EOF
<meta name="description" content="$DESC">
<meta name="keywords" content="$KEYWORDS">
<meta name="robots" content="index, follow, max-snippet:-1, max-image-preview:large">
<meta property="og:type" content="website">
<meta property="og:locale" content="$LOCALE">
<meta property="og:site_name" content="$SITENAME">
<meta property="og:title" content="$TITLE">
<meta property="og:description" content="$DESC">
<meta name="twitter:card" content="summary">
<meta name="twitter:title" content="$TITLE">
<meta name="twitter:description" content="$DESC">
$alt<script type="application/ld+json">
{
  "@context": "https://schema.org",
  "@type": "Course",
  "name": "$TITLE",
  "description": "$DESC",
  "inLanguage": "$l",
  "isAccessibleForFree": true,
  "url": "$SITE/$l/",
  "provider": { "@type": "Person", "name": "ah00ra0", "url": "https://github.com/ah00ra0" }
}
</script>
EOF
}

build_one() {
  local l="$1"
  prepare "$l"
  mdbook build "books/$l" 2>&1 | grep -vE "^ INFO" || true
  touch "docs/$l/.nojekyll"
}

test_one() {
  local l="$1"
  prepare "$l"
  echo "== mdbook test: $l"
  mdbook test "books/$l" 2>&1 | grep -vE "^ INFO" || true
  return "${PIPESTATUS[0]}"
}

landing_page() {
  local items="" l
  for l in $(active_langs); do
    # shellcheck disable=SC1090
    source "books/$l/meta.conf"
    items+="    <li><a href=\"$l/\" hreflang=\"$l\" lang=\"$l\" dir=\"$DIR\"><strong>$NAME</strong><span>$TITLE</span></a></li>"$'\n'
  done
  local alt=""
  for l in $(active_langs); do alt+="<link rel=\"alternate\" hreflang=\"$l\" href=\"$SITE/$l/\">"$'\n'; done
  alt+="<link rel=\"alternate\" hreflang=\"x-default\" href=\"$SITE/\">"

  cat > docs/index.html <<EOF
<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Learn Rust — Free Complete Course in 6 Languages | آموزش Rust | Aprende Rust</title>
<meta name="description" content="Free, open-source Rust course in English, Persian (فارسی), Spanish, French, Italian and Arabic. Learn ownership, traits, lifetimes, concurrency, async and build real projects.">
<link rel="canonical" href="$SITE/">
$alt
<style>
  :root { --bg:#fff; --fg:#1d1d1f; --card:#f6f3f0; --accent:#b7410e; --muted:#666; }
  @media (prefers-color-scheme: dark) { :root { --bg:#16161a; --fg:#eee; --card:#23232a; --accent:#e8895a; --muted:#aaa; } }
  body { margin:0; background:var(--bg); color:var(--fg); font:17px/1.6 system-ui,-apple-system,"Segoe UI",Roboto,Tahoma,sans-serif; }
  main { max-width:720px; margin:0 auto; padding:48px 16px; }
  h1 { font-size:2rem; margin:0 0 .25em; }
  p.lead { color:var(--muted); margin-top:0; }
  ul { list-style:none; padding:0; display:grid; gap:12px; margin:32px 0; }
  a { display:block; background:var(--card); border-radius:12px; padding:16px 20px; text-decoration:none; color:inherit; border:1px solid transparent; }
  a:hover, a:focus-visible { border-color:var(--accent); }
  a strong { display:block; color:var(--accent); font-size:1.15rem; }
  a span { color:var(--muted); font-size:.95rem; }
  footer { color:var(--muted); font-size:.9rem; }
  footer a { display:inline; background:none; padding:0; color:var(--accent); }
</style>
</head>
<body>
<main>
  <h1>🦀 Learn Rust</h1>
  <p class="lead">A free, complete, open-source Rust course. Choose your language.</p>
  <ul>
$items  </ul>
  <footer>Open source on <a href="$REPO">GitHub</a>. Every code example is compiled and tested.</footer>
</main>
</body>
</html>
EOF
}

sitemap_and_robots() {
  local today l f name o
  today=$(date +%F)
  {
    echo '<?xml version="1.0" encoding="UTF-8"?>'
    echo '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">'
    echo "  <url><loc>$SITE/</loc><lastmod>$today</lastmod><priority>1.0</priority></url>"
    for l in $(active_langs); do
      for f in docs/$l/*.html; do
        name=$(basename "$f")
        case "$name" in
          intro.html|print.html|404.html|toc.html) continue ;;
        esac
        local url="$SITE/$l/"
        [ "$name" != "index.html" ] && url="$SITE/$l/$name"
        echo "  <url>"
        echo "    <loc>$url</loc><lastmod>$today</lastmod><priority>$([ "$name" = index.html ] && echo 0.9 || echo 0.7)</priority>"
        for o in $(active_langs); do
          [ -f "docs/$o/$name" ] || continue
          local ourl="$SITE/$o/"
          [ "$name" != "index.html" ] && ourl="$SITE/$o/$name"
          echo "    <xhtml:link rel=\"alternate\" hreflang=\"$o\" href=\"$ourl\"/>"
        done
        echo "  </url>"
      done
    done
    echo '</urlset>'
  } > docs/sitemap.xml

  cat > docs/robots.txt <<EOF
User-agent: *
Allow: /

Sitemap: $SITE/sitemap.xml
EOF
  touch docs/.nojekyll
  echo "OK: $(grep -c '<loc>' docs/sitemap.xml) آدرس در sitemap"
}

cmd="${1:-all}"
case "$cmd" in
  test)
    rc=0
    if [ -n "${2:-}" ]; then langs="$2"; else langs=$(active_langs); fi
    for l in $langs; do test_one "$l" || rc=1; done
    exit $rc
    ;;
  build)
    build_one "${2:?زبان را بدهید}"
    ;;
  all)
    rm -rf docs
    mkdir -p docs
    for l in $(active_langs); do build_one "$l"; done
    landing_page
    sitemap_and_robots
    ;;
  *) echo "usage: $0 [all|test [lang]|build lang]"; exit 2 ;;
esac
