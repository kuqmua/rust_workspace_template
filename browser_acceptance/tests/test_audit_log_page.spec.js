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

test("test_audit_log_read_navigates_to_record_details", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/audit_log");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(10);
  const values = await cells.allTextContents();
  const path = `/admin/audit_log/${values[0].trim()}/read`;
  await row.getByRole("link", { name: "read", exact: true }).click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="audit_log-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 9));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 9));
  await page.goto(`${path}?search=missing&offset=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 9));
  await page.goto("/admin/audit_log/9223372036854775807/read");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});
