import { expect, test } from "@playwright/test";
import {
  signInAdministrator,
  signOutIfAuthenticated
} from "./support/admin.js";
import {
  dataTablePages,
  navigationAdminPaths,
  serverRenderedPages,
  tablePages
} from "./support/pages.js";

test.describe.configure({ mode: "serial" });

test.afterEach(async ({ page }) => {
  await signOutIfAuthenticated(page);
});

async function openAdminPage(page, path) {
  const response = await page.goto(path);
  expect(response).not.toBeNull();
  expect(response.status()).toBe(200);
  await expect(page).toHaveURL(new RegExp(`${path.replaceAll("/", "\\/")}$`));
  await expect(page.locator("main")).toBeVisible();
  await expect(page.getByRole("alert")).toHaveCount(0);
}

async function expectAdminShell(page) {
  await expect(page.locator("header.topbar")).toHaveCount(1);
  await expect(page.locator("nav[aria-label='Admin sections']")).toBeVisible();
  await expect(page.locator("main.main-content")).toHaveCount(1);
  await expect(page.locator("header form button")).toHaveAccessibleName("sign_out");
}

async function expectUniqueIds(page) {
  const ids = await page.locator("[id]").evaluateAll(elements =>
    elements.map(element => element.id)
  );
  expect(new Set(ids).size).toBe(ids.length);
}

test("administrator root resolves to the configured default page", async ({ page }) => {
  await signInAdministrator(page);
  await page.goto("/admin");
  await expect(page).toHaveURL(/\/admin\/users$/);
  await expectAdminShell(page);
});

test("shared page catalog matches every rendered navigation destination", async ({
  page
}) => {
  await signInAdministrator(page);
  const destinations = await page
    .locator('nav[aria-label="Admin sections"] a')
    .evaluateAll(links => links.map(link => link.getAttribute("href")));
  expect(destinations).toEqual(navigationAdminPaths);
});

test("revisiting sign-in preserves the authenticated administrator session", async ({
  page
}) => {
  await signInAdministrator(page);
  await page.goto("/admin/sign_in");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
  await expect(page.getByRole("button", { name: "Sign in" })).toBeVisible();
  await page.goto("/admin/users");
  await expect(page).toHaveURL(/\/admin\/users$/);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
});

test("table route rejects an unknown table name", async ({ page }) => {
  await signInAdministrator(page);
  const response = await page.goto("/admin/tables");
  expect(response).not.toBeNull();
  expect(response.status()).toBe(422);
  await expect(page).toHaveURL(/\/admin\/tables$/);
});

test("unknown table rejection is stable after reload", async ({ page }) => {
  await signInAdministrator(page);
  await page.goto("/admin/tables");
  const response = await page.reload();
  expect(response).not.toBeNull();
  expect(response.status()).toBe(422);
  await expect(page).toHaveURL(/\/admin\/tables$/);
});

test("disabled OpenAPI route remains linked but rejects rendering", async ({
  page
}) => {
  await signInAdministrator(page);
  await expect(
    page.locator('nav[aria-label="Admin sections"] a[href="/admin/swagger_ui"]')
  ).toHaveCount(1);
  const response = await page.goto("/admin/swagger_ui");
  expect(response).not.toBeNull();
  expect(response.status()).toBe(422);
  await expect(page).toHaveURL(/\/admin\/swagger_ui$/);
});

for (const pageSpec of tablePages) {
  test(`${pageSpec.name} table has the expected semantic structure`, async ({ page }) => {
    await signInAdministrator(page);
    await openAdminPage(page, pageSpec.path);
    await expectAdminShell(page);
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    await expect(page.getByRole("table")).toHaveCount(1);
    await expect(page.locator("thead th")).toHaveText(pageSpec.headers);
    await expect(page.locator("tbody tr").first()).toBeVisible();
    await expect(page.locator("tbody tr").first().locator("td")).toHaveCount(
      pageSpec.headers.length
    );
    await expect(page.locator("nav.table-pagination")).toHaveCount(
      pageSpec.pagination ? 1 : 0
    );
  });

  test(`${pageSpec.name} table exposes an active navigation destination`, async ({
    page
  }) => {
    await signInAdministrator(page);
    await openAdminPage(page, pageSpec.path);
    const destination = page.locator(`nav[aria-label="Admin sections"] a[href="${pageSpec.path}"]`);
    await expect(destination).toHaveCount(1);
    await expect(destination).toHaveAttribute("aria-current", "page");
    await expectUniqueIds(page);
  });

  test(`${pageSpec.name} table survives a direct reload`, async ({ page }) => {
    const browserErrors = [];
    page.on("pageerror", error => browserErrors.push(error.message));
    await signInAdministrator(page);
    await openAdminPage(page, pageSpec.path);
    await page.reload();
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    await expect(page.getByRole("table")).toBeVisible();
    await expect(page.locator('[role="status"]')).toHaveCount(0);
    expect(browserErrors).toEqual([]);
  });

  if (pageSpec.readOnly) {
    test(`${pageSpec.name} rows contain no interactive controls`, async ({ page }) => {
      await signInAdministrator(page);
      await openAdminPage(page, pageSpec.path);
      await expect(
        page.locator("tbody button, tbody input, tbody select, tbody textarea")
      ).toHaveCount(0);
      await expect(page.locator("tbody td").first()).toHaveAttribute(
        "data-name",
        "TableCell"
      );
    });
  }
}

