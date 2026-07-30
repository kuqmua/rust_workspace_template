import { expect, test } from "@playwright/test";

test.describe.configure({ mode: "serial" });
const fullMatrix = process.env.BROWSER_ACCEPTANCE_FULL === "1";

async function signIn(page) {
  await page.goto("/admin/sign_in");
  await page.getByLabel("Login").fill("administrator");
  await page.getByLabel("Password").fill("Changed-password2!");
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page).toHaveURL(/\/admin\/users$/);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
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

  await expect(page.locator(".password-policy")).toContainText(
    "uppercase, lowercase, digit, and special characters"
  );
  await page.getByLabel("Login").fill("administrator");
  await page.getByLabel("Password").fill("Initial-password1!");

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
  await page.goto("/admin/users");
  await expect(page).toHaveURL(/\/admin\/profile$/);

  await page.getByLabel("Current password").fill("Initial-password1!");
  await page.getByLabel("New password").fill("admin");
  await page.getByRole("button", { name: "Change password" }).click();
  await expect(page.getByRole("alert")).toHaveText(
    "Check both passwords and ensure the new password satisfies the policy."
  );

  await page.getByLabel("New password").fill("Changed-password2!");
  const passwordChanged = page.waitForResponse(
    response =>
      response.url().endsWith("/api/v1/admin/auth/password") &&
      response.status() === 204
  );
  await page.getByRole("button", { name: "Change password" }).click();
  await passwordChanged;
  await expect(page).toHaveURL(/\/admin\/profile$/);

  await page.goto("/admin/users");
  await expect(page).toHaveURL(/\/admin\/users$/);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
  await expect(
    page.getByRole("table").locator('input[value="administrator"]')
  ).toBeVisible();
});

test("administrator CSR supports user lifecycle, filtering, deep links, and viewports", async ({
  page
}) => {
  await signIn(page);

  await page.getByPlaceholder("Login").fill("browser_user");
  await page.getByPlaceholder("Display name").fill("Browser User");
  await page
    .getByPlaceholder("Password", { exact: true })
    .fill("Browser-password3!");
  const userCreated = page.waitForResponse(
    response =>
      response.request().method() === "POST" &&
      response.url().endsWith("/api/v1/admin/users") &&
      response.status() === 201
  );
  await page.getByRole("button", { name: "Create user" }).click();
  await userCreated;
  await expect(page.locator('input[value="browser_user"]')).toBeVisible();

  await page.getByLabel("Search").fill("browser_user");
  await page.getByRole("button", { name: "Apply" }).click();
  await expect(page).toHaveURL(/search=browser_user/);
  await expect(page.locator("tbody tr")).toHaveCount(1);

  const deepLink = page.url();
  await page.reload();
  await expect(page).toHaveURL(deepLink);
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();

  if (fullMatrix) {
    let userRow = page.locator("tbody tr");
    await userRow.locator('td[data-label="display_name"] input').fill(
      "Browser User Updated"
    );
    let mutation = page.waitForResponse(
      response =>
        response.request().method() === "PATCH" &&
        response.url().includes("/api/v1/admin/users/") &&
        response.status() === 204
    );
    await userRow.getByRole("button", { name: "Save", exact: true }).click();
    await mutation;
    await expect(page.locator('input[value="Browser User Updated"]')).toBeVisible();

    userRow = page.locator("tbody tr");
    await userRow.getByLabel("admin").check();
    mutation = page.waitForResponse(
      response =>
        response.request().method() === "PUT" &&
        response.url().endsWith("/roles") &&
        response.status() === 204
    );
    await userRow.getByRole("button", { name: "Save roles" }).click();
    await mutation;
    await expect(page.locator("tbody tr").getByLabel("admin")).toBeChecked();

    mutation = page.waitForResponse(
      response =>
        response.request().method() === "POST" &&
        response.url().endsWith("/ban") &&
        response.status() === 204
    );
    await page.locator("tbody tr").getByRole("button", { name: "Ban" }).click();
    await mutation;
    await expect(page.locator("tbody tr")).toContainText("true");

    mutation = page.waitForResponse(
      response =>
        response.request().method() === "POST" &&
        response.url().endsWith("/ban") &&
        response.status() === 204
    );
    await page.locator("tbody tr").getByRole("button", { name: "Unban" }).click();
    await mutation;

    page.once("dialog", dialog => dialog.accept());
    mutation = page.waitForResponse(
      response =>
        response.request().method() === "DELETE" &&
        response.url().includes("/api/v1/admin/users/") &&
        response.status() === 204
    );
    await page.locator("tbody tr").getByRole("button", { name: "Delete" }).click();
    await mutation;
    await expect(page.locator("tbody tr")).toHaveCount(0);
  }

  await page.setViewportSize({ width: 390, height: 844 });
  await expect(page.locator("nav[aria-label='Admin sections']")).toBeVisible();
  await expect(page.locator(".table-scroll")).toBeVisible();
  await page.setViewportSize({ width: 1440, height: 900 });
  await expect(page.getByRole("table")).toBeVisible();
});

test("keyboard navigation reaches every primary administrator route", async ({ page }) => {
  await signIn(page);

  for (const path of [
    "/admin/users",
    "/admin/roles",
    "/admin/permissions",
    "/admin/sessions",
    "/admin/profile",
    "/admin/settings",
    "/admin/version"
  ]) {
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
