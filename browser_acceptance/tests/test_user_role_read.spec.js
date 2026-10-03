import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_user_role_details_follow_table_link_and_ignore_list_filters", async ({ page }) => {
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
  const userLink = page.locator("tbody tr").first().getByRole("link", { name: "read", exact: true });
  await expect(userLink).toBeVisible();
  const userAppearance = await appearance(userLink);
  await userLink.click();
  await expect(page.locator('section[data-page="user-read"] .health-result')).toHaveCount(7);
  await page.goto("/admin/user_roles");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(5);
  const values = await cells.allTextContents();
  const path = `/admin/user_roles/${values[0].trim()}/read`;
  const link = row.getByRole("link", { name: "read", exact: true });
  expect(await appearance(link)).toEqual(userAppearance);
  await expect(link).toHaveAttribute("href", path);
  await expect(row.getByRole("button", { name: "read", exact: true })).toHaveCount(0);
  await expect(page.locator('td[data-label="actions"] dialog[aria-label="read"]')).toHaveCount(0);
  expect(await link.getAttribute("commandfor")).toBeNull();
  await link.click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="user-role-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.goto(`${path}?search=missing&offset=999&filter_field=id&filter_operation=eq&filter_value=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await expect(detail.locator("input, select, textarea, form")).toHaveCount(0);
  await page.goto(`/admin/user_roles/${values[0].trim()}`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.goto("/admin/user_roles/9223372036854775807/read");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_user_role_read_links_follow_each_row_and_support_keyboard_and_back", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const roles = await page.request.post("/roles/create", {
    data: [{ name: "read_link_role_first" }, { name: "read_link_role_second" }],
    headers: await adminHeaders(page.context())
  });
  expect(roles.status()).toBe(201);
  const user = await page.request.post("/users/create", {
    data: [{ display_name: "Read role links", login: "read_role_links_user", password: "Fixture-password7!", role_ids: await roles.json() }],
    headers: await adminHeaders(page.context())
  });
  expect(user.status()).toBe(201);
  await page.goto("/admin/user_roles");
  const rows = page.locator("tbody tr");
  await expect(rows).toHaveCount(3);
  for (const row of await rows.all()) {
    const id = (await row.locator('td[data-label="id"]').textContent()).trim();
    await expect(row.getByRole("link", { name: "read", exact: true })).toHaveAttribute("href", `/admin/user_roles/${id}/read`);
    await expect(row.locator('dialog[aria-label="read"]')).toHaveCount(0);
  }
  const row = rows.first();
  const values = (await row.locator("td").allTextContents()).slice(0, 4);
  const read = row.getByRole("link", { name: "read", exact: true });
  await read.focus();
  await page.keyboard.press("Enter");
  await expect(page).toHaveURL(`/admin/user_roles/${values[0].trim()}/read`);
  await expect(page.locator('[data-page="user-role-read"] .health-result')).toHaveText(values);
  await page.goBack();
  await expect(page).toHaveURL("/admin/user_roles");
  await expect(read).toBeVisible();
});
