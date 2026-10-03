import { execFile } from "node:child_process";
import { promisify } from "node:util";
import { expect, test } from "@playwright/test";
import {
  adminHeaders,
  adminOrigin,
  changePassword,
  cookieValue,
  observeBrowserErrors,
  signIn,
  signInAdministrator,
  signOutIfAuthenticated
} from "./support/admin.js";
import {
  adminPages,
  diagnosticAdminPaths,
  mobileAdminPaths
} from "./support/pages.js";

test.describe.configure({ mode: "serial" });
test.skip(
  process.env.BROWSER_ACCEPTANCE_FULL !== "1",
  "production-readiness scenarios run on the scheduled matrix"
);

async function createUser(page, login, displayName, password) {
  const response = await page.request.post("/users/create", {
    data: [{
      display_name: displayName,
      login,
      password
    }],
    headers: await adminHeaders(page.context())
  });
  expect(response.status()).toBe(201);
  return (await response.json())[0];
}

async function createRole(page, name) {
  const response = await page.request.post("/roles/create", {
    data: [{ name }],
    headers: await adminHeaders(page.context())
  });
  expect(response.status()).toBe(201);
  return (await response.json())[0];
}

async function deleteUser(page, userId) {
  const response = await page.request.delete("/users/delete", {
    data: { filter: { user_id: userId } },
    headers: await adminHeaders(page.context())
  });
  expect(response.status()).toBe(204);
}

async function readPermissionRuleIds(page) {
  const response = page.waitForResponse(value =>
    new URL(value.url()).pathname === "/rules/read" && value.request().method() === "POST"
  );
  await page.goto("/admin/rules?limit=100");
  const table = await (await response).json();
  const idIndex = table.columns.findIndex(column => column.name === "id");
  const actionIndex = table.columns.findIndex(column => column.name === "permission_resource_action_id");
  return new Map(table.items.map(item => [Number(item.values[actionIndex]), Number(item.values[idIndex])]));
}

async function changeRequiredPassword(page, currentPassword, newPassword) {
  await expect(page).toHaveURL(/\/admin\/profile$/);
  await changePassword(page, currentPassword, newPassword);
}

test.afterEach(async ({ page }) => {
  await signOutIfAuthenticated(page);
});

test("sign-out clears all browser credentials", async ({
  page
}) => {
  await signInAdministrator(page);
  const cookies = await page.context().cookies();
  const access = cookieValue(cookies, "admin_access_token");
  const refresh = cookieValue(cookies, "admin_refresh_token");
  const csrf = cookieValue(cookies, "admin_csrf_token");
  expect(access).toBeTruthy();
  expect(refresh).toBeTruthy();
  expect(csrf).toBeTruthy();

  await page.locator("header form button").click();
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
  const remainingNames = (await page.context().cookies()).map(cookie => cookie.name);
  expect(remainingNames).not.toContain("admin_access_token");
  expect(remainingNames).not.toContain("admin_refresh_token");
  expect(remainingNames).not.toContain("admin_csrf_token");
});

test("logout prevents browser history from restoring an authenticated page", async ({
  page
}) => {
  await signInAdministrator(page);
  await page.goto("/admin/settings");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await page.locator("header form button").click();
  await expect(page).toHaveURL(/\/admin\/sign_in$/);

  await page.goBack();
  await page.reload();
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
  await expect(page.locator("header.topbar")).toHaveCount(0);
  await expect(page.getByRole("button", { name: "sign_in" })).toBeVisible();
});

test("public deployment contracts are checked by Rust", async () => {
  test.setTimeout(180_000);
  await promisify(execFile)("cargo", ["test", "--locked", "-p", "runtime_tests", "--test", "test_admin_deployment", "tests::test_public_admin_deployment_endpoints", "--", "--ignored", "--exact"], {
    cwd: new URL("../", import.meta.url),
    env: { ...process.env, SERVICE_SOCKET_ADDRESS: "127.0.0.1:18080" }
  });
});

test("user changes are visible through the read-only UI", async ({
  page
}) => {
  await signInAdministrator(page);
  const userId = await createUser(
    page,
    "production_user",
    "Production User",
    "Production-password3!"
  );

  const renamed = await page.request.patch("/users/update", {
    data: {
      updates: [{
        filter: { user_id: userId },
        changes: { display_name: "Renamed Production User", login: null }
      }]
    },
    headers: await adminHeaders(page.context())
  });
  expect(renamed.status()).toBe(204);

  await page.goto("/admin/users?limit=100");
  const row = page.locator("tbody tr").filter({ hasText: "production_user" });
  await expect(row).toContainText("Renamed Production User");
  await expect(row.locator("input, select, textarea")).toHaveCount(0);
  await expect(row.locator('td[data-label="actions"]').getByRole("link", { name: "read", exact: true })).toHaveCount(1);

  const deleted = await page.request.delete("/users/delete", {
    data: { filter: { user_id: userId } },
    headers: await adminHeaders(page.context())
  });
  expect(deleted.status()).toBe(204);
  await page.reload();
  await expect(
    page.locator("tbody tr").filter({ hasText: "production_user" })
  ).toHaveCount(0);
});

