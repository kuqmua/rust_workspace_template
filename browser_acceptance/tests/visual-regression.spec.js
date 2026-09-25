import { devices, expect, test } from "@playwright/test";
import {
  adminOrigin,
  signInInitialAdministrator
} from "./support/admin.js";
import {
  adminPages,
  dataTablePages,
  tablePages
} from "./support/pages.js";

const authenticatedTest = test.extend({
  adminPage: [async ({ browser }, use) => {
    const page = await browser.newPage({
      ...devices["Desktop Chrome"],
      baseURL: adminOrigin
    });
    await signInInitialAdministrator(page);
    await use(page);
    await page.close();
  }, { scope: "worker" }],
  page: async ({ adminPage }, use) => {
    await use(adminPage);
  }
});

authenticatedTest.describe.configure({ mode: "serial" });
authenticatedTest.skip(
  ({ browserName }) => browserName !== "chromium",
  "visual baselines are intentionally generated on Chromium/Linux"
);

const viewports = [
  { height: 900, name: "desktop", width: 1440 },
  { height: 844, name: "mobile", width: 390 }
];
const tableSnapshotStylePath = new URL(
  "./support/table-snapshot.css",
  import.meta.url
).pathname;

const tableMask = Object.freeze([
  ".table-scroll",
  ".table-pagination",
  ".table-page > p"
]);
const authenticatedPages = [
  ...tablePages.map(({ name, path }) => ({ mask: tableMask, name, path })),
  { mask: [".profile-account-value"], ...adminPages.profile },
  { mask: [], ...adminPages.settings },
  { mask: ["main pre"], ...adminPages.metrics },
  { mask: ["main pre"], ...adminPages.version },
  ...dataTablePages.map(({ path, snapshotName }) => ({
    mask: tableMask,
    name: snapshotName,
    path
  }))
];
const crudPages = [
  {
    activePath: "/admin/users",
    name: "user-create",
    path: "/admin/users/create"
  },
  {
    activePath: "/admin/users",
    name: "user-update",
    path: "/admin/users/update"
  },
  {
    activePath: "/admin/users",
    name: "user-manage",
    path: "/admin/users/manage"
  },
  {
    activePath: "/admin/roles",
    name: "role-create",
    path: "/admin/roles/create"
  },
  {
    activePath: "/admin/roles",
    name: "role-update",
    path: "/admin/roles/update"
  },
  {
    activePath: "/admin/roles",
    name: "role-manage",
    path: "/admin/roles/manage"
  }
];

async function stabilize(page) {
  await page.evaluate(async () => {
    await document.fonts.ready;
    await new Promise(resolve =>
      requestAnimationFrame(() => requestAnimationFrame(resolve))
    );
  });
}

async function expectPixelPerfect(page, name, mask) {
  await stabilize(page);
  const options = {
    animations: "disabled",
    caret: "hide",
    fullPage: true,
    mask: mask.map(selector => page.locator(selector)),
    maxDiffPixels: 10,
    scale: "css",
    threshold: 0.2
  };
  if (mask.includes(".table-scroll")) {
    options.stylePath = tableSnapshotStylePath;
  }
  await expect(page).toHaveScreenshot(`${name}.png`, options);
}

async function expectComponentPixelPerfect(page, component, name) {
  await stabilize(page);
  await expect(component).toHaveScreenshot(`${name}.png`, {
    animations: "disabled",
    caret: "hide",
    maxDiffPixels: 100,
    scale: "css",
    threshold: 0.2
  });
}

async function expectInformationCanvasToUseAvailableWidth(page) {
  const geometry = await page.locator(".main-content").evaluate(main => {
    const frame = main.querySelector(".page-frame");
    const content = frame?.querySelector(":scope > :not(.flash-success)");
    if (!(content instanceof HTMLElement)) {
      return null;
    }
    const primary = [...content.querySelectorAll(
      ".table-page, .crud-page, .profile-grid, .settings-grid, .code-card"
    )].find(element => element.getBoundingClientRect().width > 0);
    return {
      contentWidth: content.getBoundingClientRect().width,
      mainWidth: main.getBoundingClientRect().width,
      paddingLeft: Number.parseFloat(getComputedStyle(main).paddingLeft),
      paddingRight: Number.parseFloat(getComputedStyle(main).paddingRight),
      primaryWidth: primary?.getBoundingClientRect().width ?? null
    };
  });
  expect(geometry).not.toBeNull();
  const availableWidth = geometry.mainWidth - geometry.paddingLeft - geometry.paddingRight;
  expect(geometry.contentWidth / availableWidth).toBeGreaterThanOrEqual(0.96);
  if (geometry.primaryWidth !== null) {
    expect(geometry.primaryWidth / geometry.contentWidth).toBeGreaterThanOrEqual(0.96);
  }
}

