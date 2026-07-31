import { expect } from "@playwright/test";

export const adminOrigin = "http://127.0.0.1:18080";
export const changedAdminPassword = "Changed-password2!";
export const initialAdminPassword = "Initial-password1!";

export async function signIn(
  page,
  login = "administrator",
  password = changedAdminPassword
) {
  await page.goto("/admin/sign_in");
  await page.getByLabel("Login").fill(login);
  await page.getByLabel("Password").fill(password);
  await page.getByRole("button", { name: "Sign in" }).click();
}

export async function signInAdministrator(page) {
  await signIn(page);
  await expect(page).toHaveURL(/\/admin\/users$/);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
}

export async function changePassword(page, currentPassword, newPassword) {
  await page.getByLabel("Current password").fill(currentPassword);
  await page.getByLabel("New password").fill(newPassword);
  const passwordChanged = page.waitForResponse(
    response =>
      response.url().endsWith("/v1/admin/auth/password") &&
      response.status() === 204
  );
  await page.getByRole("button", { name: "Change password" }).click();
  await passwordChanged;
}

export async function bootstrapAdministrator(page) {
  await signIn(page, "administrator", initialAdminPassword);
  await expect(page).toHaveURL(/\/admin\/profile$/);
  await changePassword(page, initialAdminPassword, changedAdminPassword);
}

export async function signInBootstrappedAdministrator(page) {
  await signIn(page);
  if (page.url().endsWith("/admin/actions/sign_in")) {
    await bootstrapAdministrator(page);
    await page.goto("/admin/users");
  }
  await expect(page).toHaveURL(/\/admin\/users$/);
}

export function cookieValue(cookies, name) {
  return cookies.find(cookie => cookie.name === name)?.value;
}

export async function adminHeaders(context) {
  const csrf = cookieValue(await context.cookies(), "admin_csrf_token");
  expect(csrf).toBeTruthy();
  return {
    Origin: adminOrigin,
    "X-CSRF-Token": csrf
  };
}

export async function signOutIfAuthenticated(page) {
  await page.goto("/admin/users");
  const signOut = page.locator("header form button");
  if (await signOut.isVisible()) {
    await signOut.click();
    await expect(page).toHaveURL(/\/admin\/sign_in$/);
  }
}

export function observeBrowserErrors(page, includeFailedRequests = false) {
  const consoleErrors = [];
  const failedRequests = [];
  const pageErrors = [];
  page.on("console", message => {
    if (message.type() === "error") {
      consoleErrors.push(message.text());
    }
  });
  page.on("pageerror", error => pageErrors.push(error.message));
  if (includeFailedRequests) {
    page.on("requestfailed", request => {
      failedRequests.push(`${request.method()} ${request.url()}`);
    });
  }
  return { consoleErrors, failedRequests, pageErrors };
}
