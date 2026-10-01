import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

const healthPaths = ["/health/read", "/health_check/read", "/health/live/read", "/health/ready/read"];
const brandingFields = [
  ["site_name", "site_name"], ["tab_title", "tab_title"],
  ["main_logo_url", "main_logo"], ["primary_color", "primary_color"],
  ["support_url", "support_url"], ["default_admin_route", "default_admin_route"]
];

async function expectHealthResponses(page, responses) {
  await expect.poll(() => responses.size).toBe(healthPaths.length);
  await Promise.all(healthPaths.map(async (path, index) => {
    const response = responses.get(path);
    expect(response.status()).toBe(200);
    await expect(page.locator(".health-result").nth(index)).toHaveText(`${response.status()} ${await response.text()}`);
  }));
}

async function expectBrandingFields(page, branding) {
  await expect(page.locator('[data-name="Field"]')).toHaveCount(brandingFields.length);
  await Promise.all(brandingFields.map(([label, key]) =>
    expect(page.locator('[data-name="Label"]').filter({ has: page.getByText(label, { exact: true }) }).locator("span").nth(1)).toHaveText(branding[key] ?? "")
  ));
}

test.beforeEach(async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
});

test("test_health_displays_every_endpoint_response_after_reload", async ({ page }) => {
  const responses = new Map();
  page.on("response", response => {
    const path = new URL(response.url()).pathname;
    if (healthPaths.includes(path)) responses.set(path, response);
  });
  await page.goto("/admin/health");
  await expect(page.locator(".health-label")).toHaveText(healthPaths);
  await expectHealthResponses(page, responses);
  responses.clear();
  await page.reload();
  await expectHealthResponses(page, responses);
});

test("test_health_displays_failed_probe_without_hiding_other_results", async ({ page }) => {
  await page.route("**/health/ready/read", route => route.fulfill({ status: 503, body: "probe_unavailable" }));
  await page.goto("/admin/health");
  await expect(page.locator(".health-label")).toHaveText(healthPaths);
  await expect(page.locator(".health-result").nth(3)).toHaveText("503 probe_unavailable");
  await Promise.all([0, 1, 2].map(index => expect(page.locator(".health-result").nth(index)).toContainText("200 ")));
});

test("test_health_displays_network_error_without_hiding_other_results", async ({ page }) => {
  await page.route("**/health/ready/read", route => route.abort("failed"));
  await page.goto("/admin/health");
  await expect(page.locator(".health-result").nth(3)).toHaveText("request failed");
  await Promise.all([0, 1, 2].map(index => expect(page.locator(".health-result").nth(index)).toContainText("200 ")));
});

test("test_branding_displays_all_public_settings_after_reload", async ({ page }) => {
  const response = await page.request.get("/branding/read");
  expect(response.status()).toBe(200);
  const branding = await response.json();
  await page.goto("/admin/branding");
  await expectBrandingFields(page, branding);
  await page.reload();
  await expectBrandingFields(page, branding);
});

test("test_branding_displays_populated_optional_fields", async ({ page }) => {
  const branding = {
    site_name: "Coverage site", tab_title: "Coverage title",
    main_logo: "https://example.com/logo.svg", primary_color: "#123456",
    support_url: "https://example.com/support", default_admin_route: "/admin/users"
  };
  await page.route("**/branding/read", route => route.fulfill({ json: branding }));
  await page.goto("/admin/branding");
  await expectBrandingFields(page, branding);
  await page.reload();
  await expectBrandingFields(page, branding);
});