for (const pageSpec of dataTablePages) {
  test(`${pageSpec.name} data table renders its grid and pagination`, async ({ page }) => {
    await signInAdministrator(page);
    await openAdminPage(page, pageSpec.path);
    await expectAdminShell(page);
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    await expect(page.getByRole("table")).toHaveCount(1);
    await expect(page.locator("thead th").first()).toBeVisible();
    await expect(page.locator("nav.table-pagination")).toHaveCount(1);
    await expect(
      page.locator(`nav[aria-label="Admin sections"] a[href="${pageSpec.path}"]`)
    ).toHaveAttribute("aria-current", "page");
  });

  test(`${pageSpec.name} data table keeps its deep link after reload`, async ({ page }) => {
    const browserErrors = [];
    page.on("pageerror", error => browserErrors.push(error.message));
    await signInAdministrator(page);
    await openAdminPage(page, pageSpec.path);
    await page.reload();
    await expect(page).toHaveURL(
      new RegExp(`${pageSpec.path.replaceAll("/", "\\/")}$`)
    );
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    await expect(page.getByRole("table")).toBeVisible();
    await expectUniqueIds(page);
    expect(browserErrors).toEqual([]);
  });
}

test("profile page exposes account details and a labeled password form", async ({
  page
}) => {
  await signInAdministrator(page);
  await openAdminPage(page, "/admin/profile");
  await expectAdminShell(page);
  await expect(page.locator(".profile-card")).toBeVisible();
  await expect(page.getByLabel("Current password")).toHaveAttribute("type", "password");
  await expect(page.getByLabel("New password")).toHaveAttribute("type", "password");
  await expect(page.getByRole("button", { name: "Change password" })).toBeEnabled();
});

test("profile page preserves its content after reload", async ({ page }) => {
  await signInAdministrator(page);
  await openAdminPage(page, "/admin/profile");
  await page.reload();
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(page.getByLabel("Current password")).toHaveValue("");
  await expect(page.getByLabel("New password")).toHaveValue("");
  await expectUniqueIds(page);
});

test("settings page exposes labeled editable settings", async ({ page }) => {
  await signInAdministrator(page);
  await openAdminPage(page, "/admin/settings");
  await expectAdminShell(page);
  await expect(page.locator("form.settings-form")).toHaveCount(1);
  await expect(page.getByLabel("Site name")).toBeEditable();
  await expect(page.getByLabel("Tab title")).toBeEditable();
  await expect(page.getByLabel("Default route")).toBeEditable();
  await expect(page.getByRole("button", { name: "Save settings" })).toBeEnabled();
  await expect(
    page.getByRole("button", { name: "Reset to template defaults" })
  ).toBeEnabled();
});

test("settings page preserves server values after reload", async ({ page }) => {
  await signInAdministrator(page);
  await openAdminPage(page, "/admin/settings");
  const siteName = await page.getByLabel("Site name").inputValue();
  const tabTitle = await page.getByLabel("Tab title").inputValue();
  const defaultRoute = await page.getByLabel("Default route").inputValue();
  await page.reload();
  await expect(page.getByLabel("Site name")).toHaveValue(siteName);
  await expect(page.getByLabel("Tab title")).toHaveValue(tabTitle);
  await expect(page.getByLabel("Default route")).toHaveValue(defaultRoute);
  await expectUniqueIds(page);
});

for (const pageSpec of serverRenderedPages) {
  test(`${pageSpec.name} server-rendered page has the admin shell`, async ({ page }) => {
    await signInAdministrator(page);
    await openAdminPage(page, pageSpec.path);
    await expectAdminShell(page);
    await expect(page.locator("main")).not.toBeEmpty();
    await expect(
      page.locator(`nav[aria-label="Admin sections"] a[href="${pageSpec.path}"]`)
    ).toHaveAttribute("aria-current", "page");
  });

  test(`${pageSpec.name} server-rendered page reloads without client errors`, async ({
    page
  }) => {
    const browserErrors = [];
    page.on("pageerror", error => browserErrors.push(error.message));
    await signInAdministrator(page);
    await openAdminPage(page, pageSpec.path);
    const content = await page.locator("main").innerText();
    await page.reload();
    if (pageSpec.dynamic) {
      await expect(page.locator("main")).not.toBeEmpty();
    } else {
      await expect(page.locator("main")).toContainText(content);
    }
    await expectUniqueIds(page);
    expect(browserErrors).toEqual([]);
  });
}
