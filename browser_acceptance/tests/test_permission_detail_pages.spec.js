import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset, signOutIfAuthenticated } from "./support/admin.js";

test("test_permission_tables_read_actions_open_matching_detail_pages", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  try {
    await [
      ["permission_actions", "permission-action-read"],
      ["permission_resource_actions", "permission-resource-action-read"],
      ["permission_resources", "permission-resource-read"]
    ].reduce(async (previous, [table, detailPage]) => {
      await previous;
      await page.goto(`/admin/${table}`);
      const row = page.locator("tbody tr").first();
      await expect(row).toBeVisible();
      const cells = await row.locator("td:not(:last-child)").allTextContents();
      const identifier = cells[0].trim();
      const path = `/admin/${table}/${identifier}/read`;
      const read = row.locator('td[data-label="actions"]').getByRole("link", { name: "read", exact: true });
      await expect(read).toHaveAttribute("href", path);
      await read.click();
      await expect(page).toHaveURL(new RegExp(`${path}$`));
      const detail = page.locator(`[data-page="${detailPage}"]`);
      await expect(detail.locator(".health-result")).toHaveText(cells.map(value => value.trim()));
      await page.goto(`${path}?offset=999&sort=missing`);
      await expect(detail.locator(".health-result")).toHaveText(cells.map(value => value.trim()));
      await page.goto(`/admin/${table}/9223372036854775807`);
      await expect(page.locator(`[data-page="${detailPage}"]`)).toContainText("resource not found");
    }, Promise.resolve());
  } finally {
    await signOutIfAuthenticated(page);
  }
});

test("test_permission_table_filters_reach_read_requests", async ({ page }) => {
  test.setTimeout(45000);
  await signInAdministratorWithPasswordReset(page);
  try {
    await [
      ["permission_actions", "key", "read"],
      ["permission_resource_actions", "permission_action_id", "1"],
      ["permission_resources", "key", "read"],
    ].reduce(async (previous, [table, field, value]) => {
      await previous;
      const request = page.waitForRequest(candidate => new URL(candidate.url()).pathname === `/${table}/read`, { timeout: 10000 });
      const response = page.waitForResponse(candidate => new URL(candidate.url()).pathname === `/${table}/read`, { timeout: 10000 });
      await page.goto(`/admin/${table}?filter_field=${field}&filter_operation=eq&filter_value=${value}`);
      const body = (await request).postDataJSON();
      expect(body.where_many?.[field], table).toBeTruthy();
      expect((await response).status(), table).toBe(200);
    }, Promise.resolve());
  } finally {
    await signOutIfAuthenticated(page);
  }
});


test("test_permission_action_one_read_and_legacy_routes_show_same_record", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/permission_actions/1/read");
  const values = page.locator('[data-page="permission-action-read"] .health-result');
  await expect(values.first()).toHaveText("1");
  const expected = await values.allTextContents();
  await page.reload();
  await expect(values).toHaveText(expected);
  await page.goto("/admin/permission_actions/1");
  await expect(values).toHaveText(expected);
});

test("test_permission_action_read_requires_authentication", async ({ page }) => {
  await page.goto("/admin/permission_actions/1/read");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});

test("test_permission_action_read_rejects_invalid_identifiers", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const id of ["0", "-1", "invalid", "9223372036854775808"]) {
    const response = await page.goto(`/admin/permission_actions/${id}/read`);
    expect(response.status()).toBe(422);
  }
});


test("test_permission_resource_action_one_read_and_legacy_routes_show_same_record", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/permission_resource_actions/1/read");
  const values = page.locator('[data-page="permission-resource-action-read"] .health-result');
  await expect(values.first()).toHaveText("1");
  const expected = await values.allTextContents();
  await page.reload();
  await expect(values).toHaveText(expected);
  await page.goto("/admin/permission_resource_actions/1");
  await expect(values).toHaveText(expected);
});

test("test_permission_resource_action_read_requires_authentication", async ({ page }) => {
  await page.goto("/admin/permission_resource_actions/1/read");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});

test("test_permission_resource_action_read_rejects_invalid_identifiers", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const id of ["0", "-1", "invalid", "9223372036854775808"]) {
    const response = await page.goto(`/admin/permission_resource_actions/${id}/read`);
    expect(response.status()).toBe(422);
  }
});


test("test_permission_resource_one_read_and_legacy_routes_show_same_record", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/permission_resources/1/read");
  const values = page.locator('[data-page="permission-resource-read"] .health-result');
  await expect(values.first()).toHaveText("1");
  const expected = await values.allTextContents();
  await page.reload();
  await expect(values).toHaveText(expected);
  await page.goto("/admin/permission_resources/1");
  await expect(values).toHaveText(expected);
});

test("test_permission_resource_read_requires_authentication", async ({ page }) => {
  await page.goto("/admin/permission_resources/1/read");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});

test("test_permission_resource_read_rejects_invalid_identifiers", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const id of ["0", "-1", "invalid", "9223372036854775808"]) {
    const response = await page.goto(`/admin/permission_resources/${id}/read`);
    expect(response.status()).toBe(422);
  }
});
