const { test, expect } = require('@playwright/test');
const [
  routeCatalog,
  permissionCatalog,
  authenticatedAdmin,
  users,
  roles,
  permissions,
  auditLog,
  sessions,
  noBody,
] = require('../../target/admin_contract_fixture.json');

const apiRoute = (operationId) => {
  const route = routeCatalog.find(([candidate]) => candidate === operationId);
  if (!route) throw new Error(`missing generated route fixture for ${operationId}`);
  return `/api/v1/admin${route[2]}`;
};
const tableOperations = new Set(['list_users', 'list_roles', 'list_permissions']);
const apiRouteGlob = (operationId) => `**${apiRoute(operationId)}${operationId === 'audit_log' ? '*' : tableOperations.has(operationId) ? '?*' : ''}`;
const usersPage = (requestUrl) => {
  const query = new URL(requestUrl).searchParams;
  const search = (query.get('search') || '').toLowerCase();
  const sort = query.get('sort') || 'login';
  const direction = query.get('direction') === 'desc' ? -1 : 1;
  const limit = Number(query.get('limit') || 20);
  const offset = Number(query.get('offset') || 0);
  const items = users.items
    .filter((user) => !search
      || user.login.toLowerCase().includes(search)
      || user.display_name.toLowerCase().includes(search)
      || String(user.id) === search)
    .sort((left, right) => String(left[sort] ?? left.login)
      .localeCompare(String(right[sort] ?? right.login)) * direction);
  return { ...users, items: items.slice(offset, offset + limit), total: items.length };
};

test.beforeEach(async ({ page }) => {
  await page.route(apiRouteGlob('branding'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({
      default_admin_route: '/admin/users',
      main_logo: null,
      primary_color: '#6757e8',
      site_name: 'Workspace Admin',
      support_url: 'https://support.example.com',
      tab_title: 'Workspace Control',
    }) });
  });
  await page.route(apiRouteGlob('me'), async (route) => {
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify(authenticatedAdmin),
    });
  });
  await page.route(apiRouteGlob('list_users'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(usersPage(route.request().url())) });
  });
  await page.route(apiRouteGlob('list_roles'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(roles) });
  });
  await page.route('**/api/v1/admin/openapi.json', async (route) => {
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify({ openapi: '3.1.0', paths: { '/users': { get: {} } } }),
    });
  });
  await page.route(apiRouteGlob('list_permissions'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(permissions) });
  });
  await page.route(apiRouteGlob('audit_log'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(auditLog) });
  });
  await page.route(apiRouteGlob('export_audit_log'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({ csv: 'id,action\n1,update\n' }) });
  });
  await page.route(apiRouteGlob('sessions'), async (route) => {
    if (route.request().method() === 'DELETE') {
      await route.fulfill({ status: 204 });
      return;
    }
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(sessions) });
  });
  await page.route(apiRouteGlob('change_own_password'), async (route) => {
    await route.fulfill({ status: 204 });
  });
  await page.route(apiRouteGlob('settings'), async (route) => {
    if (route.request().method() === 'PATCH') {
      await route.fulfill({ status: 204 });
      return;
    }
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({
      default_admin_route: '/admin/users',
      main_logo: null,
      organization_contacts: null,
      organization_name: 'Example Org',
      primary_color: '#6757e8',
      site_name: 'Workspace Admin',
      support_url: 'https://support.example.com',
      tab_title: 'Workspace Control',
    }) });
  });
  await page.route(apiRouteGlob('dashboard'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({
      active_sessions: 12,
      database_healthy: true,
      failed_sign_ins_24h: 3,
      last_cleanup: { deleted_rows: 7, last_success_at: "2026-07-17T12:00:00Z" },
      recent_changes: auditLog.items,
      uptime_seconds: 3600,
      version: '0123456789abcdef',
    }) });
  });
  await page.route(apiRouteGlob('mfa_status'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({
      enabled: false,
      recovery_codes_remaining: 0,
    }) });
  });
  await page.route(apiRouteGlob('mfa_enroll'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({
      secret: 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRST',
      uri: 'otpauth://totp/Admin%20Console:root?secret=ABCDEFGHIJKLMNOPQRSTUVWXYZ234567ABCDEFGHIJKLMNOPQRST&issuer=Admin%20Console&algorithm=SHA256&digits=6&period=30',
    }) });
  });
  await page.route(apiRouteGlob('mfa_confirm'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({
      recovery_codes: ['abcd-1234-5678-90ef', '1111-2222-3333-4444'],
    }) });
  });
});

