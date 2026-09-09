import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

test("test_table_views_use_unprefixed_api_paths", async ({ page }) => {
  await signInInitialAdministrator(page);
  try {
    const document = await (await page.request.get("/openapi.json")).json();
    const resources = ["user_roles", "role_permissions", "refresh_tokens", "access_sessions", "login_attempts", "rate_limits", "cleanup_status"];
    await resources.reduce(async (previous, resource) => {
      await previous;
      const response = await page.request.get(`/${resource}?limit=1&offset=0`);
      expect(response.status(), resource).toBe(200);
      expect((await response.json()).table).toBe(resource);
      expect(document.paths[`/${resource}`].get).toBeDefined();
      const legacy = await page.request.get(`/tables/${resource}`, { maxRedirects: 0 });
      expect(legacy.status(), resource).toBe(422);
      const loaded = page.waitForResponse(value => new URL(value.url()).pathname === `/${resource}` && value.request().method() === "GET");
      await page.goto(`/admin/${resource}`);
      expect((await loaded).status(), resource).toBe(200);
      await expect(page.locator('section[data-renderer="csr"] table')).toBeVisible();
    }, Promise.resolve());
    await ["users", "roles", "permissions", "audit_log", "system_settings"].reduce(async (previous, resource) => {
      await previous;
      expect((await page.request.get(`/tables/${resource}`)).status(), resource).toBe(200);
    }, Promise.resolve());
    expect((await page.request.get("/tables")).status()).toBe(200);
  } finally {
    await signOutIfAuthenticated(page);
  }
});
