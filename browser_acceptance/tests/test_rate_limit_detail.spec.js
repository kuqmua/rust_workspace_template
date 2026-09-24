import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_rate_limit_details_follow_table_link", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/rate_limits");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(6);
  const values = await cells.allTextContents();
  const identifier = values[0];
  const path = `/admin/rate_limits/${identifier}`;
  const detailResponsePromise = page.waitForResponse((response) =>
    new URL(response.url()).pathname.endsWith("/rate_limits/read"),
  );
  await page.goto(path);
  const detailResponseUrl = new URL((await detailResponsePromise).url());
  expect(detailResponseUrl.searchParams.get("filter_field")).toBe("id");
  expect(detailResponseUrl.searchParams.get("filter_operation")).toBe("eq");
  expect(detailResponseUrl.searchParams.get("filter_value")).toBe(identifier);
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="rate-limit-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto("/admin/rate_limits/9223372036854775807");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_rate_limit_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/rate_limits/1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
