# -----------------------------------------------------------------------------
# Project : Origin Observer
# File    : scripts/run.sh
# Purpose : Run the Origin Observer command-line interface.
# Author  : İrfan Gedik
# Year    : 2026
# -----------------------------------------------------------------------------

set -eu
cargo run -p oo-cli -- "$@"
