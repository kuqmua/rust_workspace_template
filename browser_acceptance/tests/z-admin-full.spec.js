import { expect, test } from "@playwright/test";

test.describe.configure({ mode: "serial" });
test.skip(
  process.env.BROWSER_ACCEPTANCE_FULL !== "1",
  "expanded acceptance runs on the scheduled matrix"
);

async function signIn(page) {
  await page.goto("/admin/sign_in");
  await page.getByLabel("Login").fill("administrator");
  await page.getByLabel("Password").fill("Changed-password2!");
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page).toHaveURL(/\/admin\/users$/);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
}

function csrfCookie(cookies) {
  return cookies.find(cookie => cookie.name === "admin_csrf_token")?.value;
}

test("direct API access is denied and refresh restores an access session", async ({
  context,
  page,
  request
}) => {
  const forbidden = await request.get("/api/v1/admin/users");
  expect(forbidden.status()).toBe(401);

  await signIn(page);
  const cookies = await context.cookies();
  const csrf = csrfCookie(cookies);
  expect(csrf).toBeTruthy();
  await context.clearCookies({ name: "admin_access_token" });
  const refreshed = await page.request.post("/api/v1/admin/auth/refresh", {
    data: {},
    headers: {
      Origin: "http://127.0.0.1:18080",
      "X-CSRF-Token": csrf
    }
  });
  expect(refreshed.status()).toBe(200);
  await page.goto("/admin/users");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
});

test("role permissions and runtime branding persist", async ({ page }) => {
  await signIn(page);
  await page.goto("/admin/roles");
  const csrf = csrfCookie(await page.context().cookies());
  expect(csrf).toBeTruthy();
  const created = await page.request.post("/api/v1/admin/roles", {
    data: { name: "browser_role" },
    headers: {
      Origin: "http://127.0.0.1:18080",
      "X-CSRF-Token": csrf
    }
  });
  expect(created.status()).toBe(201);
  await page.reload();
  let roleRow = page.locator("tbody tr").filter({
    has: page.locator('input[value="browser_role"]')
  });
  await expect(roleRow).toBeVisible();
  await roleRow.getByLabel("users:read").check();
  let mutation = page.waitForResponse(
    response =>
      response.request().method() === "PUT" &&
      response.url().endsWith("/permissions") &&
      response.status() === 204
  );
  await roleRow.getByRole("button", { name: "Save permissions" }).click();
  await mutation;
  roleRow = page.locator("tbody tr").filter({
    has: page.locator('input[value="browser_role"]')
  });
  await expect(roleRow.getByLabel("users:read")).toBeChecked();

  page.once("dialog", dialog => dialog.accept());
  mutation = page.waitForResponse(
    response =>
      response.request().method() === "DELETE" &&
      response.url().includes("/api/v1/admin/roles/") &&
      response.status() === 204
  );
  await roleRow.getByRole("button", { name: "Delete" }).click();
  await mutation;
  await expect(page.locator('input[value="browser_role"]')).toHaveCount(0);

  await page.goto("/admin/settings");
  await page.getByLabel("Site name").fill("Browser Acceptance Admin");
  await page.getByLabel("Tab title").fill("Acceptance Console");
  mutation = page.waitForResponse(
    response =>
      response.request().method() === "PATCH" &&
      response.url().endsWith("/api/v1/admin/system_settings") &&
      response.status() === 204
  );
  await page.getByRole("button", { name: "Save settings" }).click();
  await mutation;
  await expect(page).toHaveTitle("Acceptance Console");
  await page.reload();
  await expect(page.getByLabel("Site name")).toHaveValue("Browser Acceptance Admin");

  page.once("dialog", dialog => dialog.accept());
  mutation = page.waitForResponse(
    response =>
      response.request().method() === "PATCH" &&
      response.url().endsWith("/api/v1/admin/system_settings") &&
      response.status() === 204
  );
  await page
    .getByRole("button", { name: "Reset to template defaults" })
    .click();
  await mutation;
  await expect(page.getByLabel("Site name")).toHaveValue("Admin");
  await expect(page.getByLabel("Default route")).toHaveValue("/admin/users");
});

test("one-session and all-session revocation are enforced", async ({
  browser,
  context,
  page
}) => {
  await signIn(page);
  const otherContext = await browser.newContext({
    baseURL: "http://127.0.0.1:18080"
  });
  const otherPage = await otherContext.newPage();
  await signIn(otherPage);
  await page.goto("/admin/sessions");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  const sessionCount = await page.locator("tbody tr").count();
  expect(sessionCount).toBeGreaterThanOrEqual(2);
  const otherSession = page
    .locator("tbody tr")
    .filter({ hasText: "false" })
    .first();
  page.once("dialog", dialog => dialog.accept());
  const oneRevoked = page.waitForResponse(
    response =>
      response.request().method() === "DELETE" &&
      response.url().includes("/api/v1/admin/auth/sessions/") &&
      response.status() === 204
  );
  await otherSession.getByRole("button", { name: "Revoke session" }).click();
  await oneRevoked;
  await expect(page.locator("tbody tr")).toHaveCount(sessionCount - 1);
  await otherContext.close();

  const csrf = csrfCookie(await context.cookies());
  expect(csrf).toBeTruthy();
  const revoked = await page.request.delete("/api/v1/admin/auth/sessions", {
    data: {},
    headers: {
      Origin: "http://127.0.0.1:18080",
      "X-CSRF-Token": csrf
    }
  });
  expect(revoked.status()).toBe(204);
  await page.goto("/admin/users");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
});

test("failed sign-in reaches the concealed account lockout", async ({ page }) => {
  await page.goto("/admin/sign_in");
  for (let attempt = 0; attempt < 10; attempt += 1) {
    await page.getByLabel("Login").fill("missing_browser_user");
    await page.getByLabel("Password").fill("Wrong-password1!");
    const rejected = page.waitForResponse(
      response =>
        response.url().endsWith("/admin/actions/sign_in") &&
        response.status() === 401
    );
    await page.getByRole("button", { name: "Sign in" }).click();
    await rejected;
  }
  await page.getByLabel("Login").fill("missing_browser_user");
  await page.getByLabel("Password").fill("Wrong-password1!");
  const concealedLockout = page.waitForResponse(
    response =>
      response.url().endsWith("/admin/actions/sign_in") &&
      response.status() === 401
  );
  await page.getByRole("button", { name: "Sign in" }).click();
  await concealedLockout;
  await expect(page.getByRole("alert")).toBeVisible();

  await signIn(page);
  await page.goto(
    "/admin/login_attempts?search=missing_browser_user&limit=20"
  );
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(
    page.locator("tbody tr").filter({ hasText: "missing_browser_user" })
  ).toHaveCount(10);
});