test('sign-in renders without starting authenticated resources', async ({ page }) => {
  const authenticatedRequests = [];
  const pageErrors = [];
  page.on('request', (request) => {
    if (request.url().includes(apiRoute('me'))) authenticatedRequests.push(request.url());
  });
  page.on('pageerror', (error) => pageErrors.push(error.message));

  const response = await page.goto('/admin/sign-in');
  expect(response.headers()['cache-control']).toContain('no-cache');
  await expect(page.getByRole('heading', { name: 'Welcome back' })).toBeVisible();
  await expect(page.locator('.auth-brand')).toContainText('Workspace Admin');
  await expect(page).toHaveTitle('Workspace Control');
  await expect(page.getByRole('button', { name: 'Sign in' })).toBeVisible();
  await expect(page.locator('link[rel="modulepreload"]')).toHaveAttribute(
    'href',
    /server_admin_frontend-[a-f0-9]+\.js$/,
  );
  await page.waitForTimeout(100);
  expect(authenticatedRequests).toEqual([]);
  expect(pageErrors).toEqual([]);
});

test('settings validate and save branding with explicit optional clearing', async ({ page }) => {
  let settingsRequest;
  await page.unroute(apiRouteGlob('settings'));
  await page.route(apiRouteGlob('settings'), async (route) => {
    if (route.request().method() === 'PATCH') {
      settingsRequest = route.request().postDataJSON();
      await route.fulfill({ status: 204 });
      return;
    }
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({
      default_admin_route: '/admin/users', main_logo: 'https://cdn.example.com/logo.svg',
      organization_contacts: null, organization_name: 'Example Org', primary_color: '#6757e8',
      site_name: 'Workspace Admin', support_url: 'https://support.example.com', tab_title: 'Workspace Control',
    }) });
  });

  await page.goto('/admin/system-settings');
  await page.getByRole('button', { name: 'Restore defaults' }).click();
  await expect(page.getByLabel('Site name')).toHaveValue('Admin Console');
  await expect(page.getByLabel('Default admin route')).toHaveValue('/admin/dashboard');
  await page.getByRole('button', { name: 'Reset unsaved changes' }).click();
  await page.getByLabel('Primary color').fill('red');
  await page.getByRole('button', { name: 'Save changes' }).click();
  await expect(page.getByRole('alert')).toContainText('#RRGGBB');
  await page.getByLabel('Primary color').fill('#123ABC');
  await page.getByLabel('Logo URL').fill('');
  await page.getByRole('button', { name: 'Save changes' }).click();
  await expect.poll(() => settingsRequest).toBeTruthy();
  expect(settingsRequest.clear).toContain('main_logo');
  expect(settingsRequest.primary_color).toBe('#123ABC');
  await expect(page.getByText('Settings saved')).toBeVisible();
});

test('expired access session is refreshed without returning to sign-in', async ({ page }) => {
  await page.unroute(apiRouteGlob('me'));
  let meRequests = 0;
  let refreshRequests = 0;
  await page.route(apiRouteGlob('me'), async (route) => {
    meRequests += 1;
    if (meRequests === 1) {
      await route.fulfill({ status: 401, contentType: 'application/problem+json', body: '{}' });
      return;
    }
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify(authenticatedAdmin),
    });
  });
  await page.route(apiRouteGlob('refresh'), async (route) => {
    refreshRequests += 1;
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(noBody) });
  });

  await page.goto('/admin/users');
  await expect.poll(() => refreshRequests).toBe(1);
  await expect.poll(() => meRequests).toBe(2);
  await expect(page.getByRole('heading', { name: 'Users' })).toBeVisible();
  expect(page.url()).toContain('/admin/users');
  expect(meRequests).toBe(2);
  expect(refreshRequests).toBe(1);
});

