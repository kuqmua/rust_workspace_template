import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset, signOutIfAuthenticated } from "./support/admin.js";

test("test_roles_and_personal_sessions_use_the_shared_data_grid", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  try {
    await page.goto("/admin/roles");
    const roles = page.locator('section[data-renderer="csr"].table-page.table-admin_roles_page');
    await expect(roles).toBeVisible();
    await expect(roles.locator('th[data-field]')).toHaveCount(5);
    await expect(roles.getByRole("link", { name: "create" })).toHaveAttribute("href", "/admin/roles/create");
    const roleFilter = roles.locator('th[data-field="is_system"] .table-column-filter');
    await roleFilter.getByRole("button", { name: "filter_is_system" }).click();
    await expect(roleFilter.getByRole("radio", { name: "eq", exact: true })).toBeChecked();
    await roleFilter.getByRole("button", { name: "close", exact: true }).click();

    await page.goto("/admin/sessions");
    const sessions = page.locator('section[data-renderer="csr"].table-page.table-admin_sessions_page');
    await expect(sessions).toBeVisible();
    await expect(sessions.locator('th[data-field]')).toHaveCount(4);
    await expect(sessions.locator('form.table-page-size')).toHaveAttribute("action", "/admin/sessions");
    const currentFilter = sessions.locator('th[data-field="current"] .table-column-filter');
    await currentFilter.getByRole("button", { name: "filter_current" }).click();
    await expect(currentFilter.getByRole("radio", { name: "eq", exact: true })).toBeChecked();
  } finally {
    await signOutIfAuthenticated(page);
  }
});
