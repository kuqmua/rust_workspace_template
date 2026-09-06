import { expect, test } from "@playwright/test";
import { readFile } from "node:fs/promises";
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
  const response = await page.request.post("/v1/admin/users", {
    data: {
      display_name: displayName,
      login,
      password
    },
    headers: await adminHeaders(page.context())
  });
  expect(response.status()).toBe(201);
  return (await response.json()).id;
}

async function createRole(page, name) {
  const response = await page.request.post("/v1/admin/roles", {
    data: { name },
    headers: await adminHeaders(page.context())
  });
  expect(response.status()).toBe(201);
  return (await response.json()).id;
}

async function changeRequiredPassword(page, currentPassword, newPassword) {
  await expect(page).toHaveURL(/\/admin\/profile$/);
  await changePassword(page, currentPassword, newPassword);
}

test.afterEach(async ({ page }) => {
  await signOutIfAuthenticated(page);
});

test("mutations reject missing, invalid, and cross-origin CSRF credentials", async ({
  browser,
  page
}) => {
  await signInAdministrator(page);
  const original = await page.request.get("/v1/admin/system_settings");
  expect(original.status()).toBe(200);
  const originalSettings = await original.json();
  const update = {
    clear: [],
    site_name: "CSRF must not persist"
  };

  const missing = await page.request.patch("/v1/admin/system_settings", {
    data: update,
    headers: { Origin: adminOrigin }
  });
  expect([401, 403]).toContain(missing.status());

  const invalid = await page.request.patch("/v1/admin/system_settings", {
    data: update,
    headers: {
      Origin: adminOrigin,
      "X-CSRF-Token": "invalid-csrf-token"
    }
  });
  expect([401, 403]).toContain(invalid.status());

  const otherContext = await browser.newContext({ baseURL: adminOrigin });
  const otherPage = await otherContext.newPage();
  await signInAdministrator(otherPage);
  const otherCsrf = cookieValue(
    await otherContext.cookies(),
    "admin_csrf_token"
  );
  expect(otherCsrf).toBeTruthy();
  const otherSessionToken = await page.request.patch(
    "/v1/admin/system_settings",
    {
      data: update,
      headers: {
        Origin: adminOrigin,
        "X-CSRF-Token": otherCsrf
      }
    }
  );
  expect([401, 403]).toContain(otherSessionToken.status());
  await otherContext.close();

  const crossOrigin = await page.request.patch("/v1/admin/system_settings", {
    data: update,
    headers: {
      ...(await adminHeaders(page.context())),
      Origin: "https://attacker.invalid"
    }
  });
  expect([401, 403]).toContain(crossOrigin.status());

  const unchanged = await page.request.get("/v1/admin/system_settings");
  expect(unchanged.status()).toBe(200);
  expect(await unchanged.json()).toEqual(originalSettings);
});

