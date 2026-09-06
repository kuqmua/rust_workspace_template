import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";
import { dataTablePages, tablePages } from "./support/pages.js";

test.beforeEach(async ({ page }) => {
  await signInInitialAdministrator(page);
});

test.afterEach(async ({ page }) => {
  await page.setViewportSize({ width: 1280, height: 720 });
  await signOutIfAuthenticated(page);
});

[...tablePages, ...dataTablePages].forEach(({ name, path }) => {
  test(`test_${name}_table_fills_available_width`, async ({ page }) => {
    await page.goto(path);
    const table = page.locator(".table-scroll table");
    await expect(table).toBeVisible();
    await [390, 1280, 1920, 2560].reduce(async (previous, width) => {
      await previous;
      await page.setViewportSize({ width, height: 1080 });
      await expect.poll(async () => table.evaluate(element => {
        const container = element.closest(".table-scroll");
        return element.getBoundingClientRect().width - container.clientWidth;
      }), { message: `${path} should fill its scroll container at ${width}px` }).toBeGreaterThanOrEqual(-1);
      await expect.poll(async () => page.evaluate(() =>
        document.documentElement.scrollWidth - document.documentElement.clientWidth
      ), { message: `${path} should keep horizontal overflow inside the table at ${width}px` }).toBeLessThanOrEqual(1);
    }, Promise.resolve());
  });
});
