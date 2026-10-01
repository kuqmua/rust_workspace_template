import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

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

  for (const roleId of [alphaId, betaId]) {
    const deleted = await page.request.delete("/roles/delete", {
      data: { filter: { role_id: roleId } },
      headers
    });
    expect(deleted.status()).toBe(204);
  }
});
