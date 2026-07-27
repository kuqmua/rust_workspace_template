#!/usr/bin/env bash
set -euo pipefail

CARGO_PROFILE_RELEASE_LTO=false \
  CARGO_PROFILE_RELEASE_STRIP=none \
  cargo fuzz run domain_boundaries -- "$@"
