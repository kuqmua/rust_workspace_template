import { expect, test } from "@playwright/test";

test.skip(
  process.env.BROWSER_ACCEPTANCE_SCREENSHOTS !== "1",
  "documentation screenshots are generated explicitly"
);

test("capture administrator documentation screenshots", async ({ page }) => {
  await page.goto("/admin/sign_in");
  await page.screenshot({
    fullPage: true,
    path: "../docs/images/admin/sign-in.png"
  });
  await page.getByLabel("Login").fill("administrator");
  await page.getByLabel("Password").fill("Initial-password1!");
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page).toHaveURL(/\/admin\/profile$/);
  await page.getByLabel("Current password").fill("Initial-password1!");
  await page.getByLabel("New password").fill("Changed-password2!");
  const passwordChanged = page.waitForResponse(
    response =>
      response.url().endsWith("/api/v1/admin/auth/password") &&
      response.status() === 204
  );
  await page.getByRole("button", { name: "Change password" }).click();
  await passwordChanged;

  for (const [name, path] of [
    ["users", "/admin/users"],
    ["roles", "/admin/roles"],
    ["settings", "/admin/settings"],
    ["sessions", "/admin/sessions"],
    ["data-table", "/admin/audit_log"]
  ]) {
    await page.goto(path);
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    await page.screenshot({
      fullPage: true,
      path: `../docs/images/admin/${name}.png`
    });
  }
});