test('temporary session check failure does not discard authentication', async ({ page }) => {
  await page.unroute(apiRouteGlob('me'));
  await page.route(apiRouteGlob('me'), async (route) => {
    await route.fulfill({
      status: 503,
      contentType: 'application/problem+json',
      body: JSON.stringify({ detail: 'temporary failure' }),
    });
  });

  await page.goto('/admin/users');
  await expect(page.getByText('Unable to verify session')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Try again' })).toBeVisible();
  expect(page.url()).toContain('/admin/users');
});

test('OpenAPI page is rendered by the Leptos SPA', async ({ page }) => {
  await page.goto('/admin/swagger-ui');
  await expect(page.getByRole('heading', { name: 'OpenAPI document' })).toBeVisible();
  await expect(page.locator('#openapi')).toContainText('"openapi": "3.1.0"');
  await expect(page.locator('#openapi')).toContainText('"/users"');
  await expect(page.locator('script[src$="swagger.js"]')).toHaveCount(0);
});

test('dashboard renders structured operational summary', async ({ page }) => {
  await page.goto('/admin/dashboard');
  await expect(page.getByRole('heading', { name: 'Dashboard' })).toBeVisible();
  await expect(page.locator('.dashboard-grid')).toContainText('Healthy');
  await expect(page.locator('.dashboard-grid')).toContainText('12');
  await expect(page.locator('.dashboard-grid')).toContainText('3');
  await expect(page.locator('.dashboard-grid')).toContainText('7 rows deleted');
  await expect(page.locator('.recent-changes')).toContainText('Recent administrative changes');
});

test('header navigation and table discovery controls work in the SPA', async ({ page }) => {
  await page.goto('/admin/users');

  const header = page.locator('header.topbar');
  await expect(header).toBeVisible();
  await expect(header.getByRole('navigation', { name: 'Admin sections' })).toBeVisible();
  await expect(header.getByRole('link', { name: 'Users' })).toHaveClass(/active/);
  await expect(page.locator('header.sidebar')).toHaveCount(0);

  await expect(page.locator('tbody tr')).toHaveCount(20);
  await expect(page.getByText('1-20 of 25')).toBeVisible();
  await page.getByRole('button', { name: 'Next page' }).click();
  await expect(page).toHaveURL(/offset=20/);
  await expect(page.locator('tbody tr')).toHaveCount(5);
  await expect(page.getByText('21-25 of 25')).toBeVisible();
  await page.getByRole('button', { name: 'Previous page' }).click();

  await page.getByRole('searchbox', { name: 'Filter rows' }).fill('alpha');
  await expect(page).toHaveURL(/search=alpha/);
  await expect(page.locator('tbody tr')).toHaveCount(1);
  await expect(page.locator('tbody')).toContainText('Alpha Operator');
  await expect(page.getByText('1-1 of 1')).toBeVisible();

  await page.getByRole('searchbox', { name: 'Filter rows' }).fill('');
  await page.getByLabel('Sort field').selectOption('display_name');
  await expect(page.locator('tbody tr').first()).toContainText('Alpha Operator');
  await page.getByRole('button', { name: 'Toggle sort direction' }).click();
  await expect(page.locator('tbody tr').first()).not.toContainText('Alpha Operator');

  await page.getByLabel('Rows per page').selectOption('10');
  await expect(page.locator('tbody tr')).toHaveCount(10);
  await expect(page.getByText('1-10 of 25')).toBeVisible();
});

test('role and permission editors show current named assignments', async ({ page }) => {
  await page.goto('/admin/users');
  const userRow = page.locator('tbody tr').first();
  await userRow.getByText('Roles', { exact: true }).click();
  const roleCheckbox = userRow.getByRole('checkbox', { name: 'administrator' });
  await expect(roleCheckbox).toBeChecked();
  await userRow.getByRole('searchbox', { name: 'Filter roles' }).fill('missing');
  await expect(roleCheckbox).toBeHidden();

  await page.getByRole('link', { name: 'Roles' }).click();
  const createRole = page.locator('details.mutation-form').filter({ hasText: 'Create role' }).first();
  await createRole.locator('summary').click();
  await createRole.getByRole('button', { name: 'Create role' }).click();
  await expect(createRole.getByRole('alert')).toContainText('Role name');
  const roleRow = page.locator('tbody tr').first();
  await roleRow.getByText('Permissions', { exact: true }).click();
  await expect(roleRow.getByRole('checkbox').first()).toBeChecked();
  await expect(roleRow.getByRole('button', { name: 'Save permissions' })).toBeEnabled();
});

test('role assignment sends the complete before-and-after sets', async ({ page }) => {
  const viewer = { id: 2, is_system: false, name: 'viewer', permission_ids: [] };
  let requestBody;
  await page.unroute(apiRouteGlob('list_users'));
  await page.route(apiRouteGlob('list_users'), async (route) => {
    const response = usersPage(route.request().url());
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify({ ...response, roles: [...response.roles, viewer] }),
    });
  });
  await page.route('**/*', async (route) => {
    if (route.request().method() !== 'PUT' || !route.request().url().includes('/users/') || !route.request().url().endsWith('/roles')) {
      await route.fallback();
      return;
    }
    requestBody = route.request().postDataJSON();
    await route.fulfill({ status: 204 });
  });

  await page.goto('/admin/users');
  const row = page.locator('tbody tr').first();
  await row.getByText('Roles', { exact: true }).click();
  await row.getByRole('checkbox', { name: 'viewer' }).check();
  await row.getByRole('button', { name: 'Save roles' }).click();
  await expect(row.getByRole('button', { name: 'Saving...' })).toBeDisabled();
  await expect(page.getByText('User roles updated')).toBeVisible();
  expect(requestBody).toEqual({ expected_role_ids: [1], role_ids: [1, 2] });
});

