import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_refresh_token_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/refresh_tokens");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(6);
  const values = await cells.allTextContents();
  const path = `/admin/refresh_tokens/${values[0].trim()}`;
  await page.goto(path);
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="refresh-token-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await expect(detail).not.toContainText("token_hash");
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto(`${path}?search=missing&offset=999&filter_field=user_id&filter_operation=eq&filter_value=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto("/admin/refresh_tokens/ffffffff-ffff-4fff-bfff-ffffffffffff");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_refresh_token_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/refresh_tokens/67e55044-10b1-426f-9247-bb680e5fe0c8");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
