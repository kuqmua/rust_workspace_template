# Administrator frontend

The native target renders the authenticated admin shell. The `wasm32` target renders the users,
roles, permissions, sessions, profile, settings, and generated data-table pages in the browser.
Those pages read and mutate data exclusively through the typed `/v1/admin` JSON API.
Sign-in and static operational pages remain server-rendered.

Build browser assets before running the server locally:

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