test('role assignment preserves edits on stale and last-admin conflicts', async ({ page }) => {
  const viewer = { id: 2, is_system: false, name: 'viewer', permission_ids: [] };
  const responses = ['assignments changed by another administrator', 'cannot remove the last active administrator'];
  const requests = [];
  await page.unroute(apiRouteGlob('list_users'));
  await page.route(apiRouteGlob('list_users'), async (route) => {
    const response = usersPage(route.request().url());
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify({ ...response, roles: [...response.roles, viewer] }),
    });
  });
  await page.route('**/*', async (route) => {
    if (route.request().method() !== 'PUT' || !route.request().url().includes('/users/') || !route.request().url().endsWith('/roles')) {
      await route.fallback();
      return;
    }
    requests.push(route.request().postDataJSON());
    const detail = responses.shift();
    await route.fulfill({
      status: 409,
      contentType: 'application/problem+json',
      body: JSON.stringify({ detail, kind: 'conflict', request_id: null, status: 409, violations: [] }),
    });
  });

  await page.goto('/admin/users');
  const row = page.locator('tbody tr').first();
  await row.getByText('Roles', { exact: true }).click();
  await row.getByRole('checkbox', { name: 'viewer' }).check();
  await row.getByRole('button', { name: 'Save roles' }).click();
  await expect(row.getByRole('alert')).toContainText('another administrator');
  await expect(row.getByRole('checkbox', { name: 'viewer' })).toBeChecked();

  await row.getByRole('checkbox', { name: 'viewer' }).uncheck();
  await row.getByRole('checkbox', { name: 'administrator' }).uncheck();
  await row.getByRole('button', { name: 'Save roles' }).click();
  await expect(row.getByRole('alert')).toContainText('last active administrator');
  expect(requests).toEqual([
    { expected_role_ids: [1], role_ids: [1, 2] },
    { expected_role_ids: [1], role_ids: [] },
  ]);
});

test('user mutations use inline validated forms instead of browser prompts', async ({ page }) => {
  await page.goto('/admin/users');
  const createForm = page.locator('details.mutation-form').filter({ hasText: 'Create user' }).first();
  await createForm.locator('summary').click();
  await createForm.getByLabel('New user login').fill('invalid login');
  await createForm.getByLabel('New user display name').fill('Operator');
  await createForm.getByLabel('New user password').fill('short');
  await createForm.getByRole('button', { name: 'Create user' }).click();
  await expect(createForm.getByRole('alert')).toContainText('Check login');
  await expect(createForm.getByLabel('New user login')).toHaveValue('invalid login');
});

test('edit, ban, password and destructive user flows reach their typed routes', async ({ page }) => {
  const mutations = [];
  await page.route('**/*', async (route) => {
    const request = route.request();
    const path = new URL(request.url()).pathname;
    if (!/\/api\/v1\/admin\/users\/\d+(\/ban|\/password)?$/.test(path)) {
      await route.fallback();
      return;
    }
    mutations.push({ method: request.method(), path, body: request.postDataJSON() });
    await route.fulfill({ status: 204 });
  });

  await page.goto('/admin/users');
  let row = page.locator('tbody tr').first();
  await row.getByText('Edit', { exact: true }).click();
  await row.getByLabel('Edit user display name').fill('Updated Operator');
  await row.getByRole('button', { name: 'Save', exact: true }).click();
  await expect(page.getByText('User updated')).toBeVisible();

  row = page.locator('tbody tr').first();
  await row.getByRole('button', { name: 'Unban', exact: true }).click();
  await expect(page.getByText('User unbanned')).toBeVisible();

  row = page.locator('tbody tr').first();
  await row.getByText('Password', { exact: true }).click();
  await row.getByLabel('Change user password').fill('NewCorrectHorseBatteryStaple1!');
  await row.getByRole('button', { name: 'Change password' }).click();
  await expect(page.getByText('Password updated')).toBeVisible();

  row = page.locator('tbody tr').first();
  await row.getByText('Delete', { exact: true }).click();
  await row.getByLabel('Confirm user deletion').fill('alpha');
  await row.getByRole('button', { name: 'Delete permanently' }).click();
  await expect(page.getByText('User deleted')).toBeVisible();

  expect(mutations.map(({ method, path }) => [method, path.replace(/\/\d+/, '/{id}')])).toEqual([
    ['PATCH', '/api/v1/admin/users/{id}'],
    ['POST', '/api/v1/admin/users/{id}/ban'],
    ['POST', '/api/v1/admin/users/{id}/password'],
    ['DELETE', '/api/v1/admin/users/{id}'],
  ]);
  expect(mutations[0].body.display_name).toBe('Updated Operator');
  expect(mutations[1].body).toEqual({ is_banned: false });
});

