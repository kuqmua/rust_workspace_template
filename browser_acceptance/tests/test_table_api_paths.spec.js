import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

test("test_table_views_use_unprefixed_api_paths", async ({ page }) => {
  await signInInitialAdministrator(page);
  try {
    const document = await (await page.request.get("/openapi.json/read")).json();
    const resources = ["refresh_tokens", "login_attempts", "rate_limits", "cleanup_status"];
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
    await ["access_sessions", "role_rules"].reduce(async (previous, resource) => {
      await previous;
      const path = `/${resource}/read`;
      const example = await page.request.get(`${path}_payload_example`);
      expect(example.status(), resource).toBe(200);
      const response = await page.request.post(path, { data: await example.json() });
      expect(response.status(), resource).toBe(200);
      expect((await response.json()).table).toBe(resource);
      expect(document.paths[path].post).toBeDefined();
      expect(document.paths[path].get).toBeUndefined();
      const loaded = page.waitForResponse(value => new URL(value.url()).pathname === path && value.request().method() === "POST");
      await page.goto(`/admin/${resource}`);
      expect((await loaded).status(), resource).toBe(200);
      await expect(page.locator('section[data-renderer="csr"] table')).toBeVisible();
    }, Promise.resolve());
    const userRoles = await page.request.post("/user_roles/read", {
      data: { limit: 1, offset: 0 }
    });
    expect(userRoles.status()).toBe(200);
    expect((await userRoles.json()).table).toBe("user_roles");
    expect(document.paths["/user_roles/read"].post).toBeDefined();
    expect(document.paths["/user_roles/read"].get).toBeUndefined();
    const loadedUserRoles = page.waitForResponse(value => new URL(value.url()).pathname === "/user_roles/read" && value.request().method() === "POST");
    await page.goto("/admin/user_roles");
    expect((await loadedUserRoles).status()).toBe(200);
    await expect(page.locator('section[data-renderer="csr"] table')).toBeVisible();
    await ["users", "roles", "rules", "audit_log"].reduce(async (previous, resource) => {
      await previous;
      expect((await page.request.get(`/tables/${resource}/read`)).status(), resource).toBe(422);
    }, Promise.resolve());
    expect((await page.request.get("/tables/system_settings/read")).status()).toBe(422);
    expect((await page.request.get("/tables/read")).status()).toBe(200);
  } finally {
    await signOutIfAuthenticated(page);
  }
});
