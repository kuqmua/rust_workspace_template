import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_rule_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/rules");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(14);
  const values = (await cells.allTextContents()).slice(0, 13);
  const path = `/admin/rule/${values[0].trim()}/read`;
  const read = cells.last().getByRole("link", { name: "read", exact: true });
  await expect(read).toHaveAttribute("href", path);
  await read.click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="rule-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values);
  await page.goto(`/admin/rules/${values[0].trim()}/read`);
  await expect(detail.locator(".health-result")).toHaveText(values);
  await page.goto(`/admin/rules/${values[0].trim()}`);
  await expect(detail.locator(".health-result")).toHaveText(values);
  await page.goto(path);
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values);
  await page.goto(`${path}?search=missing&offset=999&filter_field=id&filter_operation=eq&filter_value=999`);
  await expect(detail.locator(".health-result")).toHaveText(values);
  await page.goto("/admin/rules/9223372036854775807/read");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
  await page.goto("/admin/rules/1/read");
  await expect(detail.locator(".health-result")).toHaveCount(13);
  await expect(detail.locator(".health-result").first()).toHaveText("1");
  await page.goto("/admin/rule/1/read");
  await expect(detail.locator(".health-result").first()).toHaveText("1");
});

test("test_rule_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/rules/1/read");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});


test("test_rule_read_routes_reject_invalid_identifiers", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const resource of ["rules", "rule"]) {
    for (const id of ["0", "-1", "invalid", "9223372036854775808"]) {
      const response = await page.goto(`/admin/${resource}/${id}/read`);
      expect(response.status()).toBe(422);
    }
  }
});
