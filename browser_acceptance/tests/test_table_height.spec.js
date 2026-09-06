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
  test(`test_${name}_table_matches_navigation_row_height`, async ({ page }) => {
    await page.goto(path);
    await expect(page.locator(".table-scroll table")).toBeVisible();
    await [1920, 1280, 768, 390, 320].reduce(async (previous, width) => {
      await previous;
      await page.setViewportSize({ width, height: 1080 });
      await expect.poll(async () => page.evaluate(() => {
        const header = document.querySelector(".topbar");
        const mobile = getComputedStyle(document.querySelector(".nav-menu-toggle")).display !== "none";
        const height = mobile ? header.getBoundingClientRect().height :
          Math.max(...Array.from(header.querySelectorAll(".nav-menu-list > li"),
            element => element.getBoundingClientRect().height)) +
          parseFloat(getComputedStyle(header).borderBottomWidth);
        const selectors = mobile ? ".table-scroll tbody td, .table-pagination" :
          ".table-scroll thead tr, .table-scroll tbody tr, .table-pagination";
        return Array.from(document.querySelectorAll(selectors)).flatMap(element => {
          const actual = element.getBoundingClientRect().height;
          return Math.abs(actual - height) > 1 ? [{ element: element.tagName, actual, expected: height }] : [];
        });
      }), { message: `${path} should use the navigation row height at ${width}px` }).toEqual([]);
      await expect.poll(async () => page.evaluate(() =>
        document.documentElement.scrollWidth - document.documentElement.clientWidth
      )).toBeLessThanOrEqual(1);
    }, Promise.resolve());
  });
});

test("test_compact_session_dialog_keeps_text_inside_its_mobile_bounds", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 844 });
  await page.goto("/admin/sessions");
  await page.getByRole("button", { name: "revoke_session", exact: true }).first().click();
  const dialog = page.locator("dialog:visible");
  await expect(dialog).toBeVisible();
  expect(await dialog.evaluate(element => element.scrollWidth - element.clientWidth)).toBeLessThanOrEqual(1);
  await dialog.getByRole("button", { name: "cancel", exact: true }).click();
  await expect(dialog).toHaveCount(0);
});

test("test_compact_mobile_pagination_keeps_controls_reachable", async ({ page }) => {
  await page.setViewportSize({ width: 320, height: 844 });
  await page.goto("/admin/permissions?limit=1&offset=1");
  const pagination = page.locator(".table-pagination");
  await expect(pagination.getByRole("spinbutton")).toBeVisible();
  await ["apply", "next", "previous"].reduce(async (previous, name) => {
    await previous;
    await pagination.getByRole("button", { name, exact: true }).click({ trial: true });
  }, Promise.resolve());
});
