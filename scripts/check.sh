#!/usr/bin/env bash
# Non-mutating Rust quality checks for ordinary Cargo projects.
set -Eeuo pipefail

usage() {
  cat <<'USAGE'
Usage: check.sh [CARGO_PROJECT_DIRECTORY] [--skip-clippy] [--skip-test]

Runs cargo fmt --check, cargo clippy, and cargo test in the supplied project.
Does not install toolchains or alter dependency manifests. Cargo may create
local build artifacts in target/ while compiling.
USAGE
}

project='.'
skip_clippy=false
skip_test=false

for arg in "$@"; do
  case "$arg" in
    -h|--help) usage; exit 0 ;;
    --skip-clippy) skip_clippy=true ;;
    --skip-test) skip_test=true ;;
    -*) echo "Unknown option: $arg" >&2; usage >&2; exit 2 ;;
    *) if [[ "$project" != '.' ]]; then echo 'Only one project directory is supported.' >&2; exit 2; fi; project="$arg" ;;
  esac
done

command -v cargo >/dev/null 2>&1 || { echo 'cargo not found' >&2; exit 127; }
cd "$project"
[[ -f Cargo.toml ]] || { echo "Cargo.toml not found in: $(pwd)" >&2; exit 2; }

printf '\n== rustfmt ==\n'
cargo fmt --all -- --check

if [[ "$skip_clippy" == false ]]; then
  printf '\n== clippy ==\n'
  cargo clippy --workspace --all-targets -- -D warnings
fi

if [[ "$skip_test" == false ]]; then
  printf '\n== tests ==\n'
  cargo test --workspace
fi

printf '\nAll enabled checks passed.\n'
