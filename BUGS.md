# Resolved Bug Review

Resolution date: 2026-07-14

All findings from the 2026-07-14 review are resolved in the current worktree. This document records the original locations, implemented corrections, and regression evidence.

## 1. Resolved: concurrent authentication refresh deadlock

- Original location: `server_admin_frontend/src/app.rs`, `AdminApiClient::refresh_session`.
- Correction: refresh state inspection and waiter registration now use one scoped write guard. The guard is dropped before any await. The `Join` branch no longer attempts to acquire the same `RwLock` recursively.
- Regression evidence: Playwright starts two protected requests that receive `401` concurrently, verifies one refresh request, two successful retries, and a responsive UI.

## 2. Resolved: stale page responses overwrote the selected page

- Original location: `server_admin_frontend/src/app.rs`, `load`.
- Correction: `PageLoader` assigns a monotonic generation to every navigation. A response updates page state only if its generation is still current.
- Regression evidence: Playwright delays the permissions response, navigates to audit, and verifies that the late permissions response cannot replace audit content.

## 3. Resolved: browser history desynchronized URL and content

- Original location: `server_admin_frontend/src/app/pages.rs`, `Shell`.
- Correction: `Shell` owns a `popstate` callback that updates the current-path signal and loads the matching page through `PageLoader`.
- Regression evidence: Playwright navigates across users, permissions, and audit, then verifies Back and Forward update the URL, active layout, and content without rechecking the session.

## 4. Resolved: refresh could consume a token before delivering a replacement

- Original location: `server_admin/src/auth/handlers.rs`, `refresh`.
- Correction: refresh validation now locks and reads the bounded refresh session without revoking it. The new access session, authenticated view, response contract, and access/CSRF cookies are prepared before the transaction commits.
- Regression evidence: the administrator API flow verifies refresh success while retaining the original bounded refresh token and issuing new access/CSRF cookies only.

## 5. Resolved: refresh was unsafe across browser tabs

- Original location: `server_admin/src/auth/handlers.rs`, refresh-token rotation.
- Correction: a refresh token now represents a persistent, server-revocable session bounded by its original database and cookie expiry. Concurrent tabs serialize validation with `FOR UPDATE` and can independently obtain new access sessions without invalidating each other.
- Security invariant: sign-out, password/security actions, session limits, explicit revocation, and expiry still revoke or reject the refresh session.

## 6. Resolved: reverse-proxy users shared one rate-limit key

- Original location: `server/src/main.rs`, global governor construction.
- Correction: `ClientIpRateLimitKeyExtractor` resolves forwarded addresses only when the direct peer belongs to configured `TRUSTED_PROXY_RANGES_TEXT`. Untrusted peers cannot spoof forwarded headers. Operational common routes are outside the application governor.
- Configuration: development defaults trust only `127.0.0.1/32` and `::1/128`; deployments must list their actual proxy CIDRs.
- Regression evidence: a server unit test verifies that a forwarded client is used through a trusted proxy.

## 7. Resolved: cross-origin frontend headers failed preflight

- Original location: `server/src/main.rs`, CORS layer.
- Correction: the allow-list now includes `commit`, `Idempotency-Key`, and `If-Match` in addition to `Content-Type` and `X-CSRF-Token`, matching the browser transport contract.

## 8. Resolved: admin HTML cleared the entire origin cache

- Original location: `server_admin_frontend/src/lib.rs`, admin HTML response layers.
- Correction: the permanent `Clear-Site-Data: "cache"` response header was removed. Hashed immutable assets and non-cacheable HTML now provide version consistency without evicting unrelated origin data.

## 9. Resolved: SIGTERM bypassed graceful shutdown

- Original location: `server/src/main.rs`, shutdown future.
- Correction: Unix builds wait for either Ctrl-C or SIGTERM. Non-Unix builds retain Ctrl-C handling. Both paths continue through request draining and cleanup-task shutdown.

## Verification gates

- `cargo fmt`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test -p tests code_style`
- `cargo test -p server_admin`
- `cargo test -p server_admin_frontend`
- `trunk build --release`
- Playwright administrator navigation, history, stale-response, refresh, and layout tests

