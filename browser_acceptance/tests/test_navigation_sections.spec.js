import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

["users", "roles", "user_roles"].forEach(section => {
  test(`test_${section}_navigation_stays_active_on_read_pages`, async ({ page }) => {
    await signInAdministratorWithPasswordReset(page);
    await page.goto(`/admin/${section}`);
    const navigation = page.locator(`header a[href="/admin/${section}"]`);
    await expect(navigation).toHaveAttribute("aria-current", "page");
    await expect(navigation).toHaveClass(/\bactive\b/);
    const read = page.locator("tbody tr").first().getByRole("link", { name: "read", exact: true });
    await expect(read).toBeVisible();
    const path = await read.getAttribute("href");
    await read.click();
    await expect(page).toHaveURL(path);
    await expect(navigation).toHaveAttribute("aria-current", "page");
    await expect(navigation).toHaveClass(/\bactive\b/);
    await expect(page.locator('header a[aria-current="page"]')).toHaveCount(1);
    await page.reload();
    await expect(navigation).toHaveAttribute("aria-current", "page");
    await expect(navigation).toHaveClass(/\bactive\b/);
    await page.goBack();
    await expect(navigation).toHaveAttribute("aria-current", "page");
  });
});
