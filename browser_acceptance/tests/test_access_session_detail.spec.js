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
