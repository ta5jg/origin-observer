# -----------------------------------------------------------------------------
# Project : Origin Observer
# File    : scripts/test.sh
# Purpose : Run all workspace tests.
# Author  : İrfan Gedik
# Year    : 2026
# -----------------------------------------------------------------------------

set -eu
cargo test --workspace "$@"
