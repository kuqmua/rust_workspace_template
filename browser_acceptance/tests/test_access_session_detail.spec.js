import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_access_session_read_opens_details_in_the_table", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/access_sessions");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(6);
  const values = await cells.allTextContents();
  await row.getByRole("button", { name: "read", exact: true }).click();
  await expect(page).toHaveURL("/admin/access_sessions");
  const detail = row.getByRole("dialog", { name: "read" });
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await expect(detail).not.toContainText("token_identifier_hash");
  await expect(detail).not.toContainText("csrf_token_hash");
  await expect(detail).not.toContainText("token_context_hash");
  await detail.getByRole("button", { name: "close" }).click();
  await expect(detail).not.toBeVisible();
});

test("test_access_session_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/access_sessions/123e4567-e89b-42d3-a456-426614174000");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});

test("test_access_session_datetime_filter_reaches_read_request", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const request = page.waitForRequest(candidate => new URL(candidate.url()).pathname === "/access_sessions/read", { timeout: 10000 });
  const response = page.waitForResponse(candidate => new URL(candidate.url()).pathname === "/access_sessions/read", { timeout: 10000 });
  await page.goto("/admin/access_sessions?filter_field=created_at&filter_operation=eq&filter_value=2026-01-01T00%3A00");
  const body = (await request).postDataJSON();
  expect(body.where_many?.created_at?.values?.[0]?.Eq?.values).toMatchObject({ date_naive: "2026-01-01" });
  expect((await response).status()).toBe(200);
});
