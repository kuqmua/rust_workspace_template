import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset, signOutIfAuthenticated } from "./support/admin.js";

test("test_permission_tables_read_actions_open_matching_detail_pages", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  try {
    await [
      ["permission_actions", "permission-action-read"],
      ["permission_resource_actions", "permission-resource-action-read"],
      ["permission_resources", "permission-resource-read"]
    ].reduce(async (previous, [table, detailPage]) => {
      await previous;
      await page.goto(`/admin/${table}`);
      const row = page.locator("tbody tr").first();
      await expect(row).toBeVisible();
      const cells = await row.locator("td:not(:last-child)").allTextContents();
      const identifier = cells[0].trim();
      const path = `/admin/${table}/${identifier}`;
      const read = row.locator('td[data-label="actions"]').getByRole("button", { name: "read", exact: true });
      await expect(read).toBeVisible();
      await read.click();
      await expect(page).toHaveURL(`/admin/${table}`);
      const dialog = row.getByRole("dialog", { name: "read" });
      await expect(dialog.locator(".health-result")).toHaveText(cells.map(value => value.trim()));
      await dialog.getByRole("button", { name: "close" }).click();
      await page.goto(path);
      await expect(page).toHaveURL(new RegExp(`${path}$`));
      const detail = page.locator(`[data-page="${detailPage}"]`);
      await expect(detail.locator(".health-result")).toHaveText(cells.map(value => value.trim()));
      await page.goto(`${path}?offset=999&sort=missing`);
      await expect(detail.locator(".health-result")).toHaveText(cells.map(value => value.trim()));
      await page.goto(`/admin/${table}/9223372036854775807`);
      await expect(page.locator(`[data-page="${detailPage}"]`)).toContainText("resource not found");
    }, Promise.resolve());
  } finally {
    await signOutIfAuthenticated(page);
  }
});
