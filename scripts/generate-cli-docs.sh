#!/usr/bin/env sh
# Generates the CLI reference page by running `yolk generate-markdown-help`.
# If cargo isn't available (or the command fails), we just warn and skip it —
# the site still builds, only the CLI Reference page will be missing.
set -eu

OUT="site/book/cli_reference.md"

if ! command -v cargo >/dev/null 2>&1; then
  echo "warning: cargo not found — skipping the generated CLI Reference page." >&2
  exit 0
fi

if ! markdown=$(cargo run --quiet -- generate-markdown-help); then
  echo "warning: 'cargo run -- generate-markdown-help' failed — skipping the CLI Reference page." >&2
  exit 0
fi

# Strip the stray "↴" glyphs clap-markdown adds to the command-overview links.
printf '%s\n' "$markdown" | sed 's/↴//g' > "$OUT"
echo "Generated $OUT"