test('create, edit, permission assignment and destructive role flows reach typed routes', async ({ page }) => {
  const mutations = [];
  await page.route('**/*', async (route) => {
    const request = route.request();
    const path = new URL(request.url()).pathname;
    const isCreate = request.method() === 'POST' && path === '/api/v1/admin/roles';
    const isDynamic = /\/api\/v1\/admin\/roles\/\d+(\/permissions)?$/.test(path);
    if (!isCreate && !isDynamic) {
      await route.fallback();
      return;
    }
    mutations.push({ method: request.method(), path, body: request.postDataJSON() });
    await route.fulfill(isCreate
      ? { status: 201, contentType: 'application/json', body: '{}' }
      : { status: 204 });
  });

  await page.goto('/admin/roles');
  const create = page.locator('details.mutation-form').filter({ hasText: 'Create role' }).first();
  await create.locator('summary').click();
  await create.getByLabel('New role name').fill('operators');
  await create.getByRole('button', { name: 'Create role' }).click();
  await expect(page.getByText('Role created')).toBeVisible();

  let row = page.locator('tbody tr').first();
  await row.getByText('Edit', { exact: true }).click();
  await row.getByLabel('Edit role name').fill('renamed_role');
  await row.getByRole('button', { name: 'Save', exact: true }).click();
  await expect(page.getByText('Role updated')).toBeVisible();

  row = page.locator('tbody tr').first();
  await row.getByText('Permissions', { exact: true }).click();
  await row.getByRole('checkbox').first().uncheck();
  await row.getByRole('button', { name: 'Save permissions' }).click();
  await expect(page.getByText('Role permissions updated')).toBeVisible();

  row = page.locator('tbody tr').first();
  await row.getByText('Delete', { exact: true }).click();
  await row.getByLabel('Confirm role deletion').fill('administrator');
  await row.getByRole('button', { name: 'Delete permanently' }).click();
  await expect(page.getByText('Role deleted')).toBeVisible();

  expect(mutations.map(({ method, path }) => [method, path.replace(/\/\d+/, '/{id}')])).toEqual([
    ['POST', '/api/v1/admin/roles'],
    ['PATCH', '/api/v1/admin/roles/{id}'],
    ['PUT', '/api/v1/admin/roles/{id}/permissions'],
    ['DELETE', '/api/v1/admin/roles/{id}'],
  ]);
  expect(mutations[2].body.expected_permission_ids.length).toBeGreaterThan(0);
  expect(mutations[2].body.permission_ids.length).toBe(mutations[2].body.expected_permission_ids.length - 1);
});

test('sessions page lists and revokes current administrator sessions', async ({ page }) => {
  let revokedSessionRequests = 0;
  let revokedAllRequests = 0;
  await page.route('**/api/v1/admin/auth/sessions/*', async (route) => {
    revokedSessionRequests += 1;
    await route.fulfill({ status: 204 });
  });
  await page.route(apiRouteGlob('sessions'), async (route) => {
    if (route.request().method() === 'DELETE') {
      revokedAllRequests += 1;
      await route.fulfill({ status: 204 });
      return;
    }
    await route.fallback();
  });

  await page.goto('/admin/sessions');
  await expect(page.getByRole('heading', { name: 'Sessions' })).toBeVisible();
  const revokeAll = page.locator('details.mutation-form').filter({ hasText: 'Revoke all sessions' });
  await revokeAll.locator('summary').click();
  await revokeAll.getByLabel('Confirm all session revocation').fill('wrong');
  await revokeAll.getByRole('button', { name: 'Revoke every session' }).click();
  await expect(revokeAll.getByRole('alert')).toContainText('Type REVOKE');
  await expect(page.locator('tbody')).toContainText('00000000-0000-4000-8000-000000000001');
  await expect(page.locator('tbody')).toContainText('(current)');
  await expect(page.getByRole('button', { name: 'Revoke', exact: true }).first()).toBeDisabled();
  await page.getByRole('button', { name: 'Revoke', exact: true }).nth(1).click();
  await expect.poll(() => revokedSessionRequests).toBe(1);
  await expect(page.getByRole('heading', { name: 'Sessions' })).toBeVisible();
  const revokeEverySession = page.locator('details.mutation-form').filter({ hasText: 'Revoke all sessions' });
  await revokeEverySession.locator('summary').click();
  await revokeEverySession.getByLabel('Confirm all session revocation').fill('REVOKE');
  await revokeEverySession.getByRole('button', { name: 'Revoke every session' }).click();
  await expect.poll(() => revokedAllRequests).toBe(1);
});

