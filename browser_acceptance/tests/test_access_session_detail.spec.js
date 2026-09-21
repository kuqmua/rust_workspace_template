import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_access_session_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/access_sessions");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(6);
  const values = await cells.allTextContents();
  const path = `/admin/access_sessions/${values[0].trim()}`;
  const link = row.getByRole("link", { name: "read", exact: true });
  await expect(link).toHaveAttribute("href", path);
  await link.click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="access-session-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await expect(detail).not.toContainText("token_identifier_hash");
  await expect(detail).not.toContainText("csrf_token_hash");
  await expect(detail).not.toContainText("token_context_hash");
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto(`${path}?search=missing&offset=999&filter_field=user_id&filter_operation=eq&filter_value=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto("/admin/access_sessions/ffffffff-ffff-4fff-bfff-ffffffffffff");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_access_session_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/access_sessions/123e4567-e89b-42d3-a456-426614174000");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
