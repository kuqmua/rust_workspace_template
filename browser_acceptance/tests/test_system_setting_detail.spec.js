import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_system_setting_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/system_settings");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(11);
  const values = await cells.allTextContents();
  const path = `/admin/system_settings/${values[0].trim()}`;
  const link = row.getByRole("link", { name: "read", exact: true });
  await expect(link).toHaveAttribute("href", path);
  await link.click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="system_settings_read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 10));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 10));
  await page.goto(`${path}?search=missing&offset=999&filter_field=id&filter_operation=eq&filter_value=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 10));
  await page.goto("/admin/system_settings/9223372036854775807");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_system_setting_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/system_settings/1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