test('profile displays identity and changes the current administrator password', async ({ page }) => {
  let passwordRequest;
  await page.unroute(apiRouteGlob('change_own_password'));
  await page.route(apiRouteGlob('change_own_password'), async (route) => {
    passwordRequest = route.request().postDataJSON();
    await route.fulfill({ status: 204 });
  });

  await page.goto('/admin/profile');
  await expect(page.getByRole('heading', { name: 'Profile' })).toBeVisible();
  await expect(page.locator('.profile-details')).toContainText('root');
  await expect(page.locator('.profile-details')).toContainText('Root Admin');
  await expect(page.locator('.profile-details')).toContainText('administrator');
  await page.getByLabel('Current password', { exact: true }).fill('CorrectHorseBatteryStaple!');
  await page.getByLabel('Profile new password').fill('NewCorrectHorseBatteryStaple1!');
  await page.getByLabel('Revoke other sessions and all refresh tokens').check();
  await page.getByRole('button', { name: 'Change password' }).click();
  await expect.poll(() => passwordRequest).toEqual({
    current_password: 'CorrectHorseBatteryStaple!',
    new_password: 'NewCorrectHorseBatteryStaple1!',
    revoke_other_sessions: true,
  });
  await expect(page.getByText('Password changed')).toBeVisible();
});

test('profile completes TOTP enrollment and reveals recovery codes once', async ({ page }) => {
  await page.goto('/admin/profile');
  await page.getByLabel('MFA current password').fill('CorrectHorseBatteryStaple!');
  await page.getByRole('button', { name: 'Start new TOTP enrollment' }).click();
  await expect(page.locator('.enrollment-secret')).toContainText('ABCDEFGHIJKLMNOPQRSTUVWXYZ');
  await page.getByLabel('MFA proof').fill('123456');
  await page.getByRole('button', { name: 'Confirm enrollment' }).click();
  await expect(page.locator('.recovery-codes')).toContainText('Save these one-time recovery codes now');
  await expect(page.locator('.recovery-codes')).toContainText('abcd-1234-5678-90ef');
});

test('users permissions and audit keep one header layout and session', async ({ page }) => {
  let meRequests = 0;
  let apiRequests = 0;
  page.on('request', (request) => {
    if (request.url().includes('/api/v1/admin/')) apiRequests += 1;
    if (request.url().includes(apiRoute('me'))) meRequests += 1;
  });

  await page.goto('/admin/users');
  await expect(page.locator('header.topbar')).toBeVisible();
  await expect(page.locator('header.sidebar')).toHaveCount(0);

  await page.getByRole('link', { name: 'Permissions' }).click();
  await expect(page).toHaveURL('/admin/permissions');
  await expect(page.getByRole('heading', { name: 'Permissions' })).toBeVisible();
  await expect(page.locator('header.topbar')).toBeVisible();
  await expect(page.locator('header.sidebar')).toHaveCount(0);

  await page.getByRole('link', { name: 'Audit log' }).click();
  await expect(page).toHaveURL('/admin/audit-log');
  await expect(page.getByRole('heading', { name: 'Audit log' })).toBeVisible();
  await expect(page.locator('header.topbar')).toBeVisible();
  await expect(page.locator('header.sidebar')).toHaveCount(0);
  await expect(page.locator('tbody')).toContainText('alpha (#25)');
  await expect(page.locator('tbody')).toContainText('user #25');
  await page.getByText('Event #1').click();
  await expect(page.locator('.audit-event pre')).toContainText('display_name');

  await page.goBack();
  await expect(page).toHaveURL('/admin/permissions');
  await expect(page.getByRole('heading', { name: 'Permissions' })).toBeVisible();
  await page.goForward();
  await expect(page).toHaveURL('/admin/audit-log');
  await expect(page.getByRole('heading', { name: 'Audit log' })).toBeVisible();
  expect(meRequests).toBe(1);
  expect(apiRequests).toBe(7);
});

