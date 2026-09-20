import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

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
  await expect(page.locator('[data-page="user-read"] .health-result')).toHaveCount(5);
  await page.goto("/admin/user_roles");
  const row = page.locator("tbody tr").last();
  const cells = row.locator("td");
  await expect(cells).toHaveCount(5);
  const values = await cells.allTextContents();
  const path = `/admin/user_roles/${values[0].trim()}`;
  const link = row.getByRole("link", { name: "read", exact: true });
  await expect(link).toHaveAttribute("href", path);
  expect(await appearance(link)).toEqual(userAppearance);
  await link.click();
  await expect(page).toHaveURL(new RegExp(`${path}$`));
  const detail = page.locator('[data-page="user-role-read"]');
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.reload();
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.goto(`${path}?search=missing&offset=999&filter_field=id&filter_operation=eq&filter_value=999`);
  await expect(detail.locator(".health-result")).toHaveText(values.slice(0, 4));
  await page.goto("/admin/user_roles/9223372036854775807");
  await expect(detail).toContainText("resource not found");
  await expect(detail.locator(".health-result")).toHaveCount(0);
});

test("test_user_role_details_require_authentication", async ({ page }) => {
  await page.goto("/admin/user_roles/1");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});