test("administrator password reset invalidates the old session and credentials", async ({
  browser,
  page
}) => {
  await signInAdministrator(page);
  const userId = await createUser(
    page,
    "password_lifecycle_user",
    "Password Lifecycle User",
    "Lifecycle-password1!"
  );
  const userContext = await browser.newContext({ baseURL: adminOrigin });
  const userPage = await userContext.newPage();
  await signIn(
    userPage,
    "password_lifecycle_user",
    "Lifecycle-password1!"
  );
  await changeRequiredPassword(
    userPage,
    "Lifecycle-password1!",
    "Lifecycle-password2!"
  );

  const reset = await page.request.patch(
    "/users/update",
    {
      data: { updates: [{ filter: { user_id: userId }, changes: { password: "Lifecycle-password3!" } }] },
      headers: await adminHeaders(page.context())
    }
  );
  expect(reset.status()).toBe(204);
  await userContext.close();

  const oldContext = await browser.newContext({ baseURL: adminOrigin });
  const oldPage = await oldContext.newPage();
  await signIn(
    oldPage,
    "password_lifecycle_user",
    "Lifecycle-password2!"
  );
  await expect(oldPage.getByRole("alert")).toBeVisible();
  await oldContext.close();

  const resetContext = await browser.newContext({ baseURL: adminOrigin });
  const resetPage = await resetContext.newPage();
  await signIn(
    resetPage,
    "password_lifecycle_user",
    "Lifecycle-password3!"
  );
  await expect(resetPage).toHaveURL(/\/admin\/profile$/);
  await resetContext.close();
  await deleteUser(page, userId);
});

test("banning blocks browser sign-in and unbanning restores access", async ({
  browser,
  page
}) => {
  await signInAdministrator(page);
  const userId = await createUser(
    page,
    "ban_lifecycle_user",
    "Ban Lifecycle User",
    "Ban-password1!"
  );
  const userContext = await browser.newContext({ baseURL: adminOrigin });
  const userPage = await userContext.newPage();
  await signIn(userPage, "ban_lifecycle_user", "Ban-password1!");
  await expect(userPage).toHaveURL(/\/admin\/profile$/);

  const banned = await page.request.patch("/users/update", {
    data: { updates: [{ filter: { user_id: userId }, changes: { is_banned: true } }] },
    headers: await adminHeaders(page.context())
  });
  expect(banned.status()).toBe(204);
  await userPage.goto("/admin/sign_in");
  await signIn(userPage, "ban_lifecycle_user", "Ban-password1!");
  await expect(userPage.getByRole("alert")).toBeVisible();

  const unbanned = await page.request.patch("/users/update", {
    data: { updates: [{ filter: { user_id: userId }, changes: { is_banned: false } }] },
    headers: await adminHeaders(page.context())
  });
  expect(unbanned.status()).toBe(204);
  await signIn(userPage, "ban_lifecycle_user", "Ban-password1!");
  await expect(userPage).toHaveURL(/\/admin\/profile$/);
  await userContext.close();
  await deleteUser(page, userId);
});

test("role changes and deletion are reflected in administrator pages", async ({
  page
}) => {
  await signInAdministrator(page);
  const roleId = await createRole(page, "lifecycle_role");
  const renamed = await page.request.patch("/roles/update", {
    data: { updates: [{ filter: { role_id: roleId }, changes: { name: "renamed_lifecycle_role" } }] },
    headers: await adminHeaders(page.context())
  });
  expect(renamed.status()).toBe(204);

  await page.goto("/admin/roles");
  const systemRole = page.locator("tbody tr").first();
  await expect(systemRole.locator('td[data-label="is_system"]')).toHaveText("true");
  const deleted = await page.request.delete("/roles/delete", {
    data: { filter: { role_id: roleId } },
    headers: await adminHeaders(page.context())
  });
  expect(deleted.status()).toBe(204);
  await page.goto("/admin/roles/manage");
  await expect(page.locator('input[name="name"][value="renamed_lifecycle_role"]')).toHaveCount(0);
});