test('audit filters and keyset cursor are kept in the URL', async ({ page }) => {
  await page.goto('/admin/audit-log');
  await page.getByRole('button', { name: 'Prepare CSV' }).click();
  await expect(page.getByRole('link', { name: 'Download CSV' })).toHaveAttribute('download', 'admin-audit.csv');
  await page.getByLabel('Audit user login').fill('alpha');
  await page.getByRole('button', { name: 'Apply filters' }).click();
  await expect(page).toHaveURL(/user_login=alpha/);
  await expect(page.getByRole('heading', { name: 'Audit log' })).toBeVisible();
  await page.getByRole('button', { name: 'Older events' }).click();
  await expect(page).toHaveURL(/cursor_created_at=/);
  await expect(page).toHaveURL(/cursor_id=1/);
});

test('a stale page response cannot replace the latest navigation', async ({ page }) => {
  await page.unroute(apiRouteGlob('list_permissions'));
  await page.route(apiRouteGlob('list_permissions'), async (route) => {
    await new Promise((resolve) => setTimeout(resolve, 300));
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(permissions) });
  });

  await page.goto('/admin/users');
  await page.getByRole('link', { name: 'Permissions' }).click();
  await page.getByRole('link', { name: 'Audit log' }).click();
  await expect(page).toHaveURL('/admin/audit-log');
  await expect(page.getByRole('heading', { name: 'Audit log' })).toBeVisible();
  await page.waitForTimeout(400);
  await expect(page.getByRole('heading', { name: 'Audit log' })).toBeVisible();
});

test('concurrent unauthorized requests join one refresh without freezing', async ({ page }) => {
  await page.unroute(apiRouteGlob('list_users'));
  await page.unroute(apiRouteGlob('list_permissions'));
  let userRequests = 0;
  let permissionRequests = 0;
  let refreshRequests = 0;
  await page.route(apiRouteGlob('list_users'), async (route) => {
    userRequests += 1;
    await route.fulfill(userRequests === 1
      ? { status: 401, contentType: 'application/problem+json', body: '{}' }
      : { contentType: 'application/json', body: JSON.stringify(users) });
  });
  await page.route(apiRouteGlob('list_permissions'), async (route) => {
    permissionRequests += 1;
    await route.fulfill(permissionRequests === 1
      ? { status: 401, contentType: 'application/problem+json', body: '{}' }
      : { contentType: 'application/json', body: JSON.stringify(permissions) });
  });
  await page.route(apiRouteGlob('refresh'), async (route) => {
    refreshRequests += 1;
    await new Promise((resolve) => setTimeout(resolve, 100));
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(noBody) });
  });

  await page.goto('/admin/users');
  await page.getByRole('link', { name: 'Permissions' }).click();
  await expect(page.getByRole('heading', { name: 'Permissions' })).toBeVisible();
  expect(userRequests).toBe(2);
  expect(permissionRequests).toBe(2);
  expect(refreshRequests).toBe(1);
});

test('create user submits once, preserves typed data and reports success', async ({ page }) => {
  let requests = 0;
  let requestBody;
  await page.route(apiRouteGlob('create_user'), async (route) => {
    requests += 1;
    requestBody = route.request().postDataJSON();
    await new Promise((resolve) => setTimeout(resolve, 100));
    await route.fulfill({ status: 201, contentType: 'application/json', body: '{}' });
  });

  await page.goto('/admin/users');
  const form = page.locator('details.mutation-form').filter({ hasText: 'Create user' }).first();
  await form.locator('summary').click();
  await form.getByLabel('New user login').fill('new_operator');
  await form.getByLabel('New user display name').fill('New Operator');
  await form.getByLabel('New user password').fill('CorrectHorseBatteryStaple1!');
  const submit = form.locator('button[type="submit"]');
  await submit.click();
  await expect(submit).toBeDisabled();
  await expect(submit).toHaveText('Creating...');
  await expect.poll(() => requests).toBe(1);
  expect(requestBody).toEqual({
    display_name: 'New Operator',
    login: 'new_operator',
    password: 'CorrectHorseBatteryStaple1!',
  });
  await expect(page.getByText('User created')).toBeVisible();
});

