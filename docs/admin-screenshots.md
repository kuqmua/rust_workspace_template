# Administrator console screenshots

Screenshots are generated from the same disposable PostgreSQL and local application harness used
by browser acceptance:

```bash
cd browser_acceptance
BROWSER_ACCEPTANCE_DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/rust_workspace_template_browser_test \
  npm run screenshots
```

## Sign in

![Administrator sign-in](images/admin/sign-in.png)

## Users

![Administrator user management](images/admin/users.png)

## Roles

![Administrator role management](images/admin/roles.png)

## Settings

![Administrator runtime settings](images/admin/settings.png)

## Sessions

![Administrator session management](images/admin/sessions.png)

## Data tables

![Administrator read-only data table](images/admin/data-table.png)
