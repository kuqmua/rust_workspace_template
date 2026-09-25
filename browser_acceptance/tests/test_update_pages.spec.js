import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_users_and_roles_update_pages_submit_existing_records", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);

  const userRow = page.locator("tbody tr").first();
  const userId = (await userRow.locator('td[data-label="id"]').textContent()).trim();
  const login = (await userRow.locator('td[data-label="login"]').textContent()).trim();
  await userRow.locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }).click();
  await expect(page).toHaveURL(/\/admin\/users\/update$/);
  const userForm = page.locator("form.user-update-form");
  await expect(userForm.locator('input[name="user_id"]')).toBeVisible();
  await userForm.locator('input[name="user_id"]').fill(userId);
  await userForm.locator('input[name="login"]').fill(login);
  await userForm.locator('input[name="display_name"]').fill("Administrator Updated From Page");
  await userForm.getByRole("button", { name: "update" }).click();
  await expect(page).toHaveURL(/\/admin\/users#saved$/);
  await page.goto("/admin/users");
  await expect(page.locator("tbody tr").first()).toContainText("Administrator Updated From Page");

  const created = await page.request.post("/roles/create", {
    data: [{ name: "role_update_page_fixture" }],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  const [roleId] = await created.json();
  await page.goto("/admin/roles");
  const roleRow = page.locator("tbody tr").filter({
    has: page.locator('td[data-label="id"]').filter({ hasText: new RegExp(`^${roleId}$`) })
  });
  await roleRow.locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }).click();
  await expect(page).toHaveURL(/\/admin\/roles\/update$/);
  const roleForm = page.locator("form.role-update-form");
  await expect(roleForm.locator('input[name="role_id"]')).toBeVisible();
  await roleForm.locator('input[name="role_id"]').fill(String(roleId));
  await roleForm.locator('input[name="name"]').fill("role_updated_from_page");
  await roleForm.getByRole("button", { name: "update" }).click();
  await expect(page).toHaveURL(/\/admin\/roles#saved$/);
  await page.goto("/admin/roles");
  await expect(page.locator("tbody tr").filter({ hasText: "role_updated_from_page" })).toHaveCount(1);
});

test("test_update_pages_require_authentication", async ({ page }) => {
  await page.goto("/admin/users/update");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
  await page.goto("/admin/roles/update");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});

test("test_update_pages_require_identifiers_and_new_values", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const [path, formClass, identifier, value] of [
    ["/admin/users/update", "user-update-form", "user_id", "display_name"],
    ["/admin/roles/update", "role-update-form", "role_id", "name"]
  ]) {
    await page.goto(path);
    const form = page.locator(`form.${formClass}`);
    const identifierInput = form.locator(`input[name="${identifier}"]`);
    const valueInput = form.locator(`input[name="${value}"]`);
    await expect(identifierInput).toHaveAttribute("required", "");
    await expect(valueInput).toHaveAttribute("required", "");
    await form.getByRole("button", { name: "update" }).click();
    await expect(page).toHaveURL(path);
    expect(await identifierInput.evaluate(input => input.validity.valueMissing)).toBe(true);
  }
});