test('permissions disable unavailable mutations and direct API returns 403', async ({ page }) => {
  await page.unroute(apiRouteGlob('me'));
  await page.route(apiRouteGlob('me'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify({
      ...authenticatedAdmin,
      permissions: authenticatedAdmin.permissions.filter((value) => value === 'users:read'),
    }) });
  });
  await page.route(apiRouteGlob('create_user'), async (route) => {
    await route.fulfill({
      status: 403,
      contentType: 'application/problem+json',
      body: JSON.stringify({ detail: 'permission denied', status: 403, title: 'Forbidden', type: 'about:blank' }),
    });
  });

  await page.goto('/admin/users');
  const form = page.locator('details.mutation-form').filter({ hasText: 'Create user' }).first();
  await form.locator('summary').click();
  await expect(form.getByRole('button', { name: 'Create user' })).toBeDisabled();
  await expect(page.getByRole('button', { name: 'Ban', exact: true }).first()).toBeDisabled();
  const status = await page.evaluate(async (path) => (await fetch(path, {
    method: 'POST', headers: { 'content-type': 'application/json' }, body: '{}',
  })).status, apiRoute('create_user'));
  expect(status).toBe(403);
});

test('mutation errors expose conflict, validation and rate-limit details', async ({ page }) => {
  const responses = [
    [409, 'revision conflict'],
    [422, 'display name is invalid'],
    [429, 'rate limit exceeded; retry later'],
  ];
  await page.route(apiRouteGlob('create_user'), async (route) => {
    const [status, detail] = responses.shift();
    await route.fulfill({
      status,
      contentType: 'application/problem+json',
      headers: status === 429 ? { 'retry-after': '30' } : {},
      body: JSON.stringify({
        detail,
        kind: status === 409 ? 'conflict' : status === 422 ? 'validation' : 'rate_limited',
        request_id: null,
        status,
        violations: [],
      }),
    });
  });

  await page.goto('/admin/users');
  const form = page.locator('details.mutation-form').filter({ hasText: 'Create user' }).first();
  await form.locator('summary').click();
  await form.getByLabel('New user login').fill('new_operator');
  await form.getByLabel('New user display name').fill('New Operator');
  await form.getByLabel('New user password').fill('CorrectHorseBatteryStaple1!');
  for (const detail of ['revision conflict', 'display name is invalid', 'rate limit exceeded']) {
    await form.getByRole('button', { name: 'Create user' }).click();
    await expect(form.getByRole('alert')).toContainText(detail);
    await expect(form.getByLabel('New user login')).toHaveValue('new_operator');
  }
  await expect(form.getByRole('alert')).toContainText('Retry after: 30');
});

test('network failure preserves form values and allows an explicit retry', async ({ page }) => {
  let requests = 0;
  await page.route(apiRouteGlob('create_user'), async (route) => {
    requests += 1;
    if (requests === 1) {
      await route.abort('connectionrefused');
      return;
    }
    await route.fulfill({ status: 201, contentType: 'application/json', body: '{}' });
  });

  await page.goto('/admin/users');
  const form = page.locator('details.mutation-form').filter({ hasText: 'Create user' }).first();
  await form.locator('summary').click();
  await form.getByLabel('New user login').fill('retry_operator');
  await form.getByLabel('New user display name').fill('Retry Operator');
  await form.getByLabel('New user password').fill('CorrectHorseBatteryStaple1!');
  await form.getByRole('button', { name: 'Create user' }).click();
  await expect(form.getByRole('alert')).toContainText('Failed to fetch');
  await expect(form.getByLabel('New user login')).toHaveValue('retry_operator');
  await form.getByRole('button', { name: 'Create user' }).click();
  await expect(page.getByText('User created')).toBeVisible();
  expect(requests).toBe(2);
});

test('mobile layout and keyboard navigation keep basic accessibility invariants', async ({ page }) => {
  await page.setViewportSize({ width: 375, height: 812 });
  await page.goto('/admin/users');
  await expect(page.getByRole('heading', { name: 'Users' })).toBeVisible();
  await page.keyboard.press('Tab');
  await expect(page.locator(':focus')).toBeVisible();
  const violations = await page.evaluate(() => {
    const values = [];
    document.querySelectorAll('input, select, textarea').forEach((element) => {
      if (!element.getAttribute('aria-label') && !element.closest('label')) values.push('unlabelled field');
    });
    document.querySelectorAll('button, a').forEach((element) => {
      if (!element.textContent.trim() && !element.getAttribute('aria-label')) values.push('unnamed action');
    });
    document.querySelectorAll('img').forEach((element) => {
      if (!element.hasAttribute('alt')) values.push('image without alt');
    });
    if (document.documentElement.scrollWidth > document.documentElement.clientWidth) {
      values.push('horizontal page overflow');
    }
    return values;
  });
  expect(violations).toEqual([]);
});
