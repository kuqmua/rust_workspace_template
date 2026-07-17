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
      recent_changes: auditLog.items,
      uptime_seconds: 3600,
      version: '0123456789abcdef',
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

test('sessions page lists and revokes current administrator sessions', async ({ page }) => {
  let revokedSessionRequests = 0;
  await page.route('**/api/v1/admin/auth/sessions/*', async (route) => {
    revokedSessionRequests += 1;
    await route.fulfill({ status: 204 });
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
  await page.getByLabel('Current password').fill('CorrectHorseBatteryStaple!');
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
