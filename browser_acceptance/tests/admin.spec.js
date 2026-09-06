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

test("initial administrator sign-in forces a password change before administrator access", async ({
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
  await page.getByRole("button", { name: "sign_in" }).click();
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
  await expect(page.getByRole("alert")).toContainText(
    "Change your initial password to unlock administrator navigation."
  );
  await expect(page.locator('header a[href="/admin/users"]')).toHaveCount(0);
  await page.goto("/admin/users");
  await expect(page).toHaveURL(/\/admin\/profile$/);

  await page.getByLabel("Current password").fill(initialAdminPassword);
  await page.getByLabel("New password").fill("admin");
  await page.getByRole("button", { name: "change_password" }).click();
  await expect(page.getByRole("alert").filter({ hasText: "Check both passwords" })).toHaveText(
    "Check both passwords and ensure the new password satisfies the policy."
  );

  await changePassword(page, initialAdminPassword, changedAdminPassword);
  await expect(page).toHaveURL(/\/admin\/profile$/);
  await expect(page.getByText("Change your initial password to unlock administrator navigation.", { exact: true })).toHaveCount(0);
  await page.locator('header a[href="/admin/users"]').click();
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
  await page.getByText("navigation", { exact: true }).click();
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

test("shared administrator layout stays fixed while pages change", async ({ page }) => {
  await signInAdministrator(page);
  await page.setViewportSize({ width: 1440, height: 900 });

  const geometry = [];
  for (const path of [
    "/admin/users",
    "/admin/profile",
    "/admin/settings",
    "/admin/users/create",
    "/admin/roles/manage"
  ]) {
    await page.goto(path);
    await expect(page.locator(".loading-state")).toHaveCount(0);
    await expect(page.locator(".page-frame")).toBeVisible();
    geometry.push({
      content: await page
        .locator(".page-frame > :visible:not(.flash-success)")
        .first()
        .boundingBox(),
      main: await page.locator("main.main-content").boundingBox(),
      navigation: await page.locator("header.topbar").boundingBox()
    });
  }

  const expected = geometry[0];
  expect(expected.navigation.x).toBe(0);
  expect(expected.navigation.y).toBe(0);
  expect(expected.navigation.width).toBe(1440);
  expect(expected.main.x).toBe(0);
  expect(expected.main.y).toBeGreaterThanOrEqual(
    expected.navigation.y + expected.navigation.height
  );
  for (const current of geometry.slice(1)) {
    expect(current.main).toEqual(expected.main);
    expect(current.navigation).toEqual(expected.navigation);
    expect(current.content.x).toBe(expected.content.x);
    expect(current.content.width).toBe(expected.content.width);
  }
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

test("administrator can create, update, and delete users and roles from dedicated pages", async ({
  page
}) => {
  await signInAdministrator(page);

  await page.goto("/admin/users");
  await page.getByRole("link", { name: "create_user" }).click();
  await expect(page).toHaveURL(/\/admin\/users\/create$/);
  await page.getByLabel("Login").fill("crud_user");
  await page.getByLabel("Display name").fill("CRUD User");
  await page.getByLabel("Initial password").fill("CrudUser1!Secure");
  await page.getByRole("button", { name: "create_user" }).click();
  await expect(page).toHaveURL(/\/admin\/users#saved$/);
  await expect(page.getByRole("cell", { name: "crud_user" })).toBeVisible();

  await page.getByRole("link", { name: "manage_users" }).click();
  await expect(page).toHaveURL(/\/admin\/users\/manage$/);
  const userCard = page.locator("article.crud-record").filter({
    has: page.locator('input[name="login"][value="crud_user"]')
  });
  await userCard.getByLabel("Display name").fill("Updated CRUD User");
  await userCard.getByRole("button", { name: "save_changes" }).click();
  await expect(page).toHaveURL(/\/admin\/users#saved$/);
  await expect(page.getByRole("cell", { name: "Updated CRUD User" })).toBeVisible();
  await page.getByRole("link", { name: "manage_users" }).click();
  const updatedUserCard = page.locator("article.crud-record").filter({
    has: page.locator('input[name="login"][value="crud_user"]')
  });
  await updatedUserCard.getByLabel("I understand this cannot be undone").check();
  await updatedUserCard.getByRole("button", { name: "delete_user" }).click();
  await expect(page).toHaveURL(/\/admin\/users#saved$/);
  await expect(page.getByRole("cell", { name: "crud_user" })).toHaveCount(0);

  await page.goto("/admin/roles");
  await page.getByRole("link", { name: "create_role" }).click();
  await expect(page).toHaveURL(/\/admin\/roles\/create$/);
  await page.getByLabel("Role name").fill("crud_role");
  await page.getByRole("button", { name: "create_role" }).click();
  await expect(page).toHaveURL(/\/admin\/roles#saved$/);
  await expect(page.getByRole("cell", { name: "crud_role" })).toBeVisible();

  await page.getByRole("link", { name: "manage_roles" }).click();
  const roleCard = page.locator("article.crud-record").filter({
    has: page.locator('input[name="name"][value="crud_role"]')
  });
  await roleCard.getByLabel("Role name").fill("updated_crud_role");
  await roleCard.getByRole("button", { name: "save_changes" }).click();
  await expect(page).toHaveURL(/\/admin\/roles#saved$/);
  await expect(page.getByRole("cell", { name: "updated_crud_role" })).toBeVisible();
  await page.getByRole("link", { name: "manage_roles" }).click();
  const updatedRoleCard = page.locator("article.crud-record").filter({
    has: page.locator('input[name="name"][value="updated_crud_role"]')
  });
  await updatedRoleCard.getByLabel("I understand this cannot be undone").check();
  await updatedRoleCard.getByRole("button", { name: "delete_role" }).click();
  await expect(page).toHaveURL(/\/admin\/roles#saved$/);
  await expect(page.getByRole("cell", { name: "updated_crud_role" })).toHaveCount(0);
});

test("Rust UI primitives expose their semantic component contracts", async ({ page }) => {
  await signInAdministrator(page);
  await page.goto("/admin/users");

  await expect(page.locator('[data-name="NavigationMenu"]')).toHaveCount(1);
  await expect(page.locator('[data-name="NavigationMenuLink"]').first()).toBeVisible();
  await expect(page.locator('[data-name="TableWrapper"]')).toHaveCount(1);
  await expect(page.locator('table[data-name="Table"]')).toHaveCount(1);
  await expect(page.locator('thead[data-name="TableHeader"]')).toHaveCount(1);
  await expect(page.locator('tbody[data-name="TableBody"]')).toHaveCount(1);
  await expect(page.locator('tr[data-name="TableRow"]').first()).toBeVisible();
  await expect(page.locator('th[data-name="TableHead"]').first()).toBeVisible();
  await expect(page.locator('td[data-name="TableCell"]').first()).toBeVisible();
  await expect(page.locator('[data-name="Pagination"]')).toHaveCount(1);

  await page.goto("/admin/profile");
  await expect(page.locator('[data-name="Card"]')).toHaveCount(2);
  await expect(page.locator('[data-name="CardContent"]')).toHaveCount(2);
  await expect(page.locator('[data-name="Field"]').first()).toBeVisible();
  await expect(page.locator('[data-name="Label"]').first()).toBeVisible();
  await expect(page.locator('input[data-name="Input"]').first()).toBeVisible();
  await expect(page.locator('button.ui-button').first()).toBeVisible();

  await page.goto("/admin/settings");
  const textarea = page.locator('textarea[data-name="Textarea"]').first();
  await expect(textarea).toBeVisible();
  const textareaValue = await textarea.inputValue();
  await textarea.fill(`${textareaValue} UI test`);
  await expect(textarea).toHaveValue(`${textareaValue} UI test`);
  await page
    .getByRole("button", { name: "reset_to_template_defaults" })
    .click();
  const dialog = page.getByRole("dialog", { name: "Reset settings?" });
  await expect(dialog).toBeVisible();
  await expect(dialog.locator('[data-name="AlertDialogBody"]')).toHaveCount(1);
  await expect(dialog.locator('[data-name="AlertDialogHeader"]')).toHaveCount(1);
  await expect(dialog.locator('[data-name="AlertDialogTitle"]')).toHaveText(
    "Reset settings?"
  );
  await expect(
    dialog.locator('[data-name="AlertDialogDescription"]')
  ).toHaveCount(1);
  await expect(dialog.locator('[data-name="AlertDialogFooter"]')).toHaveCount(1);
  await dialog.getByRole("button", { name: "cancel" }).click();
  await expect(dialog).not.toBeVisible();
});

test("data-table filter places a full-width Close control below Apply", async ({
  page
}) => {
  await signInAdministrator(page);
  await page.goto("/admin/role_permissions");

  const filter = page.locator('th[data-field="role_id"] .table-column-filter');
  await filter.getByRole("button", { name: "Filter Role Id" }).click();

  const dialog = filter.getByRole("dialog");
  const close = dialog.getByRole("button", { name: "close" });
  await expect(dialog).toBeVisible();
  await expect(filter).toHaveAttribute("data-name", "Popover");
  await expect(dialog).toHaveAttribute("data-name", "PopoverContent");
  await expect(dialog.locator('[data-name="RadioButtonGroup"]')).toHaveCount(1);
  await expect(dialog.getByRole("radio", { name: "Eq", exact: true })).toBeChecked();
  await expect(close).toContainText("close");

  const controls = await filter.evaluate(element => {
    const buttons = element.querySelectorAll("button");
    const applyRect = buttons.item(1).getBoundingClientRect();
    const closeRect = buttons.item(2).getBoundingClientRect();
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

test("shared administrator shell remains visually stable across page navigation", async ({
  page
}) => {
  await page.setViewportSize({ width: 1440, height: 900 });
  await signInAdministrator(page);
  await page.goto("/admin/users");
  await expect(page.locator('[data-renderer="csr"]')).toBeVisible();

  const header = page.locator("header.topbar");
  const main = page.locator("main.main-content");
  const initialHeaderBox = await header.boundingBox();
  const initialMainBox = await main.boundingBox();
  const initialStructure = await header.evaluate(element =>
    Array.from(element.querySelectorAll("*")).map(child => child.tagName)
  );
  for (const path of ["/admin/roles", "/admin/settings", "/admin/users"]) {
    await page.locator(`header a[href="${path}"]`).click();
    await expect(page).toHaveURL(new RegExp(`${path.replaceAll("/", "\\/")}$`));
    await expect(page.locator(`header a[href="${path}"]`)).toHaveAttribute(
      "aria-current",
      "page"
    );
    expect(await header.boundingBox()).toEqual(initialHeaderBox);
    expect(await main.boundingBox()).toEqual(initialMainBox);
    expect(
      await header.evaluate(element =>
        Array.from(element.querySelectorAll("*")).map(child => child.tagName)
      )
    ).toEqual(initialStructure);
  }
});


test("header links leave profile and render the selected page on desktop and mobile", async ({ page }) => {
  await signInAdministrator(page);
  for (const width of [1440, 390]) {
    await page.setViewportSize({ width, height: 900 });
    await page.goto("/admin/profile");
    for (const name of ["users", "roles", "permissions", "profile"]) {
      if (width === 390) await page.getByText("navigation", { exact: true }).click();
      await page.locator(`header a[href="/admin/${name}"]`).click();
      await expect(page).toHaveURL(new RegExp(`/admin/${name}$`));
      await expect(page.locator(`header a[href="/admin/${name}"][aria-current="page"]`)).toHaveCount(1);
      await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
      if (name === "profile") {
        await expect(page.getByLabel("Current password")).toBeVisible();
      } else {
        await expect(page.getByRole("table")).toBeVisible();
        await expect(page.getByLabel("Current password")).toHaveCount(0);
      }
    }
  }
});
