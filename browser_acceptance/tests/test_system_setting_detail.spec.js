import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_system_setting_read_opens_details_in_the_table", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/system_settings");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(11);
  const values = await cells.allTextContents();
  await row.getByRole("button", { name: "read", exact: true }).click();
  await expect(page).toHaveURL("/admin/system_settings");
  const detail = row.getByRole("dialog", { name: "read" });
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 10));
  await detail.getByRole("button", { name: "close" }).click();
  await expect(detail).not.toBeVisible();
});

test("test_system_setting_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/system_settings/1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
