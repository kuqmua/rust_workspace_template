import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_system_setting_read_navigates_to_record_details", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/system_settings");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(11);
  const values = await cells.allTextContents();
  const path = `/admin/system_settings/${values[0].trim()}/read`;
  await row.getByRole("link", { name: "read", exact: true }).click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="system_settings-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 10));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 10));
  await page.goto(`${path}?search=missing&offset=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 10));
  await page.goto("/admin/system_settings/32767/read");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});
