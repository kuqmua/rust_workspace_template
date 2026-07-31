#!/usr/bin/env bash
set -euo pipefail

: "${BROWSER_ACCEPTANCE_DATABASE_URL:?set BROWSER_ACCEPTANCE_DATABASE_URL to a disposable PostgreSQL database ending in _browser_test}"

case "${BROWSER_ACCEPTANCE_DATABASE_URL}" in
  */*_browser_test | */*_browser_test\?*) ;;
  *)
    echo "browser acceptance database name must end in _browser_test" >&2
    exit 2
    ;;
esac

workspace_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
password_file="${workspace_dir}/target/browser_acceptance_admin_password"

cd "${workspace_dir}"

export ADMIN_ACCESS_TOKEN_TTL_SECONDS="900"
export ADMIN_COOKIE_SECURE="false"
export ADMIN_JWT_SECRET="browser-acceptance-jwt-secret-00001"
export ADMIN_LOGIN_FAILURE_LIMIT="10"
export ADMIN_PASSWORD_HASH_CONCURRENCY="1"
export ADMIN_REFRESH_TOKEN_TTL_SECONDS="3600"
export ADMIN_SESSION_LIMIT="${BROWSER_ACCEPTANCE_SESSION_LIMIT:-64}"
export ADMIN_SIGN_IN_RATE_LIMIT="128"
export ADMIN_SWAGGER_ENABLED="false"
export ADMIN_TOKEN_AUDIENCE="browser-acceptance"
export ADMIN_TOKEN_ISSUER="browser-acceptance"
export CONTENT_SECURITY_POLICY="default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; object-src 'none'; frame-ancestors 'none'"
export CORS_ALLOW_ORIGIN="http://127.0.0.1:18080"
export DATABASE_URL="${BROWSER_ACCEPTANCE_DATABASE_URL}"
export ENABLE_API_GIT_COMMIT_CHECK="true"
export HTTP_GZIP_ENABLED="true"
export MAXIMUM_SIZE_OF_HTTP_BODY_IN_BYTES="1048576"
export NO_COLOR="true"
export PG_POOL_ACQUIRE_TIMEOUT_SECONDS="10"
export PG_POOL_IDLE_TIMEOUT_SECONDS="60"
export PG_POOL_MAX_CONNECTIONS="8"
export PG_POOL_MAX_LIFETIME_SECONDS="300"
export PG_POOL_MIN_CONNECTIONS="1"
export PRODUCTION_MODE="false"
export REQUEST_TIMEOUT_SECONDS="30"
export SERVICE_SOCKET_ADDRESS="127.0.0.1:18080"
export SRC_PLACE_TYPE="src"
export TIMEZONE="10800"
export TRACING_FORMAT="text"
export TRACING_LEVEL="info"
export TRUSTED_PROXY_RANGES_TEXT="127.0.0.1/32,::1/128"

psql "${DATABASE_URL}" --set ON_ERROR_STOP=1 \
  --command "DROP SCHEMA IF EXISTS public CASCADE" \
  --command "CREATE SCHEMA public"

(
  cd "server_admin_frontend"
  trunk build --release
)

umask 077
printf '%s\n' "Initial-password1!" > "${password_file}"

cd "server"

cargo run --package admin_bootstrap -- \
  administrator "Initial Administrator" "${password_file}"

rm -f "${password_file}"

exec cargo run --package server
