#!/usr/bin/env bash
set -euo pipefail

# validate-with-genbo.sh — Validate .genexpr patches and .gendsp fixtures
# using Max's own genbo parser via Max's bundled Node.js.
#
# This is a machine-checkable conformance ring: it validates GenExpr syntax
# using the same parser that Max/RNBO uses internally. No rendering, no listening.
#
# When Max.app is not installed, the script exits 0 with a warning.
# When installed, it runs tools/validate_gendsp.js over:
#   (a) conformance/patches/*.genexpr
#   (b) crates/opengen-gendsp/tests/fixtures/*.gendsp

# ─── Locate Max.app ────────────────────────────────────────────────────────────
MAX_APP=""
for candidate in "/Applications/Max.app" "$HOME/Applications/Max.app"; do
    if [ -d "$candidate" ]; then
        MAX_APP="$candidate"
        break
    fi
done

if [ -z "$MAX_APP" ]; then
    echo "WARN: Max.app not found — skipping genbo validation"
    exit 0
fi

# ─── Locate Max's bundled Node.js ──────────────────────────────────────────────
NODE_BIN="$MAX_APP/Contents/Resources/C74/packages/Node for Max/source/bin/osx/node/node"
GENBO_JS="$MAX_APP/Contents/Resources/C74/packages/RNBO/server/node_modules/@rnbo/genexpr_js/genbo.js"

if [ ! -x "$NODE_BIN" ]; then
    echo "WARN: Max's bundled node not found at $NODE_BIN — skipping genbo validation"
    exit 0
fi

if [ ! -f "$GENBO_JS" ]; then
    echo "WARN: genbo not found at $GENBO_JS — skipping genbo validation"
    exit 0
fi

# ─── Resolve repo root (script lives in tools/) ────────────────────────────────
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Max.app:       $MAX_APP"
echo "Node:           $NODE_BIN"
echo "genbo:          $GENBO_JS"
echo "Repository:     $REPO_ROOT"
echo ""

# ─── Run validation ────────────────────────────────────────────────────────────
cd "$REPO_ROOT"
"$NODE_BIN" "$REPO_ROOT/tools/validate_gendsp.js"
