#!/usr/bin/env bash
# -----------------------------------------------------------------------------
# Project : Origin Observer
# File    : origin_observer_finalize_v1.sh
# Purpose : Complete and freeze the permanent Origin Observer v1.0 filesystem.
# Author  : İrfan Gedik
# Year    : 2026
# -----------------------------------------------------------------------------

set -Eeuo pipefail
IFS=$'\n\t'

readonly SCRIPT_VERSION="1.0.0"
readonly PROJECT_NAME="Origin Observer"
readonly EXPECTED_MANIFEST="Cargo.toml"

PROJECT_ROOT="${1:-$(pwd)}"
RUN_TESTS="${RUN_TESTS:-1}"

log() {
    printf '[Origin Observer] %s\n' "$*"
}

warn() {
    printf '[Origin Observer][WARNING] %s\n' "$*" >&2
}

die() {
    printf '[Origin Observer][ERROR] %s\n' "$*" >&2
    exit 1
}

usage() {
    cat <<'EOF'
Origin Observer Filesystem Finalizer v1.0

Usage:
  ./origin_observer_finalize_v1.sh [PROJECT_ROOT]

Examples:
  cd ~/Desktop/origin-observer
  ./origin_observer_finalize_v1.sh

  ./origin_observer_finalize_v1.sh ~/Desktop/origin-observer

Optional:
  RUN_TESTS=0 ./origin_observer_finalize_v1.sh ~/Desktop/origin-observer

Rules:
  - Existing files are never overwritten.
  - Existing Cargo manifests are never rewritten.
  - Only missing permanent directories and marker files are created.
  - The script validates the Rust workspace when Cargo is available.
EOF
}

case "${1:-}" in
    -h|--help)
        usage
        exit 0
        ;;
esac

cd "$PROJECT_ROOT" 2>/dev/null || die "Project directory cannot be opened: $PROJECT_ROOT"
PROJECT_ROOT="$(pwd -P)"

[[ -f "$EXPECTED_MANIFEST" ]] || die "Cargo.toml was not found in: $PROJECT_ROOT"
grep -q '^\[workspace\]' "$EXPECTED_MANIFEST" ||
    die "The root Cargo.toml is not a Rust workspace manifest."

log "Finalizing ${PROJECT_NAME} at: ${PROJECT_ROOT}"
log "Finalizer version: ${SCRIPT_VERSION}"

# -----------------------------------------------------------------------------
# Architectural decision
# -----------------------------------------------------------------------------
#
# Origin Observer is a virtual Cargo workspace.
#
# Therefore:
#   - A root /src directory is NOT required.
#   - Executable code belongs in crates/oo-cli/src/main.rs.
#   - Library code belongs in crates/<crate-name>/src/lib.rs.
#   - Creating a second root executable or an /apps tree would duplicate
#     responsibilities and slow development.
#
# This finalizer deliberately preserves the shortest path to working Rust code.
# -----------------------------------------------------------------------------

readonly REQUIRED_CRATES=(
    oo-abi
    oo-bytecode
    oo-cache
    oo-cli
    oo-confidence
    oo-config
    oo-core
    oo-dataset
    oo-descriptor
    oo-discovery
    oo-evidence
    oo-experiment
    oo-history
    oo-model
    oo-observer
    oo-provider
    oo-proxy
    oo-report
    oo-rpc
    oo-snapshot
    oo-storage
    oo-test-support
    oo-utils
    oo-wallet
)

readonly REQUIRED_DIRS=(
    assets
    assets/diagrams
    assets/icons
    assets/images
    assets/logos
    config
    data
    datasets
    docs
    evidence
    experiments
    fixtures
    hypotheses
    reports
    research
    scripts
    snapshots
    templates
    tests
    test-data
    tools
    research/case-studies
    research/experiments
    research/hypotheses
    research/journal
    research/laws
    research/questions
    research/unknowns
    templates/dataset
    templates/evidence
    templates/experiment
    templates/report
    test-data/abis
    test-data/bytecodes
    test-data/rpc
    test-data/snapshots
    test-data/wallets
)

create_directory() {
    local directory="$1"

    if [[ -d "$directory" ]]; then
        log "keep directory: $directory"
        return
    fi

    mkdir -p "$directory"
    log "create directory: $directory"
}

