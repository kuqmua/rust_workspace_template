# Runtime tests

This crate runs reusable HTTP contract tests against services that are already
running, including services started with Docker Compose. It does not own the
containers, so the same suite can target local processes, Compose, or a CI
environment.

With the workspace Compose stack running:

```bash
cargo run -p runtime_tests
```

The CLI defaults are `http://127.0.0.1:8080` for the application and
`http://127.0.0.1:8081` for the notification service.

The library API exposes `RuntimeTestConfig` and `run`, allowing another test
runner to execute the same suite against other service URLs without spawning
this binary. Paths, expected statuses, requests, and responses are sourced from
the workspace contract crates. The notification test creates one notification
with the message `runtime-test`.
