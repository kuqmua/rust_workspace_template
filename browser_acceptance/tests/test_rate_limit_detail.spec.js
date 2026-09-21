import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_rate_limit_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/rate_limits");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(5);
  const values = await cells.allTextContents();
  const path = `/admin/rate_limits/${values[0].trim()}/${values[1].trim()}`;
  const link = row.getByRole("link", { name: "read", exact: true });
  await expect(link).toHaveAttribute("href", path);
  await link.click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="rate_limit_read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.goto(`${path}?search=missing&offset=999&filter_field=scope&filter_operation=eq&filter_value=missing`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.goto("/admin/rate_limits/sign_in_ip_login/127.0.0.1%7Cadmin");
  await expect(detail.locator(".health-result").nth(0)).toHaveText("sign_in_ip_login");
  await expect(detail.locator(".health-result").nth(1)).toHaveText("127.0.0.1|admin");
  await page.goto("/admin/rate_limits/sign_in_ip/missing");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_rate_limit_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/rate_limits/sign_in_ip/127.0.0.1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
