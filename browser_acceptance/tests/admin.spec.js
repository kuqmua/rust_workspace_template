import { expect, test } from "@playwright/test";
import {
  changePassword,
  changedAdminPassword,
  initialAdminPassword,
  signInAdministrator
} from "./support/admin.js";
import { primaryAdminPaths } from "./support/pages.js";

test.describe.configure({ mode: "serial" });

async function firstCellStyle(page) {
  return page.locator("tbody td").first().evaluate(element => {
    const style = getComputedStyle(element);
    return {
      backgroundColor: style.backgroundColor,
      borderBottom: style.borderBottom,
      color: style.color,
      fontSize: style.fontSize,
      padding: style.padding
    };
  });
}

test("bootstrap sign-in forces a password change before administrator access", async ({
  page
}) => {
  const signInResponse = await page.goto("/admin/sign_in");
  expect(signInResponse).not.toBeNull();
  expect(signInResponse.status()).toBe(200);
  expect(signInResponse.headers()["content-security-policy"]).toContain(
    "default-src 'self'"
  );
  expect(signInResponse.headers()["x-content-type-options"]).toBe("nosniff");
  expect(signInResponse.headers()["x-frame-options"]).toBe("DENY");
  expect(signInResponse.headers()["referrer-policy"]).toBe("same-origin");

  await expect(page.locator(".password-policy")).toHaveCount(0);
  await expect(
    page.getByText("New passwords must contain", { exact: false })
  ).toHaveCount(0);
  await page.getByLabel("Login").fill("administrator");
  await page.getByLabel("Password").fill(initialAdminPassword);

  const signInCompleted = page.waitForResponse(
    response =>
      response.url().endsWith("/admin/actions/sign_in") &&
      response.status() === 303
  );
  await page.getByRole("button", { name: "Sign in" }).click();
  const response = await signInCompleted;
  const setCookie = (await response.headersArray())
    .filter(header => header.name.toLowerCase() === "set-cookie")
    .map(header => header.value);
  expect(setCookie).toEqual(
    expect.arrayContaining([
      expect.stringMatching(
        /admin_access_token=(?=.*SameSite=Strict)(?=.*HttpOnly)/
      ),
      expect.stringMatching(
        /admin_refresh_token=(?=.*SameSite=Strict)(?=.*HttpOnly)/
      ),
      expect.stringMatching(/admin_csrf_token=.*SameSite=Strict/)
    ])
  );
  expect(setCookie.find(value => value.startsWith("admin_csrf_token="))).not.toContain(
    "HttpOnly"
  );

  await expect(page).toHaveURL(/\/admin\/profile$/);
  await expect(page.locator(".password-policy")).toContainText(
    "uppercase, lowercase, digit, and special characters"
  );
  await page.goto("/admin/users");
  await expect(page).toHaveURL(/\/admin\/profile$/);

  await page.getByLabel("Current password").fill(initialAdminPassword);
  await page.getByLabel("New password").fill("admin");
  await page.getByRole("button", { name: "Change password" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Check both passwords and ensure the new password satisfies the policy."
  );

  await changePassword(page, initialAdminPassword, changedAdminPassword);
  await expect(page).toHaveURL(/\/admin\/profile$/);

  await page.goto("/admin/users");
  await expect(page).toHaveURL(/\/admin\/users$/);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(page.locator("tbody tr").filter({ hasText: "administrator" })).toBeVisible();
});

test("administrator users page contains only its header, table, and pagination", async ({
  page
}) => {
  await signInAdministrator(page);

  await expect(page.locator("header.topbar")).toHaveCount(1);
  await expect(page.getByRole("table")).toHaveCount(1);
  await expect(page.locator("nav.table-pagination")).toHaveCount(1);
  await expect(page.locator("form.table-tools")).toHaveCount(0);
  await expect(page.locator("form.mutation-form")).toHaveCount(0);
  await expect(page.locator("tbody button, tbody input, tbody select")).toHaveCount(0);
  await expect(page.locator("thead th")).toHaveCount(5);
  const usersCellStyle = await firstCellStyle(page);
  await page.goto("/admin/permissions");
  expect(usersCellStyle).toEqual(await firstCellStyle(page));

  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.locator("nav[aria-label='Admin sections']")).toBeVisible();
  await expect(page.locator(".table-scroll")).toBeVisible();
  await page.setViewportSize({ width: 1440, height: 900 });
  await expect(page.getByRole("table")).toBeVisible();
});

test("administrator roles page contains only its header, table, and pagination", async ({
  page
}) => {
  await signInAdministrator(page);
  await page.goto("/admin/roles");

  await expect(page.locator("header.topbar")).toHaveCount(1);
  await expect(page.getByRole("table")).toHaveCount(1);
  await expect(page.locator("nav.table-pagination")).toHaveCount(1);
  await expect(page.locator("form.table-tools")).toHaveCount(0);
  await expect(page.locator("form.mutation-form")).toHaveCount(0);
  await expect(page.locator("tbody button, tbody input, tbody select")).toHaveCount(0);
  await expect(page.locator("thead th")).toHaveCount(4);
  const rolesCellStyle = await firstCellStyle(page);
  await page.goto("/admin/permissions");
  expect(rolesCellStyle).toEqual(await firstCellStyle(page));
});

test("administrator permissions page contains only its header, table, and pagination", async ({
  page
}) => {
  await signInAdministrator(page);
  await page.goto("/admin/permissions");

  await expect(page.locator("header.topbar")).toHaveCount(1);
  await expect(page.getByRole("table")).toHaveCount(1);
  await expect(page.locator("nav.table-pagination")).toHaveCount(1);
  await expect(page.locator("form.table-tools")).toHaveCount(0);
});

test("data-table filter places a full-width Close control below Apply", async ({
  page
}) => {
  await signInAdministrator(page);
  await page.goto("/admin/role_permissions");

  const filter = page.locator(
    'th[data-field="role_id"] details.table-column-filter'
  );
  await filter.locator("summary").click();

  const dialog = filter.getByRole("dialog");
  const close = dialog.getByRole("button", { name: "Close" });
  await expect(dialog).toBeVisible();
  await expect(close).toContainText("Close");

  const controls = await filter.evaluate(element => {
    const applyRect = element
      .querySelector("button[type='submit']")
      .getBoundingClientRect();
    const closeRect = element
      .querySelector("button.table-filter-close")
      .getBoundingClientRect();
    return {
      applyBottom: applyRect.bottom,
      applyHeight: applyRect.height,
      applyWidth: applyRect.width,
      closeHeight: closeRect.height,
      closeTop: closeRect.top,
      closeWidth: closeRect.width
    };
  });
  expect(controls.closeTop).toBeGreaterThan(controls.applyBottom);
  expect(controls.closeWidth).toBe(controls.applyWidth);
  expect(controls.closeHeight).toBe(controls.applyHeight);

  await close.click();
  await expect(dialog).not.toBeVisible();
});

test("keyboard navigation reaches every primary administrator route", async ({ page }) => {
  await signInAdministrator(page);

  for (const path of primaryAdminPaths) {
    await page.goto(path);
    await expect(page).toHaveURL(new RegExp(`${path.replaceAll("/", "\\/")}$`));
    await expect(page.locator("main")).toBeVisible();
  }

  await page.goto("/admin/users");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await page.keyboard.press("Tab");
  await expect
    .poll(() => page.evaluate(() => document.activeElement?.tagName))
    .not.toBe("BODY");
});
