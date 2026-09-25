import { readUsers, usersReadPage } from "./support/users.js";
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

test("mutations reject missing, invalid, and cross-origin CSRF credentials", async ({
  browser,
  page
}) => {
  await signInAdministrator(page);
  const original = await page.request.get("/system_settings/read");
  expect(original.status()).toBe(200);
  const originalSettings = await original.json();
  const update = {
    clear: [],
    site_name: "CSRF must not persist"
  };

  const missing = await page.request.patch("/system_settings/update", {
    data: update,
    headers: { Origin: adminOrigin }
  });
  expect([401, 403]).toContain(missing.status());

  const invalid = await page.request.patch("/system_settings/update", {
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
    "/system_settings/update",
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

  const crossOrigin = await page.request.patch("/system_settings/update", {
    data: update,
    headers: {
      ...(await adminHeaders(page.context())),
      Origin: "https://attacker.invalid"
    }
  });
  expect([401, 403]).toContain(crossOrigin.status());

  const unchanged = await page.request.get("/system_settings/read");
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

  const replay = await page.request.post("/auth/refresh", {
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
  await expect(page.getByRole("button", { name: "sign_in" })).toBeVisible();
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
  for (const path of ["/health/live/read", "/health/ready/read"]) {
    const response = await request.get(path);
    expect(response.status()).toBe(200);
  }

  const branding = await request.get("/branding/read");
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
    "/system_settings/read"
  );
  expect(originalResponse.status()).toBe(200);
  const original = await originalResponse.json();

  const invalid = await page.request.patch("/system_settings/update", {
    data: {
      clear: [],
      default_admin_route: "/outside-admin"
    },
    headers: await adminHeaders(page.context())
  });
  expect(invalid.status()).toBe(422);

  const headers = await adminHeaders(page.context());
  const oversizedStatus = await page.evaluate(async csrfToken => {
    const response = await fetch("/users/create", {
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

  const unchanged = await page.request.get("/system_settings/read");
  expect(unchanged.status()).toBe(200);
  expect(await unchanged.json()).toEqual(original);
  const absent = await readUsers(page.request, "search=oversized_user");
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

  const duplicate = await page.request.post("/users/create", {
    data: [{
      display_name: "Duplicate User",
      login: "production_user",
      password: "Production-password3!"
    }],
    headers: await adminHeaders(page.context())
  });
  expect([409, 422]).toContain(duplicate.status());

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
  await expect(row.locator('td[data-label="actions"] button[aria-label="read"]')).toHaveCount(1);

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

test("user creation accepts a batch and rolls back a conflicting batch", async ({ page }) => {
  await signInAdministrator(page);
  for (const data of [[], { display_name: "Single Object", login: "single_object_user", password: "Batch-password1!" }]) {
    const rejected = await page.request.post("/users/create", {
      data,
      headers: await adminHeaders(page.context())
    });
    expect(rejected.status()).toBe(422);
  }
  const created = await page.request.post("/users/create", {
    data: [
      { display_name: "Batch Alpha User", login: "batch_alpha_user", password: "Batch-password1!" },
      { display_name: "Batch Beta User", login: "batch_beta_user", password: "Batch-password2!" }
    ],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  const identifiers = await created.json();
  expect(identifiers).toHaveLength(2);
  expect(new Set(identifiers).size).toBe(2);
  const users = (await usersReadPage(await readUsers(page.request, "search=batch_"))).items;
  expect(users.find(user => user.id === identifiers[0])?.login).toBe("batch_alpha_user");
  expect(users.find(user => user.id === identifiers[1])?.login).toBe("batch_beta_user");

  const conflicted = await page.request.post("/users/create", {
    data: [
      { display_name: "Rolled Back User", login: "batch_rolled_back_user", password: "Batch-password3!" },
      { display_name: "Duplicate Batch User", login: "batch_alpha_user", password: "Batch-password4!" }
    ],
    headers: await adminHeaders(page.context())
  });
  expect(conflicted.status()).toBe(409);
  const absent = await readUsers(page.request, "search=batch_rolled_back_user");
  expect(absent.status()).toBe(200);
  expect((await absent.json()).items).toHaveLength(0);
  for (const userId of identifiers) await deleteUser(page, userId);
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
    (await userPage.request.get("/auth/me/read")).status()
  ).toBe(200);

  const reset = await page.request.patch(
    "/users/update",
    {
      data: { updates: [{ filter: { user_id: userId }, changes: { password: "Lifecycle-password3!" } }] },
      headers: await adminHeaders(page.context())
    }
  );
  expect(reset.status()).toBe(204);
  expect(
    (await userPage.request.get("/auth/me/read")).status()
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
    (await oldPage.request.get("/auth/me/read")).status()
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
  await deleteUser(page, userId);
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

  const originalUser = (await usersReadPage(await readUsers(page.request, "limit=100"))).items
    .find(user => user.id === userId);
  const administrator = (await usersReadPage(await readUsers(page.request, "limit=100"))).items
    .find(user => user.login === "administrator");
  const conflicting = await page.request.patch("/users/update", {
    data: { updates: [{ filter: { user_id: userId }, changes: { login: administrator.login, display_name: "Rejected Name", is_banned: true } }] },
    headers: await adminHeaders(page.context())
  });
  expect(conflicting.status()).toBe(409);
  expect((await usersReadPage(await readUsers(page.request, "limit=100"))).items
    .find(user => user.id === userId)).toEqual(originalUser);
  expect((await userPage.request.get("/auth/me/read")).status()).toBe(200);

  const banned = await page.request.patch("/users/update", {
    data: { updates: [{ filter: { user_id: userId }, changes: { is_banned: true } }] },
    headers: await adminHeaders(page.context())
  });
  expect(banned.status()).toBe(204);
  expect(
    (await userPage.request.get("/auth/me/read")).status()
  ).toBe(401);
  const renamed = await page.request.patch("/users/update", {
    data: { updates: [{ filter: { user_id: userId }, changes: { display_name: "Renamed Banned User" } }] },
    headers: await adminHeaders(page.context())
  });
  expect(renamed.status()).toBe(204);
  expect((await usersReadPage(await readUsers(page.request, "limit=100"))).items
    .find(user => user.id === userId)).toMatchObject({
      display_name: "Renamed Banned User", is_banned: true
    });
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

test("role lifecycle enforces uniqueness, stale-assignment conflicts, and deletion", async ({
  page
}) => {
  await signInAdministrator(page);
  const roleId = await createRole(page, "lifecycle_role");
  const duplicate = await page.request.post("/roles/create", {
    data: [{ name: "lifecycle_role" }],
    headers: await adminHeaders(page.context())
  });
  expect([409, 422]).toContain(duplicate.status());

  const renamed = await page.request.patch("/roles/update", {
    data: { updates: [{ filter: { role_id: roleId }, changes: { name: "renamed_lifecycle_role" } }] },
    headers: await adminHeaders(page.context())
  });
  expect(renamed.status()).toBe(204);

  const ruleId = (await readPermissionRuleIds(page)).get(2);
  expect(ruleId).toBeDefined();
  const assigned = await page.request.patch("/roles/update", {
    data: { updates: [{ filter: { role_id: roleId }, changes: { rules: { expected_rule_ids: [], rule_ids: [ruleId] } } }] },
    headers: await adminHeaders(page.context())
  });
  expect(assigned.status()).toBe(204);
  const stale = await page.request.patch("/roles/update", {
    data: { updates: [{ filter: { role_id: roleId }, changes: { rules: { expected_rule_ids: [], rule_ids: [] } } }] },
    headers: await adminHeaders(page.context())
  });
  expect(stale.status()).toBe(409);

  await page.goto("/admin/roles");
  const systemRole = page.locator("tbody tr").first();
  await expect(systemRole.locator('td[data-label="is_system"]')).toHaveText("true");
  const systemRoleId = Number(await systemRole.locator('td[data-label="id"]').textContent());
  const protectedSystemRole = await page.request.delete("/roles/delete", {
    data: { filter: { role_id: systemRoleId } },
    headers: await adminHeaders(page.context())
  });
  expect([409, 422]).toContain(protectedSystemRole.status());

  const deleted = await page.request.delete("/roles/delete", {
    data: { filter: { role_id: roleId } },
    headers: await adminHeaders(page.context())
  });
  expect(deleted.status()).toBe(204);
  await page.goto("/admin/roles/manage");
  await expect(page.locator('input[name="name"][value="renamed_lifecycle_role"]')).toHaveCount(0);
});

test("pagination has stable non-overlapping pages and rejects invalid bounds", async ({
  page
}) => {
  await signInAdministrator(page);
  const userIds = [];
  for (const index of [1, 2, 3]) {
    userIds.push(await createUser(
      page,
      `page_user_${index}`,
      `Page User ${index}`,
      `Pagination-password${index}!`
    ));
  }

  const firstResponse = await readUsers(page.request, "limit=2&offset=0");
  const secondResponse = await readUsers(page.request, "limit=2&offset=2");
  expect(firstResponse.status()).toBe(200);
  expect(secondResponse.status()).toBe(200);
  const first = await usersReadPage(firstResponse);
  const second = await usersReadPage(secondResponse);
  expect(first.items).toHaveLength(2);
  expect(second.items).toHaveLength(2);
  expect(new Set(first.items.map(item => item.id)).size).toBe(2);
  expect(first.items.map(item => item.id)).not.toEqual(
    expect.arrayContaining(second.items.map(item => item.id))
  );
  expect(first.total).toBeGreaterThanOrEqual(4);
  expect(second.total).toBe(first.total);

  for (const query of ["limit=0", "limit=1e100", "offset=-1", "limit=invalid"]) {
    const rejected = await readUsers(page.request, query);
    expect(rejected.status()).toBe(400);
  }
  for (const userId of userIds) await deleteUser(page, userId);
});

test("search and sorting are deterministic and survive UI reloads", async ({
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

  const ascendingResponse = await readUsers(page.request, "search=query_&sort=login&direction=ascending&limit=100");
  const descendingResponse = await readUsers(page.request, "search=query_&sort=login&direction=descending&limit=100");
  expect(ascendingResponse.status()).toBe(200);
  expect(descendingResponse.status()).toBe(200);
  const ascending = (await usersReadPage(ascendingResponse)).items.map(item => item.login);
  const descending = (await usersReadPage(descendingResponse)).items.map(item => item.login);
  expect(ascending).toEqual(["query_alpha_user", "query_zeta_user"]);
  expect(descending).toEqual(["query_zeta_user", "query_alpha_user"]);

  const roleId = await createRole(page, "query_sort_role");
  for (const resource of ["roles", "rules"]) {
    const path = `/${resource}/read`;
    const example = page.waitForRequest(value =>
      new URL(value.url()).pathname === path && value.method() === "POST"
    );
    await page.goto(`/admin/${resource}?limit=100`);
    const payload = {
      ...(await example).postDataJSON(),
      pagination: { limit: 100, offset: 0 }
    };
    const sort = resource === "roles" ? "name" : "id";
    const ascendingPage = await page.request.post(path, {
      data: { ...payload, order_by: { column: { [sort]: null }, order: "ascending" } }
    });
    const descendingPage = await page.request.post(path, {
      data: { ...payload, order_by: { column: { [sort]: null }, order: "descending" } }
    });
    expect(ascendingPage.status()).toBe(200);
    expect(descendingPage.status()).toBe(200);
    const ascendingBody = await ascendingPage.json();
    const descendingBody = await descendingPage.json();
    expect(ascendingBody.items.length).toBeGreaterThan(1);
    expect(ascendingBody.items).toHaveLength(ascendingBody.total);
    if (resource === "roles") {
      expect(descendingBody.items.map(item => item.id)).toEqual(
        ascendingBody.items.map(item => item.id).reverse()
      );
    } else {
      const idIndex = ascendingBody.columns.findIndex(column => column.name === "id");
      expect(idIndex).toBeGreaterThanOrEqual(0);
      expect(descendingBody.items.map(item => item.values[idIndex])).toEqual(
        ascendingBody.items.map(item => item.values[idIndex]).reverse()
      );
    }
  }

  const unknownSort = await readUsers(page.request, "sort=unknown_column");
  expect(unknownSort.status()).toBe(400);

  const query =
    "search=query_alpha_user&sort=login&direction=descending&limit=1&offset=0";
  await page.goto(`/admin/users?${query}`);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(page.locator("tbody tr")).toHaveCount(1);
  await expect(page.locator("tbody tr")).toContainText("query_alpha_user");
  await page.reload();
  await expect(page).toHaveURL(new RegExp(`/admin/users\\?${query}$`));
  await expect(page.locator("tbody tr")).toContainText("query_alpha_user");
  const deletedRole = await page.request.delete("/roles/delete", {
    data: { filter: { role_id: roleId } },
    headers: await adminHeaders(page.context())
  });
  expect(deletedRole.status()).toBe(204);
  await deleteUser(page, alphaUserId);
  await deleteUser(page, zetaUserId);
});

test("data-table filters constrain rows and reject malformed filter contracts", async ({
  page
}) => {
  await signInAdministrator(page);
  const roleId = await createRole(page, "filter_contract_role");
  const ruleId = (await readPermissionRuleIds(page)).get(2);
  expect(ruleId).toBeDefined();
  const assigned = await page.request.patch("/roles/update", {
    data: { updates: [{ filter: { role_id: roleId }, changes: { rules: { expected_rule_ids: [], rule_ids: [ruleId] } } }] },
    headers: await adminHeaders(page.context())
  });
  expect(assigned.status()).toBe(204);

  const roleRulesPath = "/role_rules/read";
  const example = page.waitForRequest(value =>
    new URL(value.url()).pathname === roleRulesPath && value.method() === "POST"
  );
  await page.goto("/admin/role_rules");
  const payload = {
    ...(await example).postDataJSON(),
    pagination: { limit: 100, offset: 0 },
    where_many: {
      role_id: {
        operator: "And",
        values: [{ Eq: { operator: "And", values: roleId } }]
      }
    }
  };
  const filtered = await page.request.post(roleRulesPath, { data: payload });
  expect(filtered.status()).toBe(200);
  const table = await filtered.json();
  expect(table.table).toBe("role_rules");
  expect(table.total).toBe(1);
  expect(table.items).toHaveLength(1);
  expect(table.items[0].values).toContain(String(roleId));
  expect(table.items[0].values).toContain(String(ruleId));

  for (const whereMany of [
    { role_id: { operator: "And", values: [{ Eq: { operator: "And" } }] } },
    { unknown: { operator: "And", values: [{ Eq: { operator: "And", values: 1 } }] } }
  ]) {
    const rejected = await page.request.post(roleRulesPath, {
      data: { ...payload, where_many: whereMany }
    });
    expect(rejected.status()).toBe(400);
  }
  const deleted = await page.request.delete("/roles/delete", {
    data: { filter: { role_id: roleId } },
    headers: await adminHeaders(page.context())
  });
  expect(deleted.status()).toBe(204);
});

test("audit records mutations without exposing submitted passwords", async ({
  page
}) => {
  await signInAdministrator(page);
  const password = "Audit-secret-password7!";
  const userId = await createUser(
    page,
    "audit_record_user",
    "Audit Export User",
    password
  );
  const auditResponse = await page.request.post("/audit_log/read", {
    data: {
      search: null,
      select: ["id", "user_id", "user_login", "action", "resource", "resource_id", "request_id", "succeeded", "created_at"].map(field => ({ [field]: null })),
      pagination: { limit: 100, offset: 0 },
      order_by: { column: { created_at: null }, order: "descending" },
      where_many: null
    }
  });
  expect(auditResponse.status()).toBe(200);
  const auditPage = await auditResponse.json();
  expect(auditPage.items.some(item =>
    item.values.includes("create") &&
    item.values.includes("user") &&
    item.values.includes(String(userId)) &&
    item.values.includes("true")
  )).toBe(true);
  expect(JSON.stringify(auditPage)).not.toContain(password);
  await deleteUser(page, userId);
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
