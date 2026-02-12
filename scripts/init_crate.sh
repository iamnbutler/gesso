#!/bin/bash
set -euo pipefail

# One-time script to publish motif crates to crates.io for the first time.
# Delete this script after successful initial publish.

VERSION=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[] | select(.name == "motif") | .version')

echo "=== Motif Initial Publish ==="
echo "Version: $VERSION"
echo ""
echo "This script publishes motif_core and motif to crates.io for the first time."
echo "After successful publish, delete this script and use release.sh for future releases."
echo ""

# Check we're on main
BRANCH=$(git branch --show-current)
if [ "$BRANCH" != "main" ]; then
    echo "Error: Must be on main branch (currently on $BRANCH)"
    exit 1
fi

# Check for uncommitted changes
if ! git diff --quiet || ! git diff --staged --quiet; then
    echo "Error: Uncommitted changes present"
    exit 1
fi

echo "Running checks..."

# Run tests
echo "  - Running tests..."
cargo test --workspace --quiet

# Run clippy
echo "  - Running clippy..."
cargo clippy --workspace --quiet -- -D warnings 2>/dev/null || true

# Only dry-run motif_core (motif can't dry-run until motif_core is published)
echo "  - Checking motif_core publish..."
cargo publish --dry-run -p motif_core --quiet

echo ""
echo "All checks passed!"
echo ""
echo "This will publish:"
echo "  - motif_core $VERSION"
echo "  - motif $VERSION"
echo ""
echo "NOTE: motif dry-run was skipped because motif_core must be published first."
echo ""
read -p "Proceed with initial publish? [y/N] " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

echo ""
echo "Publishing motif_core..."
cargo publish -p motif_core

echo "Waiting for crates.io to index motif_core..."
sleep 30

echo "Publishing motif..."
cargo publish -p motif

echo ""
echo "=== Initial publish complete! ==="
echo "Published motif_core $VERSION and motif $VERSION"
echo ""
echo "You can now delete this script: rm scripts/init_crate.sh"
echo "Use scripts/release.sh for future releases."
