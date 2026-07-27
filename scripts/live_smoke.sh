# -----------------------------------------------------------------------------
# Project : Origin Observer
# File    : scripts/live_smoke.sh
# Purpose : Run an opt-in live JSON-RPC smoke test.
# Author  : İrfan Gedik
# Year    : 2026
# -----------------------------------------------------------------------------

set -eu

OUT_DIR="${OO_LIVE_SMOKE_OUT:-/tmp/origin-observer-live-smoke}"
RPC_A="${OO_LIVE_RPC_A:-https://cloudflare-eth.com}"
RPC_B="${OO_LIVE_RPC_B:-https://cloudflare-eth.com}"

cargo run -p oo-cli -- observe \
  --strategy chain-id \
  --provider "provider-a=${RPC_A}" \
  --provider "provider-b=${RPC_B}" \
  --format report-json \
  --out "${OUT_DIR}"
