#!/bin/sh
# Trunk post_build hook: turns each line of redirects.txt into a static
# <slug>/index.html that bounces to its URL. GitHub Pages can't do
# server-side redirects, so this is as close as a static host gets.
set -eu

src="$TRUNK_SOURCE_DIR/redirects.txt"
out="$TRUNK_STAGING_DIR"

lineno=0
fail() {
    echo "redirects.txt:$lineno: $1" >&2
    exit 1
}

while IFS= read -r line || [ -n "$line" ]; do
    lineno=$((lineno + 1))

    set -f
    # shellcheck disable=SC2086 # intentional word splitting
    set -- $line
    set +f

    [ $# -eq 0 ] && continue
    case "$1" in \#*) continue ;; esac
    [ $# -eq 2 ] || fail "expected '<slug> <url>'"
    slug=$1
    url=$2

    case "$slug" in
        *[!A-Za-z0-9_-]*) fail "slug '$slug' may only use letters, digits, '-' and '_'" ;;
    esac
    case "$url" in
        http://* | https://*) ;;
        *) fail "url '$url' must start with http:// or https://" ;;
    esac
    # Whitelist URL characters so the URL can't break out of the HTML/JS below.
    case "$url" in
        *[!A-Za-z0-9._~:/?#@!\$\&\(\)*+,\;=%-]*) fail "url '$url' contains a disallowed character" ;;
    esac
    [ -e "$out/$slug" ] && fail "slug '$slug' is a duplicate or collides with a built file"

    attr_url=$(printf '%s' "$url" | sed 's/&/\&amp;/g')

    mkdir "$out/$slug"
    cat >"$out/$slug/index.html" <<EOF
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Redirecting…</title>
    <meta name="robots" content="noindex" />
    <link rel="canonical" href="$attr_url" />
    <meta http-equiv="refresh" content="0; url=$attr_url" />
    <script>location.replace("$url");</script>
  </head>
  <body>
    <a href="$attr_url">Continue to $attr_url</a>
  </body>
</html>
EOF
done <"$src"
