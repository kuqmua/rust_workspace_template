import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

const resources = [
  "refresh_tokens", "login_attempts", "rate_limits", "cleanup_status",
  "access_sessions", "role_rules", "user_roles", "users", "roles", "rules",
  "audit_log", "system_settings",
];

test("test_table_views_use_unprefixed_api_paths", async ({ page }) => {
  test.setTimeout(120_000);
  await signInInitialAdministrator(page);
  try {
    await resources.reduce(async (previous, resource) => {
      await previous;
      const path = `/${resource}/read`;
      const loaded = page.waitForResponse(value => new URL(value.url()).pathname === path);
      await page.goto(`/admin/${resource}`);
      const response = await loaded;
      expect(response.status(), resource).toBe(200);
      expect(await response.json(), resource).toBeTruthy();
      await expect(page.locator('section[data-renderer="csr"] table')).toBeVisible();
      const legacy = await page.request.get(`/tables/${resource}/read`, { maxRedirects: 0 });
      expect(legacy.status(), resource).not.toBe(200);
    }, Promise.resolve());
  } finally {
    await signOutIfAuthenticated(page);
  }
});
