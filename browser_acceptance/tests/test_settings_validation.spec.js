import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset, signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

test("test_invalid_settings_value_reports_validation_failure", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/settings");
  await page.getByLabel("default_route").fill("/admin/not_a_page");
  let updateRequests = 0;
  page.on("request", request => {
    if (request.method() === "PATCH" && new URL(request.url()).pathname === "/system_settings/update") {
      updateRequests += 1;
    }
  });
  await page.getByRole("button", { name: "save_settings" }).click();
  await expect(page.getByText("check_settings_values", { exact: true })).toBeVisible();
  expect(updateRequests).toBe(0);
  await expect(page.getByLabel("default_route")).toHaveValue("/admin/not_a_page");
});

[
  ["filled", []],
  ["mixed", ["tab_title", "organization_contacts", "primary_color"]],
  ["empty", ["tab_title", "organization_name", "organization_contacts", "support_url", "primary_color", "main_logo"]],
].forEach(([name, clear]) => {
  test(`test_settings_${name}_optional_fields_produce_exact_update_body`, async ({ page }) => {
    await signInInitialAdministrator(page);
    await page.goto("/admin/settings");
    const values = {
      default_admin_route: "/admin/roles",
      site_name: "Fixture site",
      tab_title: "Fixture title",
      organization_name: "Fixture organization",
      organization_contacts: "Fixture contacts",
      support_url: "https://example.com/support",
      primary_color: "#123abc",
      main_logo: "https://example.com/logo.svg",
    };
    const bodies = [];
    const intercept = async route => {
      bodies.push({ method: route.request().method(), contentType: route.request().headers()["content-type"], body: route.request().postDataJSON() });
      await route.fulfill({ status: 503, body: "x" });
    };
    await page.route("**/system_settings/update", intercept);
    try {
      await Object.entries(values).reduce(async (previous, [field, value]) => {
        await previous;
        await page.locator(`[name="${field}"]`).fill(clear.includes(field) ? "" : value);
      }, Promise.resolve());
      await page.getByRole("button", { name: "save_settings", exact: true }).click();
      await expect(page.getByRole("alert")).toContainText("the_server_returned_status_503_for_/system_settings/update");
      expect(bodies).toEqual([{
        method: "PATCH",
        contentType: "application/json",
        body: { ...Object.fromEntries(Object.entries(values).map(([field, value]) => [field, clear.includes(field) ? null : value])), clear },
      }]);
      await expect(page.getByText("check_settings_values", { exact: true })).toHaveCount(0);
      await Promise.all(Object.entries(values).map(([field, value]) => expect(page.locator(`[name="${field}"]`)).toHaveValue(clear.includes(field) ? "" : value)));
    } finally {
      await page.unroute("**/system_settings/update", intercept);
      await signOutIfAuthenticated(page);
    }
  });
});

test("test_settings_reset_requires_confirmation_and_sends_exact_template_body", async ({ page }) => {
  await signInInitialAdministrator(page);
  await page.goto("/admin/settings");
  const bodies = [];
  const intercept = async route => {
    bodies.push({ method: route.request().method(), body: route.request().postDataJSON() });
    await route.fulfill({ status: 503, body: "x" });
  };
  await page.route("**/system_settings/update", intercept);
  try {
    await page.getByLabel("default_route").fill("/admin/not_a_page");
    await page.getByRole("button", { name: "save_settings", exact: true }).click();
    await expect(page.getByText("check_settings_values", { exact: true })).toBeVisible();
    const trigger = page.getByRole("button", { name: "reset_to_template_defaults", exact: true });
    const dialog = page.getByRole("dialog", { name: "reset_settings", exact: true });
    await trigger.click();
    await expect(dialog).toBeVisible();
    await dialog.getByRole("button", { name: "cancel", exact: true }).click();
    await expect(dialog).not.toBeVisible();
    expect(bodies).toEqual([]);
    await expect(page.getByLabel("default_route")).toHaveValue("/admin/not_a_page");
    await trigger.click();
    await dialog.getByRole("button", { name: "reset_settings", exact: true }).click();
    await expect(page.getByRole("alert")).toContainText("the_server_returned_status_503_for_/system_settings/update");
    expect(bodies).toEqual([{
      method: "PATCH",
      body: {
        default_admin_route: "/admin/users", site_name: "Admin",
        tab_title: null, organization_name: null, organization_contacts: null,
        support_url: null, primary_color: null, main_logo: null,
        clear: ["tab_title", "organization_name", "organization_contacts", "support_url", "primary_color", "main_logo"],
      },
    }]);
    await expect(page.getByText("check_settings_values", { exact: true })).toHaveCount(0);
    await expect(page.getByLabel("default_route")).toHaveValue("/admin/not_a_page");
  } finally {
    await page.unroute("**/system_settings/update", intercept);
    await signOutIfAuthenticated(page);
  }
});
