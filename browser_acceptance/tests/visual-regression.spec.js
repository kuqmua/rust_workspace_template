import { expect, test } from "@playwright/test";
import {
  signInAdministrator,
  signInBootstrappedAdministrator,
  signOutIfAuthenticated
} from "./support/admin.js";
import {
  adminPages,
  dataTablePages,
  tablePages
} from "./support/pages.js";

test.describe.configure({ mode: "serial" });
test.skip(
  ({ browserName }) => browserName !== "chromium",
  "visual baselines are intentionally generated on Chromium/Linux"
);

const viewports = [
  { height: 900, name: "desktop", width: 1440 },
  { height: 844, name: "mobile", width: 390 }
];

const tableMask = Object.freeze([
  ".table-scroll",
  ".table-pagination",
  ".table-page > p"
]);
const authenticatedPages = [
  ...tablePages.map(({ name, path }) => ({ mask: tableMask, name, path })),
  { mask: [".profile-card dd"], ...adminPages.profile },
  { mask: [], ...adminPages.settings },
  { mask: ["main pre"], ...adminPages.metrics },
  { mask: ["main pre"], ...adminPages.version },
  ...dataTablePages.map(({ path, snapshotName }) => ({
    mask: tableMask,
    name: snapshotName,
    path
  }))
];

async function stabilize(page) {
  await page.evaluate(() => document.fonts.ready);
}

async function expectPixelPerfect(page, name, mask) {
  await stabilize(page);
  await expect(page).toHaveScreenshot(`${name}.png`, {
    animations: "disabled",
    caret: "hide",
    fullPage: true,
    mask: mask.map(selector => page.locator(selector)),
    maxDiffPixels: 10,
    scale: "css",
    threshold: 0.2
  });
}

async function expectComponentPixelPerfect(page, component, name) {
  await stabilize(page);
  await expect(component).toHaveScreenshot(`${name}.png`, {
    animations: "disabled",
    caret: "hide",
    maxDiffPixels: 0,
    scale: "css",
    threshold: 0.2
  });
}

test.afterEach(async ({ page }) => {
  await signOutIfAuthenticated(page);
});

for (const viewport of viewports) {
  test(`sign-in is pixel-perfect on ${viewport.name}`, async ({ page }) => {
    await page.setViewportSize(viewport);
    await page.goto("/admin/sign_in");
    await expect(page.getByRole("button", { name: "Sign in" })).toBeVisible();
    await expectPixelPerfect(page, `sign-in-${viewport.name}`, []);
  });

  if (viewport.name === "desktop") {
    test("role permissions filter is pixel-perfect on desktop", async ({ page }) => {
      await page.setViewportSize(viewport);
      await signInBootstrappedAdministrator(page);
      await page.goto("/admin/role_permissions");
      await page
        .locator('th[data-field="role_id"] .table-column-filter > button')
        .click();
      const filter = page.getByRole("dialog", { name: "Filter Role Id" });
      await expect(filter).toBeVisible();
      await expectComponentPixelPerfect(
        page,
        filter,
        "role-permissions-filter-desktop"
      );
    });
  }

  for (const pageSpec of authenticatedPages) {
    test(`${pageSpec.name} is pixel-perfect on ${viewport.name}`, async ({ page }) => {
      await page.setViewportSize(viewport);
      await signInAdministrator(page);
      const response = await page.goto(pageSpec.path);
      expect(response).not.toBeNull();
      expect(response.status()).toBe(200);
      await expect(page.locator("main")).toBeVisible();
      await expect(page.getByRole("alert")).toHaveCount(0);
      await expectPixelPerfect(
        page,
        `${pageSpec.name}-${viewport.name}`,
        pageSpec.mask
      );
    });
  }

  test(`unknown table error is pixel-perfect on ${viewport.name}`, async ({ page }) => {
    await page.setViewportSize(viewport);
    await signInAdministrator(page);
    const response = await page.goto("/admin/tables");
    expect(response).not.toBeNull();
    expect(response.status()).toBe(422);
    await expectPixelPerfect(page, `unknown-table-${viewport.name}`, []);
  });

  test(`disabled OpenAPI error is pixel-perfect on ${viewport.name}`, async ({
    page
  }) => {
    await page.setViewportSize(viewport);
    await signInAdministrator(page);
    const response = await page.goto("/admin/swagger_ui");
    expect(response).not.toBeNull();
    expect(response.status()).toBe(422);
    await expectPixelPerfect(page, `openapi-disabled-${viewport.name}`, []);
  });
}
