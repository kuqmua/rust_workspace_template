# Administrator architecture

The administrator console is part of the application service. It is not a separately deployed
trust boundary.

```mermaid
flowchart LR
    Browser[Administrator browser]
    Ingress[TLS ingress]
    Server[Application server]
    Html[SSR sign-in and operational pages]
    Wasm[Typed CSR WebAssembly pages]
    Auth[Authentication, CSRF, sessions]
    Rbac[RBAC and permission catalog]
    Api[Typed admin API and OpenAPI]
    Repo[Admin repositories and migrations]
    Pg[(PostgreSQL)]
    Audit[Protected audit export destination]

    Browser --> Ingress --> Server
    Server --> Html
    Server --> Wasm
    Html --> Auth
    Wasm --> Api
    Api --> Auth
    Api --> Rbac
    Auth --> Repo
    Rbac --> Repo
    Api --> Repo
    Repo --> Pg
    Pg --> Audit
```

`server_admin_contract` owns transport wrappers, routes, permissions, page metadata, and OpenAPI
inputs. `server_admin_core` owns shared administrator domain policy. `server_admin` owns HTTP
handlers, repositories, migrations, authentication, authorization, audit, rate limiting, and
cleanup. `server_admin_frontend` owns SSR and CSR presentation. Shared service lifecycle and HTTP
policy remain in `server_runtime_core` and `server_runtime_http`.

PostgreSQL is the consistency boundary for identities, sessions, RBAC, settings, rate limits,
cleanup state, and append-only audit records. Multiple application replicas therefore share
session and rate-limit state; cleanup tasks must use the repository's database coordination rather
than process-local ownership.
