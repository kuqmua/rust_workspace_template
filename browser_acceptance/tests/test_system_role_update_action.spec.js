import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_system_role_has_no_update_action", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/roles");
  const systemRole = page.getByRole("row").filter({
    has: page.getByRole("button", { name: "true", exact: true })
  });
  await expect(systemRole).toHaveCount(1);
  await expect(systemRole.locator('a[href="/admin/roles/update"]')).toHaveCount(0);

  const created = await page.request.post("/roles/create", {
    data: [{ name: "editable_role_fixture" }],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  await page.reload();
  const customRole = page.getByRole("row").filter({
    has: page.getByRole("button", { name: "editable_role_fixture", exact: true })
  });
  await expect(customRole.locator('a[href="/admin/roles/update"]')).toHaveCount(1);
});
