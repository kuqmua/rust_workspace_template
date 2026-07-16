const { test, expect } = require('@playwright/test');
const [
  routeCatalog,
  permissionCatalog,
  authenticatedAdmin,
  users,
  permissions,
  auditLog,
  noBody,
] = require('../../target/admin_contract_fixture.json');

const apiRoute = (operationId) => {
  const route = routeCatalog.find(([candidate]) => candidate === operationId);
  if (!route) throw new Error(`missing generated route fixture for ${operationId}`);
  return `/api/v1/admin${route[2]}`;
};
const apiRouteGlob = (operationId) => `**${apiRoute(operationId)}`;

test.beforeEach(async ({ page }) => {
  await page.route(apiRouteGlob('me'), async (route) => {
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify(authenticatedAdmin),
    });
  });
  await page.route(apiRouteGlob('list_users'), async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(users) });
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
  await expect(page.getByRole('button', { name: 'Sign in' })).toBeVisible();
  await expect(page.locator('link[rel="modulepreload"]')).toHaveAttribute(
    'href',
    /server_admin_frontend-[a-f0-9]+\.js$/,
  );
  await page.waitForTimeout(100);
  expect(authenticatedRequests).toEqual([]);
  expect(pageErrors).toEqual([]);
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
  await expect(page.locator('tbody tr')).toHaveCount(5);
  await expect(page.getByText('21-25 of 25')).toBeVisible();
  await page.getByRole('button', { name: 'Previous page' }).click();

  await page.getByRole('searchbox', { name: 'Filter rows' }).fill('alpha');
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

  await page.goBack();
  await expect(page).toHaveURL('/admin/permissions');
  await expect(page.getByRole('heading', { name: 'Permissions' })).toBeVisible();
  await page.goForward();
  await expect(page).toHaveURL('/admin/audit-log');
  await expect(page.getByRole('heading', { name: 'Audit log' })).toBeVisible();
  expect(meRequests).toBe(1);
  expect(apiRequests).toBe(6);
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
