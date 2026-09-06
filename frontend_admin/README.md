# Administrator frontend

`frontend_admin` is the administrator frontend library, replacing the former `frontend`
package. It owns the shared Leptos components, server rendering, and browser application.
The server consumes this library directly; the old package and directory are removed.

The native target renders the authenticated admin shell. The `wasm32` target renders the users,
roles, permissions, sessions, profile, settings, and generated data-table pages in the browser.
Those pages read and mutate data exclusively through the typed `/v1/admin` JSON API.
Sign-in, user and role management forms, and operational pages remain server-rendered.

On the first sign-in, change the initial password on Profile before using other pages.
The server enforces this requirement; the frontend shows an explanation and displays
only Profile navigation until the change succeeds. Header links become available after
the password change reloads the page.

`cargo run -p server` (or `cargo run` from `server`) automatically builds browser
assets before starting the server. Install Node.js 22 or newer and Trunk once:

```bash
cargo install trunk --version 0.21.14 --locked
```

The Rust server startup installs the pinned npm dependencies when their manifests or Node.js version
change, ensures the WebAssembly target is installed, and runs the existing Trunk pipeline.
It reuses a separate Cargo cache in `target/frontend` and `target/frontend-build`.
If available, it uses the local Node.js installation at
`~/.local/share/rust_workspace_template_tools/node/bin`; `FRONTEND_NODE_BIN` can override
this directory. Build failures prevent server startup.
Migration mode (`SVC_MODE=migrate`) and other workspace binaries skip frontend preparation.
This is startup preparation, not a file watcher: rerun `cargo run` after frontend changes.

The orchestration lives in the existing `server_runtime_http` crate and uses Tokio child
processes; it needs no shell or Python runner. Its environment and dependency-cache access
are centralized in `frontend_build_environment`, the designated owner for development
frontend preparation. Production builds with `ADMIN_FRONTEND_STATIC_DIR` set at compile
time use their packaged assets and skip this preparation.

Run the deterministic Rust checks from the workspace root with
`cargo test -p server_runtime_http test_frontend_preparation`.

For a manual frontend-only build:

```bash
rustup target add wasm32-unknown-unknown
npm ci
trunk build --release
```

Run these commands from this directory. The Trunk pre-build hook compiles the Rust/UI Tailwind
stylesheet. Trunk writes generated JavaScript and WebAssembly into `static/csr`; the server exposes
the generated assets below `/admin/assets`.

The server CSP must allow WebAssembly compilation. Existing local `server/.env` files created
before the CSR frontend need this directive:

```text
script-src 'self' 'wasm-unsafe-eval'
```

Requests from an open browser page recover expired access and CSRF cookies through the
typed refresh route, with at most one refresh and one retry. Ordinary network failures
and non-authentication server errors are not replayed. Full-page navigation retains the
server authentication policy: an expired access session redirects to sign-in.
