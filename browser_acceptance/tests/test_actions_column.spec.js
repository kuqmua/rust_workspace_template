import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset, signOutIfAuthenticated } from "./support/admin.js";
import { dataTablePages, tablePages } from "./support/pages.js";

test("test_data_tables_have_consistent_actions_column", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  try {
    const pages = [...tablePages.filter(value => value.name !== "sessions"), ...dataTablePages];
    const readPaths = new Set([
      "/admin/users",
      "/admin/roles",
      "/admin/rules",
      "/admin/permission_actions",
      "/admin/permission_resource_actions",
      "/admin/permission_resources",
      "/admin/user_roles",
      "/admin/role_rules",
      "/admin/refresh_tokens",
      "/admin/access_sessions",
      "/admin/login_attempts",
      "/admin/audit_log",
      "/admin/cleanup_status",
      "/admin/system_settings",
      "/admin/rate_limits"
    ]);
    await pages.reduce(async (previous, { path }) => {
      await previous;
      await page.goto(path);
      const table = page.locator('section[data-renderer="csr"] table');
      await expect(table.locator("thead th").last()).toHaveText("actions");
      const valid = await table.evaluate(element => {
        const columns = element.querySelectorAll("thead th").length;
        return [...element.querySelectorAll("tbody tr")].every(row => {
          const cell = row.lastElementChild;
          return row.children.length === columns
            && cell.dataset.label === "actions";
        });
      });
      expect(valid, path).toBe(true);
      const rows = table.locator("tbody tr");
      const readButtons = table.locator('tbody td[data-label="actions"]').getByRole("button", { name: "read", exact: true });
      if (readPaths.has(path)) {
        await expect(readButtons, path).toHaveCount(await rows.count());
      } else {
        await expect(readButtons).toHaveCount(0);
        await expect(table.locator('td[data-label="actions"] button, td[data-label="actions"] input')).toHaveCount(0);
      }
    }, Promise.resolve());
  } finally {
    await signOutIfAuthenticated(page);
  }
});
