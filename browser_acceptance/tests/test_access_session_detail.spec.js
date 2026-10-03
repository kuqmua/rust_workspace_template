import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_access_session_read_opens_matching_detail_page", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/access_sessions");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(6);
  const values = await cells.allTextContents();
  const path = `/admin/access_sessions/${values[0].trim()}/read`;
  const read = row.getByRole("link", { name: "read", exact: true });
  await expect(read).toHaveAttribute("href", path);
  await read.click();
  await expect(page).toHaveURL(path);
  const detail = page.locator('[data-page="access-session-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await expect(detail).not.toContainText("token_identifier_hash");
  await expect(detail).not.toContainText("csrf_token_hash");
  await expect(detail).not.toContainText("token_context_hash");
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto(`/admin/access_sessions/${values[0].trim()}`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto(`${path}?search=missing&offset=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto("/admin/access_sessions/ffffffff-ffff-4fff-bfff-ffffffffffff/read");
  await expect(detail).toContainText("resource not found");
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
