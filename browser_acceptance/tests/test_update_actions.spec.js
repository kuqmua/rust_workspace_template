import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_user_row_update_actions_navigate_to_update_page", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/users");

  const rows = page.locator('section[data-renderer="csr"] tbody tr');
  await expect(rows.first()).toBeVisible();
  await expect(page.locator('section[data-renderer="csr"] dialog[aria-label="update"]')).toHaveCount(0);
  for (const row of await rows.all()) {
    await expect(row.locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }))
      .toHaveAttribute("href", "/admin/users/update");
  }

  await rows.first().locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }).click();
  await expect(page).toHaveURL("http://127.0.0.1:18080/admin/users/update");
  await expect(page.locator("form.user-update-form")).toBeVisible();
});

test("test_role_row_update_actions_navigate_to_update_page", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/roles");

  const rows = page.locator('section[data-renderer="csr"] tbody tr');
  await expect(rows.first()).toBeVisible();
  await expect(page.locator('section[data-renderer="csr"] dialog[aria-label="update"]')).toHaveCount(0);
  for (const row of await rows.all()) {
    await expect(row.locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }))
      .toHaveAttribute("href", "/admin/roles/update");
  }

  await rows.first().locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }).click();
  await expect(page).toHaveURL("http://127.0.0.1:18080/admin/roles/update");
  await expect(page.locator("form.role-update-form")).toBeVisible();
});
