import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_users_and_roles_update_pages_submit_existing_records", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);

  const userRow = page.locator("tbody tr").first();
  const userId = (await userRow.locator('td[data-label="id"]').textContent()).trim();
  const login = (await userRow.locator('td[data-label="login"]').textContent()).trim();
  await userRow.locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }).click();
  await expect(page).toHaveURL(`/admin/users/${userId}/update`);
  const userForm = page.locator("form.user-update-form");
  await expect(userForm.locator('input[name="user_id"]')).toBeHidden();
  await expect(userForm.locator('input[name="user_id"]')).toHaveValue(userId);
  await expect(userForm.locator('input[name="user_id"]')).toHaveAttribute("type", "hidden");
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
  await page.goto("/admin/users/1/update");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
  await page.goto("/admin/roles/update");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});

test("test_update_pages_require_identifiers_and_new_values", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const [path, formClass, identifier, value] of [
    ["/admin/users/1/update", "user-update-form", "user_id", "display_name"],
    ["/admin/roles/update", "role-update-form", "role_id", "name"]
  ]) {
    await page.goto(path);
    const form = page.locator(`form.${formClass}`);
    const identifierInput = form.locator(`input[name="${identifier}"]`);
    const valueInput = form.locator(`input[name="${value}"]`);
    if (identifier === "user_id") {
      await expect(identifierInput).toBeHidden();
      await expect(identifierInput).toHaveValue("1");
    } else {
      await expect(identifierInput).toHaveAttribute("required", "");
    }
    await expect(valueInput).toHaveAttribute("required", "");
    await form.getByRole("button", { name: "update" }).click();
    await expect(page).toHaveURL(path);
    expect(await valueInput.evaluate(input => input.validity.valueMissing)).toBe(true);
  }
});


test("test_user_update_rejects_a_different_form_identifier", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const row = page.locator("tbody tr").first();
  const id = (await row.locator('td[data-label="id"]').textContent()).trim();
  const login = (await row.locator('td[data-label="login"]').textContent()).trim();
  const displayName = (await row.locator('td[data-label="display_name"]').textContent()).trim();
  await row.getByRole("link", { name: "update", exact: true }).click();
  const form = page.locator("form.user-update-form");
  await expect(form).toHaveAttribute("action", `/admin/actions/users/${id}/update`);
  await form.locator('input[name="user_id"]').evaluate(input => { input.value = String(Number(input.value) + 1); });
  await form.locator('input[name="login"]').fill(login);
  await form.locator('input[name="display_name"]').fill("Rejected wrong identifier");
  const response = page.waitForResponse(response => response.request().method() === "POST" && new URL(response.url()).pathname === `/admin/actions/users/${id}/update`);
  await form.getByRole("button", { name: "update", exact: true }).click();
  expect((await response).status()).toBe(422);
  await page.goto("/admin/users");
  await expect(page.locator("tbody tr").filter({ has: page.locator('td[data-label="id"]').filter({ hasText: new RegExp(`^${id}$`) }) }).locator('td[data-label="display_name"]')).toHaveText(displayName);
});

test("test_user_update_page_rejects_invalid_identifiers", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  for (const id of ["0", "-1", "invalid", "9223372036854775808"]) {
    const response = await page.goto(`/admin/users/${id}/update`);
    expect(response.status()).toBe(422);
    await expect(page.locator("form.user-update-form")).toHaveCount(0);
  }
});


test("test_user_update_changes_only_the_selected_row", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const created = await page.request.post("/users/create", {
    data: [
      { display_name: "Selected user", login: "user_update_selected", password: "Fixture-password7!" },
      { display_name: "Other user", login: "user_update_other", password: "Fixture-password8!" }
    ],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  const [selectedId, otherId] = await created.json();
  const row = id => page.locator("tbody tr").filter({ has: page.locator('td[data-label="id"]').filter({ hasText: new RegExp(`^${id}$`) }) });
  await page.goto("/admin/users");
  await expect(row(selectedId).getByRole("link", { name: "update", exact: true })).toHaveAttribute("href", `/admin/users/${selectedId}/update`);
  await expect(row(otherId).getByRole("link", { name: "update", exact: true })).toHaveAttribute("href", `/admin/users/${otherId}/update`);
  await row(selectedId).getByRole("link", { name: "update", exact: true }).click();
  await page.reload();
  const form = page.locator("form.user-update-form");
  await expect(form.locator('input[name="user_id"]')).toHaveValue(String(selectedId));
  await expect(form.locator('input[name="user_id"]')).toBeHidden();
  await expect(form.getByText("user_id", { exact: true })).toHaveCount(0);
  await form.locator('input[name="display_name"]').fill("Selected user updated");
  await form.locator('input[name="login"]').fill("user_update_selected_changed");
  await form.getByRole("button", { name: "update", exact: true }).click();
  await expect(page).toHaveURL(/\/admin\/users#saved$/);
  await page.goto("/admin/users");
  await expect(row(selectedId).locator('td[data-label="display_name"]')).toHaveText("Selected user updated");
  await expect(row(selectedId).locator('td[data-label="login"]')).toHaveText("user_update_selected_changed");
  await expect(row(otherId).locator('td[data-label="display_name"]')).toHaveText("Other user");
  await expect(row(otherId).locator('td[data-label="login"]')).toHaveText("user_update_other");
});
