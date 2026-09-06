import { expect, test } from "@playwright/test";
import {
  adminHeaders,
  adminOrigin,
  cookieValue,
  signInAdministrator
} from "./support/admin.js";

test.describe.configure({ mode: "serial" });
test.skip(
  process.env.BROWSER_ACCEPTANCE_FULL !== "1",
  "expanded acceptance runs on the scheduled matrix"
);

test("direct API access is denied and refresh restores an access session", async ({
  context,
  page,
  request
}) => {
  const forbidden = await request.get("/v1/admin/users");
  expect(forbidden.status()).toBe(401);

  await signInAdministrator(page);
  const cookies = await context.cookies();
  const csrf = cookieValue(cookies, "admin_csrf_token");
  expect(csrf).toBeTruthy();
  await context.clearCookies({ name: "admin_access_token" });
  const refreshed = await page.request.post("/v1/admin/auth/refresh", {
    data: {},
    headers: {
      Origin: adminOrigin,
      "X-CSRF-Token": csrf
    }
  });
  expect(refreshed.status()).toBe(200);
  await page.goto("/admin/users");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
});

test("read-only role rows and runtime branding persist", async ({ page }) => {
  await signInAdministrator(page);
  await page.goto("/admin/roles");
  const created = await page.request.post("/v1/admin/roles", {
    data: { name: "browser_role" },
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  await page.reload();
  const roleRow = page.locator("tbody tr").filter({
    hasText: "browser_role"
  });
  await expect(roleRow).toBeVisible();
  await expect(roleRow.locator("button, input, select")).toHaveCount(0);

  await page.goto("/admin/settings");
  await page.getByLabel("Site name").fill("Browser Acceptance Admin");
  await page.getByLabel("Tab title").fill("Acceptance Console");
  let mutation = page.waitForResponse(
    response =>
      response.request().method() === "PATCH" &&
      response.url().endsWith("/v1/admin/system_settings") &&
      response.status() === 204
  );
  await page.getByRole("button", { name: "save_settings" }).click();
  await mutation;
  await expect(page).toHaveTitle("Acceptance Console");
  await page.reload();
  await expect(page.getByLabel("Site name")).toHaveValue("Browser Acceptance Admin");

  mutation = page.waitForResponse(
    response =>
      response.request().method() === "PATCH" &&
      response.url().endsWith("/v1/admin/system_settings") &&
      response.status() === 204
  );
  await page
    .getByRole("button", { name: "reset_to_template_defaults" })
    .click();
  await page
    .getByRole("dialog", { name: "Reset settings?" })
    .getByRole("button", { name: "reset_settings" })
    .click();
  await mutation;
  await expect(page.getByLabel("Site name")).toHaveValue("Admin");
  await expect(page.getByLabel("Default route")).toHaveValue("/admin/users");
});

test("one-session and all-session revocation are enforced", async ({
  browser,
  page
}) => {
  await signInAdministrator(page);
  const otherContext = await browser.newContext({
    baseURL: adminOrigin
  });
  const otherPage = await otherContext.newPage();
  await signInAdministrator(otherPage);
  await page.goto("/admin/sessions");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  const sessionCount = await page.locator("tbody tr").count();
  expect(sessionCount).toBeGreaterThanOrEqual(2);
  const otherSession = page
    .locator("tbody tr")
    .filter({ hasText: "false" })
    .first();
  const revokedSessionId = await otherSession.locator("td").first().innerText();
  const oneRevoked = page.waitForResponse(
    response =>
      response.request().method() === "DELETE" &&
      response.url().includes("/v1/admin/auth/sessions/") &&
      response.status() === 204
  );
  await otherSession.getByRole("button", { name: "revoke_session" }).click();
  await otherSession
    .getByRole("dialog", { name: "Revoke session?" })
    .getByRole("button", { name: "revoke", exact: true })
    .click();
  await oneRevoked;
  await expect(
    page.locator("tbody tr").filter({ hasText: revokedSessionId })
  ).toHaveCount(0);
  await otherContext.close();

  await page.getByRole("button", { name: "revoke_all_sessions", exact: true }).click();
  const confirmation = page.getByRole("dialog", { name: "Revoke all sessions", exact: true });
  await confirmation.getByRole("button", { name: "cancel", exact: true }).click();
  await expect(confirmation).not.toBeVisible();
  await expect(page).toHaveURL(/\/admin\/sessions$/);
  const revoked = page.waitForResponse(response =>
    response.request().method() === "DELETE" &&
    response.url().endsWith("/v1/admin/auth/sessions") &&
    response.status() === 204
  );
  await page.getByRole("button", { name: "revoke_all_sessions", exact: true }).click();
  await confirmation.getByRole("button", { name: "revoke_all_sessions", exact: true }).click();
  await revoked;
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
    await page.getByRole("button", { name: "sign_in" }).click();
    await rejected;
  }
  await page.getByLabel("Login").fill("missing_browser_user");
  await page.getByLabel("Password").fill("Wrong-password1!");
  const concealedLockout = page.waitForResponse(
    response =>
      response.url().endsWith("/admin/actions/sign_in") &&
      response.status() === 401
  );
  await page.getByRole("button", { name: "sign_in" }).click();
  await concealedLockout;
  await expect(page.getByRole("alert")).toBeVisible();

  await signInAdministrator(page);
  await page.goto(
    "/admin/login_attempts?search=missing_browser_user&limit=20"
  );
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(
    page.locator("tbody tr").filter({ hasText: "missing_browser_user" })
  ).toHaveCount(10);
});
