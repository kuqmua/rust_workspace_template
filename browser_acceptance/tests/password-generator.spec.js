import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

test.afterEach(async ({ page }) => {
  await signOutIfAuthenticated(page);
});

test("test_profile_displays_account_and_generates_a_new_password", async ({ page }) => {
  await signInInitialAdministrator(page);
  await page.goto("/admin/profile");
  const profile = page.locator(".profile-grid");
  await expect(profile.getByText("display_name", { exact: true })).toBeVisible();
  await expect(profile.getByText("login", { exact: true })).toBeVisible();
  await expect(profile.getByText("roles", { exact: true })).toBeVisible();
  await expect(profile.getByText("rules", { exact: true })).toBeVisible();
  await expect(page.getByLabel("current_password")).toBeVisible();
  const newPassword = page.getByLabel("new_password");
  await page.getByRole("button", { name: "generate_password" }).click();
  await expect(newPassword).toHaveValue(/^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[^A-Za-z0-9\s])\S{12,1024}$/);
  await page.getByRole("button", { name: "show_password" }).click();
  await expect(newPassword).toHaveAttribute("type", "text");
  await page.getByRole("button", { name: "hide_password" }).click();
  await expect(newPassword).toHaveAttribute("type", "password");
});
