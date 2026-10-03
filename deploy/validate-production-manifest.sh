#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 1 || ! -f "$1" ]]; then
  echo "usage: validate-production-manifest.sh <rendered-manifest>" >&2
  exit 2
fi

workspace_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if cargo run --quiet --locked --manifest-path "${workspace_directory}/Cargo.toml" \
  --package workspace_scaffold -- manifest "$1"; then
  exit 0
fi
exit 1