for (const viewport of viewports) {
  authenticatedTest(`sign-in is pixel-perfect on ${viewport.name}`, async ({ browser }) => {
    const page = await browser.newPage({
      ...devices["Desktop Chrome"],
      baseURL: adminOrigin,
      viewport
    });
    await page.goto("/admin/sign_in");
    await expect(page.getByRole("button", { name: "sign_in" })).toBeVisible();
    await expectPixelPerfect(page, `sign-in-${viewport.name}`, []);
    await page.close();
  });

  if (viewport.name === "desktop") {
    authenticatedTest("role rules filter is pixel-perfect on desktop", async ({ page }) => {
      await page.setViewportSize(viewport);
      await page.goto("/admin/role_rules");
      await page
        .locator('th[data-field="role_id"] .table-column-filter > button')
        .click();
      const filter = page.getByRole("dialog", { name: "filter_role_id" });
      await expect(filter).toBeVisible();
      await expectComponentPixelPerfect(
        page,
        filter,
        "role-rules-filter-desktop"
      );
    });
  } else {
    authenticatedTest("expanded navigation is pixel-perfect on mobile", async ({ page }) => {
      await page.setViewportSize(viewport);
      await page.goto("/admin/users");
      const navigationToggle = page.getByText("navigation", { exact: true });
      if (await navigationToggle.isVisible()) await navigationToggle.click();

      const navigation = page.locator('header nav[data-name="NavigationMenu"]');
      await expect(navigation).toBeVisible();
      expect(
        await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth)
      ).toBeLessThanOrEqual(1);
      await expectPixelPerfect(page, "navigation-open-mobile", []);
    });
  }

  for (const pageSpec of authenticatedPages) {
    authenticatedTest(`${pageSpec.name} is pixel-perfect on ${viewport.name}`, async ({ page }) => {
      await page.setViewportSize(viewport);
      const response = await page.goto(pageSpec.path);
      expect(response).not.toBeNull();
      expect(response.status()).toBe(200);
      await expect(page.locator("main")).toBeVisible();
      await expect(
        page.locator(
          `header nav a[href="${pageSpec.path}"][aria-current="page"]`
        )
      ).toHaveCount(1, { timeout: 15_000 });
      await expect(page.getByRole("alert")).toHaveCount(0);
      await expectPixelPerfect(
        page,
        `${pageSpec.name}-${viewport.name}`,
        pageSpec.mask
      );
    });
  }

  authenticatedTest(`users layout is pixel-perfect without masks on ${viewport.name}`, async ({
    page
  }) => {
    await page.setViewportSize(viewport);
    await page.goto("/admin/users?search=__visual_snapshot_no_match__");
    await expect(page.getByRole("table")).toBeVisible();
    await expect(page.locator("tbody tr")).toHaveCount(0);
    await expectPixelPerfect(page, `users-layout-${viewport.name}`, []);
  });

  for (const pageSpec of crudPages) {
    authenticatedTest(`${pageSpec.name} CRUD page is pixel-perfect on ${viewport.name}`, async ({
      page
    }) => {
      await page.setViewportSize(viewport);
      const response = await page.goto(pageSpec.path);
      expect(response).not.toBeNull();
      expect(response.status()).toBe(200);
      await expect(page.locator("main form").first()).toBeVisible();
      await expect(
        page.locator(
          `header nav a[href="${pageSpec.activePath}"][aria-current="page"]`
        )
      ).toHaveCount(1);
      await expectPixelPerfect(
        page,
        `${pageSpec.name}-${viewport.name}`,
        []
      );
    });
  }

  authenticatedTest(`unknown table error is pixel-perfect on ${viewport.name}`, async ({ page }) => {
    await page.setViewportSize(viewport);
    const response = await page.goto("/admin/tables");
    expect(response).not.toBeNull();
    expect(response.status()).toBe(422);
    await expectPixelPerfect(page, `unknown-table-${viewport.name}`, []);
  });

  authenticatedTest(`disabled OpenAPI error is pixel-perfect on ${viewport.name}`, async ({
    page
  }) => {
    await page.setViewportSize(viewport);
    const response = await page.goto("/admin/swagger_ui");
    expect(response).not.toBeNull();
    expect(response.status()).toBe(422);
    await expectPixelPerfect(page, `openapi-disabled-${viewport.name}`, []);
  });
}

authenticatedTest("all information pages use the wide desktop canvas", async ({ page }) => {
  authenticatedTest.setTimeout(120_000);
  await page.setViewportSize({ height: 1080, width: 1920 });
  const pages = [
    ...authenticatedPages.map(({ path }) => path),
    ...crudPages.map(({ path }) => path)
  ];
  for (const path of pages) {
    await authenticatedTest.step(path, async () => {
      const response = await page.goto(path);
      expect(response).not.toBeNull();
      await expect(page.locator("main")).toBeVisible();
      if (path !== "/admin/version") {
        await expect(page.locator(".table-page:visible, .crud-page:visible, .profile-grid:visible, .settings-grid:visible, .code-card:visible").first()).toBeVisible();
      }
      await expectInformationCanvasToUseAvailableWidth(page);
    });
  }
});