test("sign-out clears all credentials and a captured refresh token cannot be replayed", async ({
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

  const replay = await page.request.post("/v1/admin/auth/refresh", {
    data: {},
    headers: {
      Cookie: `admin_refresh_token=${refresh}; admin_csrf_token=${csrf}`,
      Origin: adminOrigin,
      "X-CSRF-Token": csrf
    }
  });
  expect(replay.status()).toBe(401);
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
  await expect(page.getByRole("button", { name: "Sign in" })).toBeVisible();
});

test("HTML success and error responses retain production security headers", async ({
  page
}) => {
  await signInAdministrator(page);
  for (const path of [
    adminPages.users.path,
    "/admin/tables",
    "/admin/swagger_ui"
  ]) {
    const response = await page.goto(path);
    expect(response).not.toBeNull();
    expect([200, 422]).toContain(response.status());
    const headers = response.headers();
    expect(headers["content-security-policy"]).toContain("default-src 'self'");
    expect(headers["content-security-policy"]).toContain("style-src 'self'");
    expect(headers["x-content-type-options"]).toBe("nosniff");
    expect(headers["x-frame-options"]).toBe("DENY");
    expect(headers["referrer-policy"]).toBe("same-origin");
  }
});

test("public health, branding, and static asset endpoints are deployable", async ({
  request
}) => {
  for (const path of ["/health/live", "/health/ready"]) {
    const response = await request.get(path);
    expect(response.status()).toBe(200);
  }

  const branding = await request.get("/v1/admin/branding");
  expect(branding.status()).toBe(200);
  expect(await branding.json()).toEqual(
    expect.objectContaining({
      default_admin_route: "/admin/users",
      site_name: expect.any(String)
    })
  );

  const stylesheet = await request.get("/admin/assets/style.css");
  expect(stylesheet.status()).toBe(200);
  expect(stylesheet.headers()["content-type"]).toContain("text/css");
  expect(stylesheet.headers()["x-content-type-options"]).toBe("nosniff");
  expect((await stylesheet.body()).byteLength).toBeGreaterThan(1_000);
});

test("oversized and invalid mutations fail without changing persisted state", async ({
  page
}) => {
  await signInAdministrator(page);
  const originalResponse = await page.request.get(
    "/v1/admin/system_settings"
  );
  expect(originalResponse.status()).toBe(200);
  const original = await originalResponse.json();

  const invalid = await page.request.patch("/v1/admin/system_settings", {
    data: {
      clear: [],
      default_admin_route: "/outside-admin"
    },
    headers: await adminHeaders(page.context())
  });
  expect(invalid.status()).toBe(422);

  const headers = await adminHeaders(page.context());
  const oversizedStatus = await page.evaluate(async csrfToken => {
    const response = await fetch("/v1/admin/users", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "X-CSRF-Token": csrfToken
      },
      body: JSON.stringify({
        display_name: "x".repeat(1_100_000),
        login: "oversized_user",
        password: "Oversized-password8!"
      })
    });
    return response.status;
  }, headers["X-CSRF-Token"]);
  expect(oversizedStatus).toBe(413);

  const unchanged = await page.request.get("/v1/admin/system_settings");
  expect(unchanged.status()).toBe(200);
  expect(await unchanged.json()).toEqual(original);
  const absent = await page.request.get(
    "/v1/admin/users?search=oversized_user"
  );
  expect(absent.status()).toBe(200);
  expect((await absent.json()).items).toHaveLength(0);
});

