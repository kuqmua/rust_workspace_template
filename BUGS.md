# Confirmed Bug Review

Review date: 2026-07-14

Scope: runtime server composition, administrator authentication, administrator Leptos frontend, browser transport, and shutdown behavior. Findings below are based on reachable code paths in the current worktree. Passing tests are not treated as proof when they do not exercise the relevant interaction.

## 1. Critical: concurrent authentication refresh deadlocks the WASM thread

- Location: `server_admin_frontend/src/app.rs:149-164`
- Trigger: two API calls receive `401` while one refresh is already in progress.
- Behavior: the `RwLockWriteGuard` created in the `match` scrutinee remains alive for the entire `match`. The `Join` arm calls `write()` on the same `std::sync::RwLock` again. Because browser WASM runs on one thread, the second acquisition blocks that thread permanently instead of waiting asynchronously.
- Impact: the administrator UI freezes on `Loading...` and cannot complete the active refresh request.
- Evidence gap in tests: `auth_keep_alive::tests::simultaneous_callers_join_one_refresh` tests only `AuthRefreshState`; it does not exercise `AdminApiClient::refresh_session` or the nested lock acquisition.
- Suggested correction: acquire the coordinator once in a separate scope, call `begin`, register a waiter using the same guard, and drop the guard before any await or branch return.

## 2. High: out-of-order page responses overwrite the currently selected page

- Location: `server_admin_frontend/src/app.rs:335-373`, initiated from `server_admin_frontend/src/app/pages.rs:31`
- Trigger: select page A and then page B before page A's API request completes.
- Behavior: every `load` call starts an independent `spawn_local`. There is no request generation, cancellation, or selected-path check before `page.set`. If A completes after B, A overwrites B's content even though the URL and active navigation item point to B.
- Impact: the header can show one section while the main area displays another section's data. Slow audit, metrics, or database requests make this reproducible.
- Suggested correction: attach a monotonically increasing load generation to the page state, or cancel the previous task, and accept a response only when its path/generation is still current.

## 3. High: browser Back and Forward desynchronize URL and page content

- Location: `server_admin_frontend/src/app.rs:308-313`, `server_admin_frontend/src/app/pages.rs:14-31`
- Trigger: navigate through two administrator sections, then use the browser Back or Forward button.
- Behavior: internal navigation writes history entries with `pushState`, but no `popstate` listener updates `current_path` or calls `load`.
- Impact: the address bar changes while the active navigation item and displayed data remain on the previous section. Reloading then shows a different page from the one visible immediately before reload.
- Suggested correction: own a `popstate` listener for the lifetime of `Shell`, update the path signal, and run the same page-loading path used by link clicks.

## 4. High: refresh rotation can consume the only valid token without delivering its replacement

- Location: `server_admin/src/auth/handlers.rs:215-267`
- Trigger: refresh-token rotation succeeds in PostgreSQL, but `load_authenticated_admin`, contract conversion, or response-header construction fails after the transaction commit.
- Behavior: the old refresh token is revoked at lines 215-221 and the replacement is committed at line 258. The new cookies are not appended until line 267. Any error between commit and the response leaves the browser holding only the revoked token.
- Impact: a transient database or serialization/header failure permanently signs the user out even though the server created a new inaccessible session.
- Suggested correction: prepare all fallible response data before commit where possible. If post-commit response construction can still fail, use a rotation protocol that permits recovery without accepting unlimited replay.

## 5. High: refresh-token rotation is not safe across browser tabs

- Location: `server_admin/src/auth/handlers.rs:215-225`; frontend coordination is process-local at `server_admin_frontend/src/app.rs:38-45`
- Trigger: two tabs share the same cookies and attempt refresh after the access token expires.
- Behavior: the first request atomically revokes the refresh token. The second request uses the same old cookie, receives `401`, and its frontend redirects to sign-in. The in-memory coordinator cannot coordinate separate tabs or windows.
- Impact: normal multi-tab use can appear to randomly lose authentication during token expiry.
- Suggested correction: coordinate refresh across tabs with a browser-wide lock/broadcast protocol, or implement a bounded server-side rotation grace/reuse model with explicit replay handling.

## 6. High: the global API rate limit is shared by every user behind one reverse proxy

- Location: `server/src/main.rs:341-355`
- Trigger: deploy behind a reverse proxy or ingress where all connections have the proxy's peer address.
- Behavior: `GovernorConfigBuilder::default()` uses `PeerIpKeyExtractor`. The configured rate is only two requests per second with burst ten, and the layer wraps every `/api/v1` route. Forwarded client-address resolution is not connected to this limiter.
- Impact: unrelated users consume one shared bucket and receive `429`. Health/version/operational requests can also be throttled by ordinary application traffic, depending on their placement inside `api_routes`.
- Suggested correction: use the repository's trusted-proxy client-IP resolution for a reviewed key extractor, separate public/health and authenticated limits, and make limits configurable.

## 7. Medium: configured cross-origin browser clients fail CORS preflight

- Location: outbound headers in `server_admin_frontend/src/transport.rs:30-44`; allowed request headers in `server/src/main.rs:370-383`
- Trigger: serve the frontend from a configured allowed origin that differs from the API origin.
- Behavior: every request adds the custom `commit` header, and some requests add `Idempotency-Key` or `If-Match`. The CORS layer allows only `Content-Type` and `X-CSRF-Token`.
- Impact: the browser rejects preflight before the API request reaches a handler, even though the origin is explicitly listed in `CORS_ALLOW_ORIGIN`.
- Suggested correction: add every contract-supported request header to the CORS allow-list and add a preflight integration test using the real frontend header set.

## 8. Medium: administrator HTML clears the cache for the entire origin on every load

- Location: `server_admin_frontend/src/lib.rs:50-57`
- Trigger: load or reload any administrator page.
- Behavior: every admin HTML response contains `Clear-Site-Data: "cache"`. This directive applies to the origin, not only `/admin` or obsolete frontend assets.
- Impact: unrelated applications hosted on the same origin lose cached resources, and immutable hashed assets are repeatedly evicted after full-page administrator navigation. It can materially increase bandwidth and startup latency.
- Suggested correction: remove the permanent origin-wide directive after the stale-build migration, or use a versioned one-time migration mechanism scoped by a small marker rather than every HTML response.

## 9. Medium: SIGTERM bypasses graceful shutdown

- Location: `server/src/main.rs:390-407`
- Trigger: stop the service using SIGTERM, which is the normal termination signal for containers and many service managers.
- Behavior: graceful shutdown waits only for `tokio::signal::ctrl_c()` (SIGINT). There is no Unix SIGTERM listener.
- Impact: in-flight requests and the cleanup task can be terminated without the shutdown timeout and task join path.
- Suggested correction: on Unix, wait for either Ctrl-C or SIGTERM; retain a portable Ctrl-C-only branch for non-Unix targets.

## Review priorities

1. Fix finding 1 before adding more concurrent administrator requests; it can freeze the entire frontend.
2. Fix findings 4 and 5 together as one refresh-rotation design change.
3. Fix findings 2 and 3 together in a single navigation owner that handles cancellation and browser history.
4. Fix finding 6 before deploying behind an ingress or serving multiple users.
5. Address findings 7-9 as deployment and operational hardening.

