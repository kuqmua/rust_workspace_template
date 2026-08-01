#!/usr/bin/env bash
set -euo pipefail

if [[ "$#" -ne 1 || ! -f "$1" ]]; then
  echo "usage: validate-production-manifest.sh <rendered-manifest>" >&2
  exit 2
fi

manifest="$1"

if grep --extended-regexp --quiet \
  'example\.invalid|replace-with-[a-z-]+' "${manifest}"; then
  echo "production manifest contains an example or replacement value" >&2
  exit 1
fi

if grep --fixed-strings --quiet \
  'TRUSTED_PROXY_RANGES_TEXT: 127.0.0.1/32' "${manifest}"; then
  echo "production manifest must declare the real trusted ingress proxy ranges" >&2
  exit 1
fi

digest_image_count=$(awk \
  '/^[[:space:]]*image: .+@sha256:[0-9a-f]{64}$/ { count += 1 } END { print count + 0 }' \
  "${manifest}")
if [[ "${digest_image_count}" -lt 2 ]]; then
  echo "production manifest must pin both service images by sha256 digest" >&2
  exit 1
fi

grep --fixed-strings --quiet 'PRODUCTION_MODE: "true"' "${manifest}" || {
  echo "production manifest must enable production mode" >&2
  exit 1
}
grep --fixed-strings --quiet 'ADMIN_COOKIE_SECURE: "true"' "${manifest}" || {
  echo "production manifest must require secure administrator cookies" >&2
  exit 1
}

for required_kind in Deployment NetworkPolicy PodDisruptionBudget; do
  grep --fixed-strings --quiet "kind: ${required_kind}" "${manifest}" || {
    echo "production manifest is missing ${required_kind}" >&2
    exit 1
  }
done
