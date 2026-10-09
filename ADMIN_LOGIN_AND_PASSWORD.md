# Local development administrator

- URL: http://127.0.0.1:8080/admin/users
- Database: rust_workspace_template
- Database user: `postgres`
- Database password: `Dev-admin-2026-Ready!`
- Login: `admin`
- Password: `Dev-admin-2026-Ready!`

The local development database and administrator account were recreated on 2026-10-09
through the initial migrations and administrator initialization command.
The documented login and password remain unchanged.

Creating an initial administrator or resetting its password keeps
`must_change_password=false`. Administrator navigation is available immediately after signing in.
The initial schema defaults the flag to `false` and PostgreSQL rejects attempts to set it to `true`.
The password form on `/admin/profile` remains available for voluntary password changes.

Run the login check against the running local server:

```bash
RUN_DEVELOPMENT_ADMIN_TEST=1 node --test browser_acceptance/test_development_admin.mjs
```

The check reads the login and password from this file so they remain the source of truth.