test("user CRUD rejects duplicates and is visible through the read-only UI", async ({
  page
}) => {
  await signInAdministrator(page);
  const userId = await createUser(
    page,
    "production_user",
    "Production User",
    "Production-password3!"
  );

  const duplicate = await page.request.post("/v1/admin/users", {
    data: {
      display_name: "Duplicate User",
      login: "production_user",
      password: "Production-password3!"
    },
    headers: await adminHeaders(page.context())
  });
  expect([409, 422]).toContain(duplicate.status());

  const renamed = await page.request.patch(`/v1/admin/users/${userId}`, {
    data: {
      display_name: "Renamed Production User",
      login: null
    },
    headers: await adminHeaders(page.context())
  });
  expect(renamed.status()).toBe(204);

  await page.goto("/admin/users?limit=100");
  const row = page.locator("tbody tr").filter({ hasText: "production_user" });
  await expect(row).toContainText("Renamed Production User");
  await expect(row.locator("button, input, select, textarea")).toHaveCount(0);

  const deleted = await page.request.delete(`/v1/admin/users/${userId}`, {
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
  expect(
    (await userPage.request.get("/v1/admin/auth/me")).status()
  ).toBe(200);

  const reset = await page.request.post(
    `/v1/admin/users/${userId}/password`,
    {
      data: { password: "Lifecycle-password3!" },
      headers: await adminHeaders(page.context())
    }
  );
  expect(reset.status()).toBe(204);
  expect(
    (await userPage.request.get("/v1/admin/auth/me")).status()
  ).toBe(401);
  await userContext.close();

  const oldContext = await browser.newContext({ baseURL: adminOrigin });
  const oldPage = await oldContext.newPage();
  await signIn(
    oldPage,
    "password_lifecycle_user",
    "Lifecycle-password2!"
  );
  await expect(oldPage.getByRole("alert")).toBeVisible();
  expect(
    (await oldPage.request.get("/v1/admin/auth/me")).status()
  ).toBe(401);
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
});

test("banning a user revokes active sessions and unbanning restores sign-in", async ({
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

  const banned = await page.request.post(
    `/v1/admin/users/${userId}/ban`,
    {
      data: { is_banned: true },
      headers: await adminHeaders(page.context())
    }
  );
  expect(banned.status()).toBe(204);
  expect(
    (await userPage.request.get("/v1/admin/auth/me")).status()
  ).toBe(401);
  await userPage.goto("/admin/sign_in");
  await signIn(userPage, "ban_lifecycle_user", "Ban-password1!");
  await expect(userPage.getByRole("alert")).toBeVisible();

  const unbanned = await page.request.post(
    `/v1/admin/users/${userId}/ban`,
    {
      data: { is_banned: false },
      headers: await adminHeaders(page.context())
    }
  );
  expect(unbanned.status()).toBe(204);
  await signIn(userPage, "ban_lifecycle_user", "Ban-password1!");
  await expect(userPage).toHaveURL(/\/admin\/profile$/);
  await userContext.close();
});

test("role lifecycle enforces uniqueness, stale-assignment conflicts, and deletion", async ({
  page
}) => {
  await signInAdministrator(page);
  const roleId = await createRole(page, "lifecycle_role");
  const duplicate = await page.request.post("/v1/admin/roles", {
    data: { name: "lifecycle_role" },
    headers: await adminHeaders(page.context())
  });
  expect([409, 422]).toContain(duplicate.status());

  const renamed = await page.request.patch(`/v1/admin/roles/${roleId}`, {
    data: { name: "renamed_lifecycle_role" },
    headers: await adminHeaders(page.context())
  });
  expect(renamed.status()).toBe(204);

  const permissionsResponse = await page.request.get(
    "/v1/admin/permissions?limit=100"
  );
  expect(permissionsResponse.status()).toBe(200);
  const permission = (await permissionsResponse.json()).items.find(
    item => item.name === "users:read"
  );
  expect(permission).toBeTruthy();
  const assigned = await page.request.put(
    `/v1/admin/roles/${roleId}/permissions`,
    {
      data: {
        expected_permission_ids: [],
        permission_ids: [permission.id]
      },
      headers: await adminHeaders(page.context())
    }
  );
  expect(assigned.status()).toBe(204);
  const stale = await page.request.put(
    `/v1/admin/roles/${roleId}/permissions`,
    {
      data: {
        expected_permission_ids: [],
        permission_ids: []
      },
      headers: await adminHeaders(page.context())
    }
  );
  expect(stale.status()).toBe(409);

  const rolesBeforeDeletion = await page.request.get(
    "/v1/admin/roles?limit=100"
  );
  expect(rolesBeforeDeletion.status()).toBe(200);
  const systemRole = (await rolesBeforeDeletion.json()).items.find(
    role => role.is_system
  );
  expect(systemRole).toBeTruthy();
  const protectedSystemRole = await page.request.delete(
    `/v1/admin/roles/${systemRole.id}`,
    {
      headers: await adminHeaders(page.context())
    }
  );
  expect([409, 422]).toContain(protectedSystemRole.status());

  const deleted = await page.request.delete(`/v1/admin/roles/${roleId}`, {
    headers: await adminHeaders(page.context())
  });
  expect(deleted.status()).toBe(204);
  const roles = await page.request.get(
    "/v1/admin/roles?search=renamed_lifecycle_role&limit=100"
  );
  expect(roles.status()).toBe(200);
  expect((await roles.json()).items).toHaveLength(0);
});

test("pagination has stable non-overlapping pages and rejects invalid bounds", async ({
  page
}) => {
  await signInAdministrator(page);
  for (const index of [1, 2, 3]) {
    await createUser(
      page,
      `page_user_${index}`,
      `Page User ${index}`,
      `Pagination-password${index}!`
    );
  }

  const firstResponse = await page.request.get("/v1/admin/users?limit=2&offset=0");
  const secondResponse = await page.request.get("/v1/admin/users?limit=2&offset=2");
  expect(firstResponse.status()).toBe(200);
  expect(secondResponse.status()).toBe(200);
  const first = await firstResponse.json();
  const second = await secondResponse.json();
  expect(first.items).toHaveLength(2);
  expect(second.items).toHaveLength(2);
  expect(new Set(first.items.map(item => item.id)).size).toBe(2);
  expect(first.items.map(item => item.id)).not.toEqual(
    expect.arrayContaining(second.items.map(item => item.id))
  );
  expect(first.total).toBeGreaterThanOrEqual(4);
  expect(second.total).toBe(first.total);

  for (const query of ["limit=0", "limit=101", "offset=-1", "limit=invalid"]) {
    const rejected = await page.request.get(`/v1/admin/users?${query}`);
    expect(rejected.status()).toBe(422);
  }
});

test("search and sorting are deterministic and survive UI reloads", async ({
  page
}) => {
  await signInAdministrator(page);
  await createUser(
    page,
    "query_alpha_user",
    "Query Alpha User",
    "Query-password1!"
  );
  await createUser(
    page,
    "query_zeta_user",
    "Query Zeta User",
    "Query-password2!"
  );

  const ascendingResponse = await page.request.get(
    "/v1/admin/users?search=query_&sort=login&direction=ascending&limit=100"
  );
  const descendingResponse = await page.request.get(
    "/v1/admin/users?search=query_&sort=login&direction=descending&limit=100"
  );
  expect(ascendingResponse.status()).toBe(200);
  expect(descendingResponse.status()).toBe(200);
  const ascending = (await ascendingResponse.json()).items.map(item => item.login);
  const descending = (await descendingResponse.json()).items.map(item => item.login);
  expect(ascending).toEqual(["query_alpha_user", "query_zeta_user"]);
  expect(descending).toEqual(["query_zeta_user", "query_alpha_user"]);

  await createRole(page, "query_sort_role");
  for (const resource of ["roles", "permissions"]) {
    const ascendingPage = await page.request.get(
      `/v1/admin/${resource}?sort=name&direction=ascending&limit=100`
    );
    const descendingPage = await page.request.get(
      `/v1/admin/${resource}?sort=name&direction=descending&limit=100`
    );
    expect(ascendingPage.status()).toBe(200);
    expect(descendingPage.status()).toBe(200);
    const ascendingBody = await ascendingPage.json();
    const descendingBody = await descendingPage.json();
    expect(ascendingBody.items.length).toBeGreaterThan(1);
    expect(ascendingBody.items).toHaveLength(ascendingBody.total);
    expect(descendingBody.items.map(item => item.id)).toEqual(
      ascendingBody.items.map(item => item.id).reverse()
    );
  }

  const unknownSort = await page.request.get(
    "/v1/admin/users?sort=unknown_column"
  );
  expect(unknownSort.status()).toBe(422);

  const query =
    "search=query_alpha_user&sort=login&direction=descending&limit=1&offset=0";
  await page.goto(`/admin/users?${query}`);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(page.locator("tbody tr")).toHaveCount(1);
  await expect(page.locator("tbody tr")).toContainText("query_alpha_user");
  await page.reload();
  await expect(page).toHaveURL(new RegExp(`/admin/users\\?${query}$`));
  await expect(page.locator("tbody tr")).toContainText("query_alpha_user");
});

test("data-table filters constrain rows and reject malformed filter contracts", async ({
  page
}) => {
  await signInAdministrator(page);
  const roleId = await createRole(page, "filter_contract_role");
  const permissionsResponse = await page.request.get(
    "/v1/admin/permissions?limit=100"
  );
  expect(permissionsResponse.status()).toBe(200);
  const permission = (await permissionsResponse.json()).items.find(
    item => item.name === "users:read"
  );
  expect(permission).toBeTruthy();
  const assigned = await page.request.put(
    `/v1/admin/roles/${roleId}/permissions`,
    {
      data: {
        expected_permission_ids: [],
        permission_ids: [permission.id]
      },
      headers: await adminHeaders(page.context())
    }
  );
  expect(assigned.status()).toBe(204);

  const filtered = await page.request.get(
    `/v1/admin/tables/role_permissions?filter_field=role_id&filter_operation=eq&filter_value=${roleId}&limit=100`
  );
  expect(filtered.status()).toBe(200);
  const table = await filtered.json();
  expect(table.table).toBe("role_permissions");
  expect(table.total).toBe(1);
  expect(table.items).toHaveLength(1);
  expect(table.items[0].values).toContain(String(roleId));
  expect(table.items[0].values).toContain(String(permission.id));

  for (const path of [
    "/v1/admin/tables/role_permissions?filter_field=role_id&filter_operation=eq",
    "/v1/admin/tables/role_permissions?filter_field=unknown&filter_operation=eq&filter_value=1"
  ]) {
    const rejected = await page.request.get(path);
    expect(rejected.status()).toBe(422);
  }
});

test("audit export records mutations without exposing submitted passwords", async ({
  page
}) => {
  await signInAdministrator(page);
  const password = "Audit-secret-password7!";
  const userId = await createUser(
    page,
    "audit_export_user",
    "Audit Export User",
    password
  );
  const auditResponse = await page.request.get("/v1/admin/audit_log?limit=100");
  expect(auditResponse.status()).toBe(200);
  const auditPage = await auditResponse.json();
  expect(auditPage.items).toEqual(expect.arrayContaining([
    expect.objectContaining({
      action: "create",
      resource: "user",
      resource_id: String(userId),
      succeeded: true
    })
  ]));
  expect(JSON.stringify(auditPage)).not.toContain(password);
  const exported = await page.request.get(
    "/v1/admin/audit_log/export?limit=100"
  );
  expect(exported.status()).toBe(200);
  const body = await exported.json();
  expect(body.csv).toContain(
    `\"create\",\"user\",\"${userId}\",\"true\"`
  );
  expect(body.csv).not.toContain(password);
  await page.goto("/admin/audit_log?limit=100&offset=0");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  const exportResponse = page.waitForResponse(response =>
    response.url().endsWith("/v1/admin/audit_log/export?limit=100&offset=0")
  );
  await page.getByRole("button", { name: "Prepare page CSV" }).click();
  const response = await exportResponse;
  expect(response.status()).toBe(200);
  const exportBody = await response.json();
  const downloadEvent = page.waitForEvent("download");
  await page.getByRole("link", { name: "Download page CSV" }).click();
  const download = await downloadEvent;
  expect(download.suggestedFilename()).toBe("audit_log.csv");
  const file = await download.path();
  expect(file).not.toBeNull();
  expect(await readFile(file, "utf8")).toBe(exportBody.csv);
  expect(exportBody.csv).not.toContain(password);
});

test("a read-only administrator sees only authorized navigation and mutations fail", async ({
  browser,
  page
}) => {
  await signInAdministrator(page);
  const roleId = await createRole(page, "production_reader");
  const permissionsResponse = await page.request.get(
    "/v1/admin/permissions?limit=100"
  );
  expect(permissionsResponse.status()).toBe(200);
  const permissions = (await permissionsResponse.json()).items;
  const usersRead = permissions.find(permission => permission.name === "users:read");
  const tablesRead = permissions.find(permission => permission.name === "tables:read");
  const auditRead = permissions.find(permission => permission.name === "audit_log:read");
  const settingsRead = permissions.find(
    permission => permission.name === "system_settings:read"
  );
  expect(usersRead).toBeTruthy();
  expect(tablesRead).toBeTruthy();
  expect(auditRead).toBeTruthy();
  expect(settingsRead).toBeTruthy();

  const rolePermissions = await page.request.put(
    `/v1/admin/roles/${roleId}/permissions`,
    {
      data: {
        expected_permission_ids: [],
        permission_ids: [auditRead.id, settingsRead.id, tablesRead.id, usersRead.id]
      },
      headers: await adminHeaders(page.context())
    }
  );
  expect(rolePermissions.status()).toBe(204);

  const userId = await createUser(
    page,
    "production_reader",
    "Production Reader",
    "Reader-password4!"
  );
  const userRoles = await page.request.put(`/v1/admin/users/${userId}/roles`, {
    data: {
      expected_role_ids: [],
      role_ids: [roleId]
    },
    headers: await adminHeaders(page.context())
  });
  expect(userRoles.status()).toBe(204);

  const context = await browser.newContext({ baseURL: adminOrigin });
  const reader = await context.newPage();
  await signIn(reader, "production_reader", "Reader-password4!");
  await expect(reader).toHaveURL(/\/admin\/profile$/);
  await reader.getByLabel("Current password").fill("Reader-password4!");
  await reader.getByLabel("New password").fill("Reader-password5!");
  const passwordChanged = reader.waitForResponse(
    response =>
      response.url().endsWith("/v1/admin/auth/password") &&
      response.status() === 204
  );
  await reader.getByRole("button", { name: "Change password" }).click();
  await passwordChanged;
  await expect(reader).toHaveURL(/\/admin\/profile$/);

  await reader.goto("/admin/users");
  await expect(reader.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(
    reader.locator('nav[aria-label="Admin sections"] a[href="/admin/users"]')
  ).toBeVisible();
  await expect(
    reader.locator('nav[aria-label="Admin sections"] a[href="/admin/roles"]')
  ).toHaveCount(0);
  await expect(
    reader.locator('nav[aria-label="Admin sections"] a[href="/admin/settings"]')
  ).toBeVisible();

  await reader.goto("/admin/audit_log");
  await expect(reader.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(reader.getByRole("button", { name: "Prepare page CSV" })).toHaveCount(0);
  expect((await reader.request.get("/v1/admin/audit_log/export")).status()).toBe(403);

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
  await expect(reader.getByRole("button", { name: "Save settings" })).toBeDisabled();
  const reset = reader.getByRole("button", {
    name: "Reset to template defaults"
  });
  await expect(reset).toBeDisabled();
  await reset.click({ force: true });
  await expect(reader.getByRole("dialog", { name: "Reset settings?" })).not.toBeVisible();

  const forbiddenPage = await reader.goto("/admin/roles");
  expect(forbiddenPage).not.toBeNull();
  expect(forbiddenPage.status()).toBe(403);
  const forbiddenApi = await reader.request.post("/v1/admin/users", {
    data: {
      display_name: "Forbidden User",
      login: "forbidden_user",
      password: "Forbidden-password6!"
    },
    headers: await adminHeaders(context)
  });
  expect(forbiddenApi.status()).toBe(403);
  await context.close();
});

test("a failed settings mutation preserves input and reports the server error", async ({
  page
}) => {
  await signInAdministrator(page);
  await page.goto("/admin/settings");
  const siteName = page.getByLabel("Site name");
  const originalSiteName = await siteName.inputValue();
  await siteName.fill("Unsaved Production Name");
  let intercepted = 0;
  let refreshes = 0;
  page.on("request", request => {
    if (request.url().endsWith("/v1/admin/auth/refresh")) refreshes += 1;
  });
  await page.route("**/v1/admin/system_settings", async route => {
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

  await page.getByRole("button", { name: "Save settings" }).click();
  await expect(page.getByRole("alert")).toBeVisible();
  await expect(siteName).toHaveValue("Unsaved Production Name");
  expect(intercepted).toBe(1);
  expect(refreshes).toBe(0);

  await page.unroute("**/v1/admin/system_settings");
  const persisted = await page.request.get("/v1/admin/system_settings");
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
    await page.getByText("Navigation", { exact: true }).click();
    await expect(page.getByRole("navigation", { name: "Admin sections" })).toBeVisible();
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


test("expired access cookies recover an audit download once", async ({ context, page }) => {
  await signInAdministrator(page);
  await page.goto("/admin/audit_log");
  const prepare = page.getByRole("button", { name: "Prepare page CSV" });
  await expect(prepare).toBeVisible();
  await context.clearCookies({ name: /admin_(access_token|csrf_token)/ });
  const responses = [];
  page.on("response", response => {
    if (response.url().includes("/v1/admin/audit_log/export") ||
        response.url().endsWith("/v1/admin/auth/refresh")) {
      responses.push([response.request().method(), response.status()]);
    }
  });
  await prepare.click();
  await expect(page.getByRole("link", { name: "Download page CSV" })).toBeVisible();
  expect(responses).toEqual([["GET", 401], ["POST", 200], ["GET", 200]]);
});

test("missing credentials stop audit recovery after one refresh", async ({ context, page }) => {
  await signInAdministrator(page);
  await page.goto("/admin/audit_log");
  const prepare = page.getByRole("button", { name: "Prepare page CSV" });
  await expect(prepare).toBeVisible();
  await context.clearCookies();
  const responses = [];
  page.on("response", response => {
    if (response.url().includes("/v1/admin/audit_log/export") ||
        response.url().endsWith("/v1/admin/auth/refresh")) {
      responses.push([response.request().method(), response.status()]);
    }
  });
  await prepare.click();
  await expect(page.getByRole("alert")).toBeVisible();
  await expect(prepare).toBeVisible();
  await expect(page.getByRole("link", { name: "Download page CSV" })).toHaveCount(0);
  expect(responses).toEqual([["GET", 401], ["POST", 401], ["GET", 401]]);
});

test("expired CSRF cookies recover a settings mutation without replaying it", async ({ context, page }) => {
  await signInAdministrator(page);
  await page.goto("/admin/settings");
  const siteName = page.getByLabel("Site name");
  const original = await siteName.inputValue();
  await siteName.fill("Recovered administration");
  await context.clearCookies({ name: /admin_(access_token|csrf_token)/ });
  const responses = [];
  page.on("response", response => {
    if (response.request().method() === "PATCH" ||
        response.url().endsWith("/v1/admin/auth/refresh")) {
      responses.push([response.request().method(), response.status()]);
    }
  });
  const saved = page.waitForResponse(response =>
    response.request().method() === "PATCH" && response.status() === 204);
  await page.getByRole("button", { name: "Save settings" }).click();
  await saved;
  await expect(siteName).toHaveValue("Recovered administration");
  expect(responses).toEqual([["POST", 200], ["PATCH", 204]]);
  await page.reload();
  await expect(siteName).toHaveValue("Recovered administration");
  await siteName.fill(original);
  const restored = page.waitForResponse(response =>
    response.request().method() === "PATCH" && response.status() === 204);
  await page.getByRole("button", { name: "Save settings" }).click();
  await restored;
});
