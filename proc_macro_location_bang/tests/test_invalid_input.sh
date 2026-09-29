#!/usr/bin/env bash
set -euo pipefail

repository_root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$repository_root"
cargo build -p location_test --quiet

target_dir=${CARGO_TARGET_DIR:-target}
dependencies_dir="$target_dir/debug/deps"
macro_file=$(ls -t "$dependencies_dir"/libproc_macro_location_bang-*.so | sed -n '1p')
location_file=$(ls -t "$dependencies_dir"/liblocation_lib-*.rlib | sed -n '1p')
if [[ ! -f $macro_file || ! -f $location_file ]]; then
    printf 'location macro test dependencies were not built\n' >&2
    exit 1
fi

test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT
if rustc --edition=2024 --crate-type=lib \
    --extern "proc_macro_location_bang=$macro_file" \
    --extern "location_lib=$location_file" \
    -L "dependency=$dependencies_dir" \
    --out-dir "$test_dir" \
    proc_macro_location_bang/tests/ui/invalid_input.rs \
    >"$test_dir/stdout" 2>"$test_dir/stderr"; then
    printf 'location! unexpectedly accepted an argument\n' >&2
    exit 1
fi
rg -q -F 'location! accepts no arguments' "$test_dir/stderr"
