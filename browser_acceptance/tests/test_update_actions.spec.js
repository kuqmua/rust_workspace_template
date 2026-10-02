import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_user_row_update_actions_navigate_to_update_page", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/users");

  const rows = page.locator('section[data-renderer="csr"] tbody tr');
  await expect(rows.first()).toBeVisible();
  await expect(page.locator('section[data-renderer="csr"] dialog[aria-label="update"]')).toHaveCount(0);
  for (const row of await rows.all()) {
    const id = (await row.locator('td[data-label="id"]').textContent()).trim();
    await expect(row.locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }))
      .toHaveAttribute("href", `/admin/users/${id}/update`);
  }

  const update = rows.first().locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true });
  await expect(update).toHaveAttribute("title", "update");
  await expect(update.locator('svg[aria-hidden="true"]')).toHaveCount(1);
  const read = rows.first().locator('td[data-label="actions"]').getByRole("link", { name: "read", exact: true });
  const actionStyles = element => {
    const style = getComputedStyle(element);
    return [style.backgroundColor, style.color, style.height, style.borderRadius];
  };
  expect(await update.evaluate(actionStyles)).toEqual(await read.evaluate(actionStyles));
  expect(await update.locator("svg path").getAttribute("d")).not.toBe(await read.locator("svg path").getAttribute("d"));
  const id = (await rows.first().locator('td[data-label="id"]').textContent()).trim();
  await update.click();
  await expect(page).toHaveURL(`http://127.0.0.1:18080/admin/users/${id}/update`);
  await expect(page.locator("form.user-update-form")).toBeVisible();
});

test("test_role_row_update_actions_navigate_to_update_page", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const created = await page.request.post("/roles/create", {
    data: [{ name: "row_update_action_fixture" }],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  const [roleId] = await created.json();
  await page.goto("/admin/roles");

  const rows = page.locator('section[data-renderer="csr"] tbody tr').filter({ has: page.locator('td[data-label="id"]').filter({ hasText: new RegExp(`^${roleId}$`) }) });
  await expect(rows.first()).toBeVisible();
  await expect(page.locator('section[data-renderer="csr"] dialog[aria-label="update"]')).toHaveCount(0);
  for (const row of await rows.all()) {
    await expect(row.locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }))
      .toHaveAttribute("href", `/admin/roles/${roleId}/update`);
  }

  await rows.first().locator('td[data-label="actions"]').getByRole("link", { name: "update", exact: true }).click();
  await expect(page).toHaveURL(`http://127.0.0.1:18080/admin/roles/${roleId}/update`);
  await expect(page.locator("form.role-update-form")).toBeVisible();
});
