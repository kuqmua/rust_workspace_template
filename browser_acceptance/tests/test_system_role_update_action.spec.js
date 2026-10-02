import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_system_and_custom_roles_have_update_actions", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/roles");
  const systemRole = page.getByRole("row").filter({
    has: page.getByRole("button", { name: "true", exact: true })
  });
  await expect(systemRole).toHaveCount(1);
  const systemUpdate = systemRole.getByRole("link", { name: "update", exact: true });
  await expect(systemUpdate).toHaveAttribute("href", "/admin/roles/1/update");
  await systemUpdate.click();
  await expect(page).toHaveURL("/admin/roles/1/update");
  await expect(page.locator('form.role-update-form input[name="role_id"]')).toHaveValue("1");
  await page.goto("/admin/roles");

  const created = await page.request.post("/roles/create", {
    data: [{ name: "editable_role_fixture" }],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  const [roleId] = await created.json();
  await page.reload();
  const customRole = page.getByRole("row").filter({
    has: page.getByRole("button", { name: "editable_role_fixture", exact: true })
  });
  await expect(customRole.locator(`a[href="/admin/roles/${roleId}/update"]`)).toHaveCount(1);
});
