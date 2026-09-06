import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

test.afterEach(async ({ page }) => {
  await signOutIfAuthenticated(page);
});

test("profile generates, reveals, and replaces a password without submitting it", async ({ page }) => {
  await signInInitialAdministrator(page);
  await page.goto("/admin/profile");
  const password = page.getByLabel("New password", { exact: true });
  const generate = page.getByRole("button", { name: "generate_password", exact: true });
  let passwordChanges = 0;
  page.on("request", request => {
    if (request.method() === "POST" && request.url().endsWith("/v1/admin/auth/password")) {
      passwordChanges += 1;
    }
  });

  await generate.click();
  await expect(password).not.toHaveValue("");
  const first = await password.inputValue();
  expect(first.length).toBe(68);
  expect(/[A-Z]/.test(first) && /[a-z]/.test(first) && /[0-9]/.test(first) && /[^A-Za-z0-9\s]/.test(first) && !/\s/.test(first)).toBe(true);
  await expect(password).toHaveAttribute("type", "password");
  await expect(page.getByLabel("Current password", { exact: true })).toHaveValue("");
  await expect(page.locator(".security-card code")).toHaveCount(0);
  await page.getByRole("button", { name: "show_password", exact: true }).click();
  await expect(page.locator(".security-card code")).toHaveText(first);
  await page.getByRole("button", { name: "hide_password", exact: true }).click();
  await expect(page.locator(".security-card code")).toHaveCount(0);
  await generate.click();
  await expect(password).not.toHaveValue(first);
  expect(passwordChanges).toBe(0);

  const second = await password.inputValue();
  await page.evaluate(() => {
    Object.defineProperty(Crypto.prototype, "getRandomValues", {
      configurable: true,
      value() { throw new DOMException("Unavailable", "OperationError"); }
    });
  });
  await generate.click();
  await expect(page.getByText("password generation failed", { exact: true })).toBeVisible();
  await expect(password).toHaveValue(second);
  expect(passwordChanges).toBe(0);
});
