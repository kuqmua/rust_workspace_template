import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";
import { navigationAdminPaths } from "./support/pages.js";

async function expectSnakeCaseButtonLabels(page) {
  const labels = await page
    .locator("button:not(.table-cell-preview), a.ui-button, header nav a, .nav-menu-toggle > span")
    .evaluateAll(elements => elements.map(element => ({
      text: element.textContent.trim(),
      transform: getComputedStyle(element).textTransform
    })).filter(label => label.text.length > 0));
  expect(labels.length).toBeGreaterThan(0);
  labels.forEach(label => {
    expect(label.text, `Button label at ${page.url()}`).toMatch(/^[a-z][a-z0-9]*(?:_[a-z0-9]+)*$/);
    expect(label.transform, `Text transformation for ${label.text}`).toBe("none");
  });
}

test("test_sign_in_button_uses_snake_case", async ({ page }) => {
  await page.goto("/admin/sign_in");
  await expect(page.getByRole("button", { name: "sign_in", exact: true })).toBeVisible();
  await expectSnakeCaseButtonLabels(page);
});

test.describe("authenticated button labels", () => {
  test.beforeEach(async ({ page }) => {
    await signInInitialAdministrator(page);
  });

  test.afterEach(async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 720 });
    await signOutIfAuthenticated(page);
  });

  const paths = [
    ...navigationAdminPaths.filter(path => path !== "/admin/swagger_ui"),
    "/admin/users/create",
    "/admin/users/manage",
    "/admin/roles/create",
    "/admin/roles/manage"
  ];

  paths.forEach(path => {
    test(`test_button_labels_use_snake_case_on_${path.replaceAll("/", "_")}`, async ({ page }) => {
      await page.goto(path);
      await expect(page.locator("main")).toBeVisible();
      await expect(page.getByText("loading", { exact: true })).toHaveCount(0);
      await expect(page.locator("main")).not.toBeEmpty();
      await expect(page.getByRole("alert")).toHaveCount(0);
      await expectSnakeCaseButtonLabels(page);
      await page.setViewportSize({ width: 390, height: 844 });
      await page.getByText("navigation", { exact: true }).click();
      await expect(page.locator("header nav")).toBeVisible();
      await expectSnakeCaseButtonLabels(page);
    });
  });

  [
    { path: "/admin/users", labels: ["create_user", "manage_users"] },
    { path: "/admin/roles", labels: ["create_role", "manage_roles"] }
  ].forEach(({ path, labels }) => {
    test(`test_resource_and_pagination_labels_on_${path.replaceAll("/", "_")}`, async ({ page }) => {
      await page.goto(path);
      await expect(page.locator(".resource-actions a")).toHaveText(labels);
      await expect(page.locator("nav.table-pagination button")).toHaveText([
        "apply", "previous", "next"
      ]);
    });
  });

  test("test_password_visibility_button_labels_use_snake_case", async ({ page }) => {
    await page.goto("/admin/profile");
    await expect(page.getByRole("button", { name: "generate_password", exact: true })).toBeVisible();
    await expect(page.getByRole("button", { name: "change_password", exact: true })).toBeVisible();
    await page.getByRole("button", { name: "show_password", exact: true }).click();
    await expect(page.getByRole("button", { name: "hide_password", exact: true })).toBeVisible();
    await expectSnakeCaseButtonLabels(page);
    await page.getByRole("button", { name: "hide_password", exact: true }).click();
    await expect(page.getByRole("button", { name: "show_password", exact: true })).toBeVisible();
  });

  test("test_confirmation_button_labels_use_snake_case", async ({ page }) => {
    await page.goto("/admin/settings");
    await page.getByRole("button", { name: "reset_to_template_defaults", exact: true }).click();
    const dialog = page.getByRole("dialog", { name: "reset_settings" });
    await expect(dialog).toBeVisible();
    await expect(dialog.getByRole("button")).toHaveText(["cancel", "reset_settings"]);
    await expectSnakeCaseButtonLabels(page);
    await dialog.getByRole("button", { name: "cancel", exact: true }).click();
    await expect(dialog).not.toBeVisible();
  });

  test("test_audit_download_button_labels_use_snake_case", async ({ page }) => {
    await page.goto("/admin/audit_log");
    await page.getByRole("button", { name: "prepare_page_csv", exact: true }).click();
    await expect(page.getByRole("link", { name: "download_page_csv", exact: true })).toBeVisible();
    await expectSnakeCaseButtonLabels(page);
  });

  test("test_database_display_name_preserves_case_and_spaces", async ({ page }) => {
    const usersResponse = page.waitForResponse(response =>
      new URL(response.url()).pathname === "/v1/admin/users" && response.request().method() === "GET"
    );
    await page.goto("/admin/users");
    const response = await usersResponse;
    expect(response.ok()).toBe(true);
    const users = await response.json();
    const administrator = users.items.find(user => user.login === "administrator");
    expect(administrator).toBeDefined();
    expect(administrator.display_name).toBe("Initial Administrator");
    await expect(page.getByRole("cell", { name: administrator.display_name, exact: true })).toBeVisible();
    await expect(page.getByRole("cell", { name: "initial_administrator", exact: true })).toHaveCount(0);
    await expectSnakeCaseButtonLabels(page);
  });
});
