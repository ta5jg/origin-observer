# -----------------------------------------------------------------------------
# Project : Origin Observer
# File    : scripts/check.sh
# Purpose : Run formatting, linting and workspace tests.
# Author  : İrfan Gedik
# Year    : 2026
# -----------------------------------------------------------------------------

set -eu
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
