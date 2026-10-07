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

test("test_password_generation_failure_preserves_input_and_recovery_resets_errors", async ({ page }) => {
  await signInInitialAdministrator(page);
  await page.goto("/admin/profile");
  const newPassword = page.getByLabel("new_password");
  const validation = page.getByText("check_both_passwords_and_ensure_the_new_password_satisfies_the_policy", { exact: true });
  const generation = page.getByText("password_generation_failed", { exact: true });
  await newPassword.fill("admin");
  await page.getByRole("button", { name: "change_password" }).click();
  await expect(validation).toBeVisible();
  await page.getByRole("button", { name: "show_password" }).click();
  await expect(newPassword).toHaveAttribute("type", "text");
  await page.evaluate(() => {
    Object.defineProperty(window.crypto, "getRandomValues", {
      configurable: true,
      value: () => { throw new DOMException("fixture failure", "OperationError"); }
    });
  });
  try {
    await page.getByRole("button", { name: "generate_password" }).click();
    await expect(generation).toBeVisible();
    await expect(validation).toBeVisible();
    await expect(newPassword).toHaveValue("admin");
    await expect(newPassword).toHaveAttribute("type", "text");
    await page.evaluate(() => {
      Object.defineProperty(window.crypto, "getRandomValues", {
        configurable: true,
        value: array => { array.fill(0); return array; }
      });
    });
    await page.getByRole("button", { name: "generate_password" }).click();
    await expect(newPassword).toHaveValue(/^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[^A-Za-z0-9\s])\S{12,1024}$/);
    await expect(newPassword).toHaveAttribute("type", "password");
    await expect(page.getByRole("button", { name: "show_password" })).toBeVisible();
    await expect(generation).toHaveCount(0);
    await expect(validation).toHaveCount(0);
  } finally {
    await page.evaluate(() => { delete window.crypto.getRandomValues; });
  }
});
