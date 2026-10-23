#!/usr/bin/env bash
# Rust quality checks for ordinary Cargo projects.
set -Eeuo pipefail

usage() {
  cat <<'USAGE'
Usage: check.sh [CARGO_PROJECT_DIRECTORY] [--skip-clippy] [--skip-test]

Runs cargo fmt --check, cargo clippy, and cargo test in the supplied project.
Requires Bash, Cargo, rustfmt, and (unless skipped) Clippy.
Runs the project's build scripts and tests. Cargo may fetch dependencies,
update Cargo.lock, and create build artifacts. Use only on trusted projects.
USAGE
}

project='.'
project_set=false
skip_clippy=false
skip_test=false

for arg in "$@"; do
  case "$arg" in
    -h|--help) usage; exit 0 ;;
    --skip-clippy) skip_clippy=true ;;
    --skip-test) skip_test=true ;;
    -*) echo "Unknown option: $arg" >&2; usage >&2; exit 2 ;;
    *) if [[ "$project_set" == true ]]; then echo 'Only one project directory is supported.' >&2; exit 2; fi; project="$arg"; project_set=true ;;
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
