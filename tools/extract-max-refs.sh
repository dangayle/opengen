#!/usr/bin/env bash
# Extract reference material from a licensed Max install into ./reference/ (gitignored).
# EULA-tagged items are reference-only: never commit, never quote verbatim.
set -euo pipefail
MAX_RES="${MAX_RES:-/Applications/Max.app/Contents/Resources}"
DEST_ROOT="$(cd "$(dirname "$0")/.." && pwd)/reference"
MAX_VERSION=$(defaults read /Applications/Max.app/Contents/Info.plist CFBundleShortVersionString)

[ -d "$MAX_RES" ] || { echo "Max not found at $MAX_RES (set MAX_RES=...)"; exit 1; }
mkdir -p "$DEST_ROOT"
echo "max_version: $MAX_VERSION" > "$DEST_ROOT/EXTRACTED.txt"
echo "extracted: $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$DEST_ROOT/EXTRACTED.txt"

while IFS=$'\t' read -r license src dest; do
  [[ "$license" =~ ^#.*$ || -z "$license" ]] && continue
  echo "[$license] $src -> reference/$dest"
  mkdir -p "$DEST_ROOT/$dest"
  rsync -a --exclude node_modules "$MAX_RES/$src/" "$DEST_ROOT/$dest/"
  echo "$license	$src	$dest" >> "$DEST_ROOT/EXTRACTED.txt"
done < "$(dirname "$0")/max-refs.manifest"
echo "Done. reference/ is gitignored; EULA items are read-only reference."
