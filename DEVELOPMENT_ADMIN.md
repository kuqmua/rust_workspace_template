# Local development administrator

- URL: http://127.0.0.1:8080/admin/users
- Database: rust_workspace_template
- Login: `admin`
- Password: `Dev-admin-2026-Ready!`

These credentials belong to the local development database recreated on 2026-09-06.
The initial mandatory password change has been completed.

Run the login check against the running local server:

```bash
RUN_DEVELOPMENT_ADMIN_TEST=1 node --test browser_acceptance/test_development_admin.mjs
```

The check reads the login and password from this file so they remain the source of truth.