test("search and sorting query survives UI reloads", async ({
  page
}) => {
  await signInAdministrator(page);
  const alphaUserId = await createUser(
    page,
    "query_alpha_user",
    "Query Alpha User",
    "Query-password1!"
  );
  const zetaUserId = await createUser(
    page,
    "query_zeta_user",
    "Query Zeta User",
    "Query-password2!"
  );

  const query =
    "search=query_alpha_user&sort=login&direction=descending&limit=1&offset=0";
  await page.goto(`/admin/users?${query}`);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(page.locator("tbody tr")).toHaveCount(1);
  await expect(page.locator("tbody tr")).toContainText("query_alpha_user");
  await page.reload();
  await expect(page).toHaveURL(new RegExp(`/admin/users\\?${query}$`));
  await expect(page.locator("tbody tr")).toContainText("query_alpha_user");
  await deleteUser(page, alphaUserId);
  await deleteUser(page, zetaUserId);
});

test("a read-only administrator sees only authorized navigation and mutations fail", async ({
  browser,
  page
}) => {
  await signInAdministrator(page);
  const roleId = await createRole(page, "production_reader");
  const rules = await readPermissionRuleIds(page);
  const ruleIds = [22, 19, 30, 2].map(actionId => rules.get(actionId));
  expect(ruleIds.every(ruleId => ruleId !== undefined)).toBe(true);
  const roleRules = await page.request.patch("/roles/update", {
    data: { updates: [{ filter: { role_id: roleId }, changes: { rules: { expected_rule_ids: [], rule_ids: ruleIds } } }] },
    headers: await adminHeaders(page.context())
  });
  expect(roleRules.status()).toBe(204);

  const userId = await createUser(
    page,
    "production_reader",
    "Production Reader",
    "Reader-password4!"
  );
  const userRoles = await page.request.patch("/users/update", {
    data: { updates: [{ filter: { user_id: userId }, changes: {
      expected_role_ids: [],
      role_ids: [roleId]
    } }] },
    headers: await adminHeaders(page.context())
  });
  expect(userRoles.status()).toBe(204);

  const context = await browser.newContext({ baseURL: adminOrigin });
  const reader = await context.newPage();
  await signIn(reader, "production_reader", "Reader-password4!");
  await expect(reader).toHaveURL(/\/admin\/profile$/);
  await reader.getByLabel("current_password").fill("Reader-password4!");
  await reader.getByLabel("new_password").fill("Reader-password5!");
  const passwordChanged = reader.waitForResponse(
    response =>
      response.url().endsWith("/auth/password") &&
      response.status() === 204
  );
  await reader.getByRole("button", { name: "change_password" }).click();
  await passwordChanged;
  await expect(reader).toHaveURL(/\/admin\/profile$/);

  await reader.goto("/admin/users");
  await expect(reader.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(
    reader.locator('nav[aria-label="admin_sections"] a[href="/admin/users"]')
  ).toBeVisible();
  await expect(
    reader.locator('nav[aria-label="admin_sections"] a[href="/admin/roles"]')
  ).toHaveCount(0);
  await expect(
    reader.locator('nav[aria-label="admin_sections"] a[href="/admin/settings"]')
  ).toBeVisible();

  const auditLogRead = reader.waitForRequest(
    request =>
      request.url().endsWith("/audit_log/read") && request.method() === "POST"
  );
  await reader.goto("/admin/audit_log");
  await auditLogRead;
  await expect(reader.locator('[data-renderer="csr"]')).toBeVisible();

  const systemSettingsRead = reader.waitForRequest(
    request =>
      request.url().endsWith("/system_settings/read") && request.method() === "POST"
  );
  await reader.goto("/admin/system_settings");
  await systemSettingsRead;
  await expect(reader.locator('[data-renderer="csr"]')).toBeVisible();

  await reader.goto("/admin/settings");
  await expect(reader.locator('[data-renderer="csr"]')).toBeVisible();
  const settingsControls = reader.locator(
    '[data-name="Input"], [data-name="Textarea"]'
  );
  expect(await settingsControls.count()).toBeGreaterThan(0);
  expect(
    await settingsControls.evaluateAll(elements =>
      elements.every(element => element.disabled)
    )
  ).toBe(true);
  await expect(reader.getByRole("button", { name: "save_settings" })).toBeDisabled();
  const reset = reader.getByRole("button", {
    name: "reset_to_template_defaults"
  });
  await expect(reset).toBeDisabled();
  await reset.click({ force: true });
  await expect(reader.getByRole("dialog", { name: "reset_settings" })).not.toBeVisible();

  const forbiddenPage = await reader.goto("/admin/roles");
  expect(forbiddenPage).not.toBeNull();
  expect(forbiddenPage.status()).toBe(403);
  const forbiddenApi = await reader.request.post("/users/create", {
    data: [{
      display_name: "Forbidden User",
      login: "forbidden_user",
      password: "Forbidden-password6!"
    }],
    headers: await adminHeaders(context)
  });
  expect(forbiddenApi.status()).toBe(403);
  await context.close();
  const deletedUser = await page.request.delete("/users/delete", {
    data: { filter: { user_id: userId } },
    headers: await adminHeaders(page.context())
  });
  expect(deletedUser.status()).toBe(204);
  const deletedRole = await page.request.delete("/roles/delete", {
    data: { filter: { role_id: roleId } },
    headers: await adminHeaders(page.context())
  });
  expect(deletedRole.status()).toBe(204);
});

test("a failed settings mutation preserves input and reports the server error", async ({
  page
}) => {
  await signInAdministrator(page);
  await page.goto("/admin/settings");
  const siteName = page.getByLabel("site_name");
  const originalSiteName = await siteName.inputValue();
  await siteName.fill("Unsaved Production Name");
  let intercepted = 0;
  let refreshes = 0;
  page.on("request", request => {
    if (request.url().endsWith("/auth/refresh")) refreshes += 1;
  });
  await page.route("**/system_settings/update", async route => {
    if (route.request().method() === "PATCH") {
      intercepted += 1;
      await route.fulfill({
        body: JSON.stringify({
          detail: "temporary failure",
          kind: "internal",
          request_id: "production-test",
          status: 503,
          violations: []
        }),
        contentType: "application/json",
        status: 503
      });
      return;
    }
    await route.continue();
  });

  await page.getByRole("button", { name: "save_settings" }).click();
  await expect(page.getByRole("alert")).toBeVisible();
  await expect(siteName).toHaveValue("Unsaved Production Name");
  expect(intercepted).toBe(1);
  expect(refreshes).toBe(0);

  await page.unroute("**/system_settings/update");
  const persisted = await page.request.get("/system_settings/read");
  expect(persisted.status()).toBe(200);
  expect((await persisted.json()).site_name).toBe(originalSiteName);
});

test("interactive controls remain named and keyboard reachable on mobile", async ({
  page
}) => {
  await page.setViewportSize({ height: 844, width: 390 });
  await signInAdministrator(page);
  for (const path of mobileAdminPaths) {
    await page.goto(path);
    await expect(page.locator("main")).toBeVisible();
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    const navigationToggle = page.getByText("navigation", { exact: true });
    if (await navigationToggle.isVisible()) await navigationToggle.click();
    await expect(page.getByRole("navigation", { name: "admin_sections" })).toBeVisible();
    const controls = page.locator(
      ":is(a, button, input:not([type='hidden']), select, textarea):visible"
    );
    const count = await controls.count();
    expect(count).toBeGreaterThan(0);
    for (let index = 0; index < count; index += 1) {
      await expect(controls.nth(index)).toHaveAccessibleName(/.+/);
    }
  }

  await page.goto("/admin/users");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await page.keyboard.press("Tab");
  await expect
    .poll(() => page.evaluate(() => document.activeElement?.tagName))
    .not.toBe("BODY");
  await expect(page.locator(".table-scroll")).toBeVisible();
});

test("primary pages emit no uncaught errors, failed requests, or console errors", async ({
  page
}) => {
  const { consoleErrors, failedRequests, pageErrors } = observeBrowserErrors(
    page,
    true
  );

  await signInAdministrator(page);
  for (const path of diagnosticAdminPaths) {
    const response = await page.goto(path);
    expect(response).not.toBeNull();
    expect(response.status()).toBe(200);
    await expect(page.locator("main")).toBeVisible();
  }
  expect(consoleErrors).toEqual([]);
  expect(failedRequests).toEqual([]);
  expect(pageErrors).toEqual([]);
});

test("expired CSRF cookies recover a settings mutation without replaying it", async ({ context, page }) => {
  await signInAdministrator(page);
  await page.goto("/admin/settings");
  const siteName = page.getByLabel("site_name");
  const original = await siteName.inputValue();
  await siteName.fill("Recovered administration");
  await context.clearCookies({ name: /admin_(access_token|csrf_token)/ });
  const responses = [];
  page.on("response", response => {
    if (response.request().method() === "PATCH" ||
        response.url().endsWith("/auth/refresh")) {
      responses.push([response.request().method(), response.status()]);
    }
  });
  const saved = page.waitForResponse(response =>
    response.request().method() === "PATCH" && response.status() === 204);
  await page.getByRole("button", { name: "save_settings" }).click();
  await saved;
  await expect(siteName).toHaveValue("Recovered administration");
  expect(responses).toEqual([["POST", 200], ["PATCH", 204]]);
  await page.reload();
  await expect(siteName).toHaveValue("Recovered administration");
  await siteName.fill(original);
  const restored = page.waitForResponse(response =>
    response.request().method() === "PATCH" && response.status() === 204);
  await page.getByRole("button", { name: "save_settings" }).click();
  await restored;
});
