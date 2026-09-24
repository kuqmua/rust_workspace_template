import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_audit_log_page_renders_generated_rows", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const responsePromise = page.waitForResponse(
    response =>
      new URL(response.url()).pathname === "/audit_log/read" &&
      response.request().method() === "POST"
  );
  await page.goto("/admin/audit_log");
  const response = await responsePromise;
  expect(response.status()).toBe(200);
  await expect(page.locator('section[data-renderer="csr"] table')).toBeVisible();
  await expect(page.locator("tbody tr").first()).toBeVisible();
});

test("test_audit_log_read_opens_details_in_the_table", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/audit_log");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(10);
  const values = await cells.allTextContents();
  await row.getByRole("button", { name: "read", exact: true }).click();
  await expect(page).toHaveURL("/admin/audit_log");
  const detail = row.getByRole("dialog", { name: "read" });
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 9));
  await detail.getByRole("button", { name: "close" }).click();
  await expect(detail).not.toBeVisible();
});

test("test_audit_log_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/audit_log/1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
