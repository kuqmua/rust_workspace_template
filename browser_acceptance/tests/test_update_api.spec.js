import { expect, test } from "@playwright/test";
import { readUsers, usersReadPage } from "./support/users.js";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_update_users_api_updates_a_batch_and_rolls_back_a_conflict", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const headers = await adminHeaders(page.context());
  const created = await page.request.post("/users/create", {
    data: [
      { display_name: "API Alpha User", login: "api_update_alpha", password: "Api-update-password1!" },
      { display_name: "API Beta User", login: "api_update_beta", password: "Api-update-password2!" }
    ],
    headers
  });
  expect(created.status()).toBe(201);
  const [alphaId, betaId] = await created.json();

  const updated = await page.request.patch("/users/update", {
    data: { updates: [
      { filter: { user_id: alphaId }, changes: { display_name: "API Alpha Updated" } },
      { filter: { user_id: betaId }, changes: { display_name: "API Beta Updated" } }
    ] },
    headers
  });
  expect(updated.status()).toBe(204);
  const afterUpdate = (await usersReadPage(await readUsers(page.request, "limit=100"))).items;
  expect(afterUpdate.find(user => user.id === alphaId)?.display_name).toBe("API Alpha Updated");
  expect(afterUpdate.find(user => user.id === betaId)?.display_name).toBe("API Beta Updated");

  const conflicted = await page.request.patch("/users/update", {
    data: { updates: [
      { filter: { user_id: alphaId }, changes: { display_name: "Rolled Back Alpha" } },
      { filter: { user_id: betaId }, changes: { login: "administrator" } }
    ] },
    headers
  });
  expect(conflicted.status()).toBe(409);
  const afterConflict = (await usersReadPage(await readUsers(page.request, "limit=100"))).items;
  expect(afterConflict.find(user => user.id === alphaId)?.display_name).toBe("API Alpha Updated");
  expect(afterConflict.find(user => user.id === betaId)?.login).toBe("api_update_beta");

  const invalid = await page.request.patch("/users/update", {
    data: { updates: [{ filter: { user_id: alphaId }, changes: {} }] },
    headers
  });
  expect(invalid.status()).toBe(422);
  for (const userId of [alphaId, betaId]) {
    const deleted = await page.request.delete("/users/delete", {
      data: { filter: { user_id: userId } },
      headers
    });
    expect(deleted.status()).toBe(204);
  }
});

test("test_update_roles_api_updates_a_batch_and_rolls_back_a_conflict", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const headers = await adminHeaders(page.context());
  const created = await page.request.post("/roles/create", {
    data: [{ name: "api_update_role_alpha" }, { name: "api_update_role_beta" }],
    headers
  });
  expect(created.status()).toBe(201);
  const [alphaId, betaId] = await created.json();

  const updated = await page.request.patch("/roles/update", {
    data: { updates: [
      { filter: { role_id: alphaId }, changes: { name: "api_updated_role_alpha" } },
      { filter: { role_id: betaId }, changes: { name: "api_updated_role_beta" } }
    ] },
    headers
  });
  expect(updated.status()).toBe(204);
  await page.goto("/admin/roles?limit=100");
  const roleRow = roleId => page.locator("tbody tr").filter({
    has: page.locator('td[data-label="id"]').filter({ hasText: new RegExp(`^${roleId}$`) })
  });
  await expect(roleRow(alphaId)).toContainText("api_updated_role_alpha");
  await expect(roleRow(betaId)).toContainText("api_updated_role_beta");

  const conflicted = await page.request.patch("/roles/update", {
    data: { updates: [
      { filter: { role_id: alphaId }, changes: { name: "api_rolled_back_role_alpha" } },
      { filter: { role_id: betaId }, changes: { rules: { expected_rule_ids: [999999], rule_ids: [] } } }
    ] },
    headers
  });
  expect(conflicted.status()).toBe(409);
  await page.reload();
  await expect(roleRow(alphaId)).toContainText("api_updated_role_alpha");
  await expect(roleRow(betaId)).toContainText("api_updated_role_beta");

  const invalid = await page.request.patch("/roles/update", {
    data: { updates: [] },
    headers
  });
  expect(invalid.status()).toBe(422);
  for (const roleId of [alphaId, betaId]) {
    const deleted = await page.request.delete("/roles/delete", {
      data: { filter: { role_id: roleId } },
      headers
    });
    expect(deleted.status()).toBe(204);
  }
});

test("test_update_api_requires_authentication", async ({ page }) => {
  for (const [path, update] of [
    ["/users/update", { filter: { user_id: 1 }, changes: { display_name: "Unauthorized User" } }],
    ["/roles/update", { filter: { role_id: 1 }, changes: { name: "unauthorized_role" } }]
  ]) {
    const response = await page.request.patch(path, {
      data: { updates: [update] }
    });
    expect([401, 403]).toContain(response.status());
  }
});
