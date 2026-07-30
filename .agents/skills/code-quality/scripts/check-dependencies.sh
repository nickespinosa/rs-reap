#!/usr/bin/env bash

set -u

missing=0
found_manifest=0

check_command() {
    local command_name=$1

    if command -v "$command_name" >/dev/null 2>&1; then
        printf 'OK      command: %s (%s)\n' "$command_name" "$(command -v "$command_name")"
    else
        printf 'MISSING command: %s\n' "$command_name" >&2
        missing=1
    fi
}

check_manifest() {
    local manifest=$1
    local command_name=$2

    if [[ -f "$manifest" ]]; then
        found_manifest=1
        printf 'FOUND   manifest: %s\n' "$manifest"
        check_command "$command_name"
    fi
}

check_manifest Cargo.toml cargo
check_manifest go.mod go
check_manifest package.json npm
check_manifest pyproject.toml python3

if [[ -f Cargo.toml ]]; then
    check_command rustc
    check_command rustfmt
    check_command make

    if cargo clippy --version >/dev/null 2>&1; then
        printf 'OK      component: clippy\n'
    else
        printf 'MISSING component: clippy\n' >&2
        missing=1
    fi
fi

if ((found_manifest == 0)); then
    printf 'MISSING manifest: no supported dependency manifest found\n' >&2
    missing=1
fi

if ((missing != 0)); then
    printf 'Dependency readiness: BLOCKED\n' >&2
    exit 1
fi

printf 'Dependency readiness: READY\n'
