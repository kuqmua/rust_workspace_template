import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_session_read_link_opens_separate_record_page", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/sessions");
  const row = page.locator("tbody tr").first();
  await expect(row).toBeVisible();
  const identifier = (await row.locator("td").first().textContent()).trim();
  const link = row.getByRole("link", { name: "read", exact: true });
  expect(identifier).toMatch(/^[1-9][0-9]*$/);
  const path = `/admin/sessions/${identifier}/read`;
  await expect(link).toHaveAttribute("href", path);
  await link.click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const values = page.locator('[data-page="session-read"] .health-result');
  await expect(values).toHaveCount(4);
  await expect(values.first()).toHaveText(identifier);
  const expected = await values.allTextContents();
  await page.reload();
  await expect(values).toHaveText(expected);
  await page.goto(`${path}?limit=0&offset=-1&limit=invalid&filter_value=invalid`);
  await expect(values).toHaveText(expected);
});

test("test_unknown_session_read_does_not_show_another_record", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/sessions/9223372036854775807/read");
  const record = page.locator('[data-page="session-read"]');
  await expect(record).toBeVisible();
  await expect(record.locator(".health-result")).toHaveCount(0);
  await expect(record).toContainText("resource");
});
