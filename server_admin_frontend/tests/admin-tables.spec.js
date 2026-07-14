const { test, expect } = require('@playwright/test');

const users = Array.from({ length: 25 }, (_, index) => ({
  display_name: index === 24 ? 'Alpha Operator' : `User ${String(index + 1).padStart(2, '0')}`,
  id: index + 1,
  is_banned: index % 2 === 0,
  login: index === 24 ? 'alpha' : `user_${String(index + 1).padStart(2, '0')}`,
}));

test.beforeEach(async ({ page }) => {
  await page.route('**/api/v1/admin/auth/me', async (route) => {
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify({
        display_name: 'Root Admin',
        id: 1,
        login: 'root',
        permissions: ['users:read', 'openapi:read'],
        roles: ['administrator'],
      }),
    });
  });
  await page.route('**/api/v1/admin/users', async (route) => {
    await route.fulfill({ contentType: 'application/json', body: JSON.stringify(users) });
  });
  await page.route('**/api/v1/admin/openapi.json', async (route) => {
    await route.fulfill({
      contentType: 'application/json',
      body: JSON.stringify({ openapi: '3.1.0', paths: { '/users': { get: {} } } }),
    });
  });
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
