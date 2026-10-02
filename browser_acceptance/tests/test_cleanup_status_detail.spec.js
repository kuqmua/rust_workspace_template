import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_cleanup_status_details_follow_table_link", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/cleanup_status");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(5);
  const values = await cells.allTextContents();
  const identifier = values[0].trim();
  const detailResponsePromise = page.waitForResponse((response) =>
    new URL(response.url()).pathname.endsWith("/cleanup_status/read"),
  );
  await row.getByRole("link", { name: "read", exact: true }).click();
  const detailResponseUrl = new URL((await detailResponsePromise).url());
  expect(detailResponseUrl.searchParams.get("filter_field")).toBe("id");
  expect(detailResponseUrl.searchParams.get("filter_operation")).toBe("eq");
  expect(detailResponseUrl.searchParams.get("filter_value")).toBe(identifier);
  await expect(page).toHaveURL(new RegExp(`/admin/cleanup_status/${identifier}/read$`));
  const detail = page.locator('[data-page="cleanup-status-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.goto(`/admin/cleanup_status/${identifier}/read?search=missing&offset=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.goto("/admin/cleanup_status/9223372036854775807/read");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_cleanup_status_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/cleanup_status/1/read");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
