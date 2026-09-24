import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_rule_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/rules");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(14);
  const values = (await cells.allTextContents()).slice(0, 13);
  const path = `/admin/rules/${values[0].trim()}`;
  const read = cells.last().getByRole("button", { name: "read", exact: true });
  await read.click();
  await expect(page).toHaveURL("/admin/rules");
  const dialog = row.getByRole("dialog", { name: "read" });
  await expect(dialog.locator(".health-result")).toHaveText(values);
  await dialog.getByRole("button", { name: "close" }).click();
  await page.goto(path);
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="rule-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values);
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values);
  await page.goto(`${path}?search=missing&offset=999&filter_field=id&filter_operation=eq&filter_value=999`);
  await expect(detail.locator(".health-result")).toHaveText(values);
  await page.goto("/admin/rules/9223372036854775807");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_rule_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/rules/1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
