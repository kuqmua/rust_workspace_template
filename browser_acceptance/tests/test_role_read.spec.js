import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_role_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const appearance = async (link) => link.evaluate((element) => {
    const button = getComputedStyle(element);
    const icon = getComputedStyle(element.querySelector("svg"));
    return {
      border: button.border,
      padding: button.padding,
      color: button.color,
      background: button.backgroundColor,
      width: icon.width,
      height: icon.height
    };
  });
  await page.goto("/admin/users");
  const userLink = page.locator("tbody tr").first().getByRole("button", { name: "read", exact: true });
  await expect(userLink).toBeVisible();
  const userAppearance = await appearance(userLink);
  await page.goto("/admin/roles");
  const row = page.locator("tbody tr").first();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(6);
  const values = await cells.allTextContents();
  const path = `/admin/roles/${values[0].trim()}`;
  const link = row.getByRole("button", { name: "read", exact: true });
  expect(await appearance(link)).toEqual(userAppearance);
  await page.goto(path);
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="role-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  await page.goto(`${path}?search=missing&offset=999&filter_field=id&filter_operation=eq&filter_value=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 5));
  const created = await page.request.post("/roles/create", {
    data: [{ name: "role_detail_fixture" }],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  const [id] = await created.json();
  await page.goto(`/admin/roles/${id}`);
  const createdRoleValues = detail.locator(".health-result");
  await expect(createdRoleValues).toHaveCount(5);
  await expect(createdRoleValues.nth(0)).toHaveText(String(id));
  await expect(createdRoleValues.nth(1)).toHaveText("role_detail_fixture");
  await expect(createdRoleValues.nth(2)).toHaveText("false");
  await expect(createdRoleValues.nth(3)).not.toBeEmpty();
  await expect(createdRoleValues.nth(4)).not.toBeEmpty();
  await page.goto("/admin/roles/9223372036854775807");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
  await page.goto("/admin/roles/manage");
  const createdRole = page.locator("article.crud-record").filter({
    has: page.locator('input[name="name"][value="role_detail_fixture"]')
  });
  await createdRole.getByLabel("i_understand_this_cannot_be_undone").check();
  await createdRole.getByRole("button", { name: "delete_role" }).click();
  await expect(page).toHaveURL(/\/admin\/roles#saved$/);
});

test("test_role_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/roles/1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