create_marker() {
    local directory="$1"
    local marker="${directory}/.gitkeep"

    if [[ -e "$marker" ]]; then
        return
    fi

    cat >"$marker" <<EOF
# -----------------------------------------------------------------------------
# Project : Origin Observer
# File    : ${marker}
# Purpose : Preserve this permanent project directory in version control.
# Author  : İrfan Gedik
# Year    : 2026
# -----------------------------------------------------------------------------
EOF

    log "create marker: $marker"
}

ensure_crate() {
    local crate="$1"
    local crate_dir="crates/${crate}"
    local manifest="${crate_dir}/Cargo.toml"
    local source_dir="${crate_dir}/src"

    [[ -d "$crate_dir" ]] ||
        die "Required crate directory is missing: $crate_dir"

    [[ -f "$manifest" ]] ||
        die "Required crate manifest is missing: $manifest"

    create_directory "$source_dir"

    if [[ "$crate" == "oo-cli" ]]; then
        [[ -f "${source_dir}/main.rs" ]] ||
            die "CLI entry point is missing: ${source_dir}/main.rs"
    else
        [[ -f "${source_dir}/lib.rs" ]] ||
            die "Library entry point is missing: ${source_dir}/lib.rs"
    fi

    create_directory "${crate_dir}/tests"
    create_marker "${crate_dir}/tests"
}

validate_workspace_members() {
    local crate

    for crate in "${REQUIRED_CRATES[@]}"; do
        if ! grep -Eq "\"crates/${crate}\"|'crates/${crate}'" Cargo.toml; then
            die "Cargo workspace member is missing from Cargo.toml: crates/${crate}"
        fi
    done
}

validate_permanent_files() {
    local required_files=(
        Cargo.toml
        README.md
        ROADMAP.md
        WDRP.md
        rustfmt.toml
        clippy.toml
        .gitignore
        config/default.toml
    )

    local file
    for file in "${required_files[@]}"; do
        [[ -f "$file" ]] || die "Required permanent file is missing: $file"
    done
}

validate_no_duplicate_application_root() {
    if [[ -d "src" ]]; then
        warn "A root src/ directory exists."
        warn "It is not required by this virtual workspace and may duplicate oo-cli."
        warn "This script will not delete or modify it."
    fi

    if [[ -d "apps" ]]; then
        warn "An apps/ directory exists."
        warn "It is not required for Origin Observer v1.0 and will not be modified."
    fi
}

run_cargo_validation() {
    if ! command -v cargo >/dev/null 2>&1; then
        warn "Cargo is unavailable; Rust validation was skipped."
        return
    fi

    log "Running cargo metadata..."
    cargo metadata --no-deps --format-version 1 >/dev/null

    log "Running cargo check..."
    cargo check --workspace

    if [[ "$RUN_TESTS" == "1" ]]; then
        log "Running cargo test..."
        cargo test --workspace
    else
        warn "RUN_TESTS=0: cargo test was skipped."
    fi
}

write_freeze_record() {
    local record=".origin-observer-layout-v1"

    if [[ -f "$record" ]]; then
        log "keep freeze record: $record"
        return
    fi

    cat >"$record" <<'EOF'
# -----------------------------------------------------------------------------
# Project : Origin Observer
# File    : .origin-observer-layout-v1
# Purpose : Mark the permanent Origin Observer filesystem architecture as frozen.
# Author  : İrfan Gedik
# Year    : 2026
# -----------------------------------------------------------------------------
layout_version=1.0.0
status=frozen
architecture=virtual-cargo-workspace
application_entry=crates/oo-cli/src/main.rs
library_root=crates
EOF

    log "create freeze record: $record"
}

main() {
    local directory
    local crate

    validate_permanent_files
    validate_workspace_members
    validate_no_duplicate_application_root

    for directory in "${REQUIRED_DIRS[@]}"; do
        create_directory "$directory"
        create_marker "$directory"
    done

    for crate in "${REQUIRED_CRATES[@]}"; do
        ensure_crate "$crate"
    done

    write_freeze_record
    run_cargo_validation

    cat <<EOF

===============================================================================
Origin Observer filesystem v1.0 is complete and frozen.
===============================================================================

Project root:
  ${PROJECT_ROOT}

Canonical application entry:
  crates/oo-cli/src/main.rs

Canonical Rust libraries:
  crates/<crate-name>/src/lib.rs

Important:
  The workspace root intentionally has no required src/ directory.
  We can now stop editing the filesystem and begin real Rust implementation.

Next implementation target:
  crates/oo-model
===============================================================================
EOF
}

main
