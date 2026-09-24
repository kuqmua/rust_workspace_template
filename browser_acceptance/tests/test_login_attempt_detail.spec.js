import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_login_attempt_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/login_attempts");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(6);
  const values = await cells.allTextContents();
  const path = `/admin/login_attempts/${values[0].trim()}`;
  await page.goto(path);
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="login-attempt-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto(`${path}?search=missing&offset=999&filter_field=login&filter_operation=eq&filter_value=missing`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto("/admin/login_attempts/9223372036854775807");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_login_attempt_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/login_attempts/1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
