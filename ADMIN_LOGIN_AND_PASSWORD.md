# Local development administrator

- URL: http://127.0.0.1:8080/admin/users
- Database: rust_workspace_template
- Database user: `postgres`
- Database password: `Dev-admin-2026-Ready!`
- Login: `admin`
- Password: `Dev-admin-2026-Ready!`

The current local development administrator account was created on 2026-09-30.
The mandatory Profile password step was completed on 2026-10-01.
The documented login and password remain unchanged.

Creating an initial administrator or resetting its password sets
`must_change_password=true`. After signing in, complete the password form on
`/admin/profile` to unlock administrator navigation. Recreating the database
requires this same step after initializing the administrator.

Run the login check against the running local server:

```bash
RUN_DEVELOPMENT_ADMIN_TEST=1 node --test browser_acceptance/test_development_admin.mjs
```

The check reads the login and password from this file so they remain the source of truth.
