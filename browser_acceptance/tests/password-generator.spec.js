import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

test.afterEach(async ({ page }) => {
  await signOutIfAuthenticated(page);
});

test("test_profile_displays_account_without_password_update_controls", async ({ page }) => {
  await signInInitialAdministrator(page);
  await page.goto("/admin/profile");
  const profile = page.locator(".profile-grid");
  await expect(profile.locator('[data-name="Label"] > span:first-child')).toHaveText(["display_name", "login", "roles", "permissions"]);
  await expect(profile.locator('[data-name="Label"] > span:last-child')).toHaveCount(4);
  await expect(profile.locator("input, button, form, dl, dt, dd")).toHaveCount(0);
});
