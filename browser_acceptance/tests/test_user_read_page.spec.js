import { expect, test } from "@playwright/test";
import { adminHeaders, signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_user_read_page_shows_only_the_selected_user_and_survives_reload", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const created = await page.request.post("/users/create", {
    data: [
      { display_name: "Read selected user", login: "user_read_selected", password: "Fixture-password7!" },
      { display_name: "Read other user", login: "user_read_other", password: "Fixture-password8!" }
    ],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  const [id] = await created.json();
  await page.goto("/admin/users");
  const row = page.locator("tbody tr").filter({ has: page.locator('td[data-label="id"]').filter({ hasText: new RegExp(`^${id}$`) }) });
  await expect(page.locator('td[data-label="actions"] dialog[aria-label="read"]')).toHaveCount(0);
  await expect(row.getByRole("button", { name: "read", exact: true })).toHaveCount(0);
  const link = row.getByRole("link", { name: "read", exact: true });
  await expect(link).toHaveAttribute("href", `/admin/users/${id}/read`);
  await link.click();
  await expect(page).toHaveURL(`/admin/users/${id}/read`);
  const details = page.locator('section[data-page="user-read"]');
  await expect(details).toContainText("user_read_selected");
  await expect(details).toContainText("Read selected user");
  await expect(details).not.toContainText("user_read_other");
  await expect(details.locator("input, select, textarea, form")).toHaveCount(0);
  await expect(details).not.toContainText("password_hash");
  await page.reload();
  await expect(details).toContainText("user_read_selected");
  await page.goto(`/admin/users/${id}`);
  await expect(details).toContainText("user_read_selected");
});

test("test_user_read_page_reports_a_missing_user", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await page.goto("/admin/users/9223372036854775807/read");
  await expect(page.locator('section[data-page="user-read"]')).toContainText("resource not found");
});

test("test_user_read_actions_have_distinct_row_paths_and_no_modal_commands", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const created = await page.request.post("/users/create", {
    data: [
      { display_name: "First read link", login: "user_read_link_first", password: "Fixture-password7!" },
      { display_name: "Second read link", login: "user_read_link_second", password: "Fixture-password8!" }
    ],
    headers: await adminHeaders(page.context())
  });
  expect(created.status()).toBe(201);
  const ids = await created.json();
  await page.goto("/admin/users");
  for (const id of ids) {
    const row = page.locator("tbody tr").filter({ has: page.locator('td[data-label="id"]').filter({ hasText: new RegExp(`^${id}$`) }) });
    const read = row.getByRole("link", { name: "read", exact: true });
    await expect(read).toHaveCount(1);
    await expect(read).toHaveAttribute("href", `/admin/users/${id}/read`);
    await expect(read).toHaveAttribute("title", "read");
    await expect(read.locator('svg[aria-hidden="true"]')).toHaveCount(1);
    expect(await read.getAttribute("commandfor")).toBeNull();
    expect(await read.getAttribute("command")).toBeNull();
    await expect(row.getByRole("button", { name: "read", exact: true })).toHaveCount(0);
    await expect(row.locator('dialog[aria-label="read"]')).toHaveCount(0);
    await expect(row.getByRole("link", { name: "update", exact: true })).toHaveAttribute("href", `/admin/users/${id}/update`);
  }
});

test("test_user_read_link_opens_in_a_new_tab_without_changing_the_table", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const row = page.locator("tbody tr").first();
  const id = (await row.locator('td[data-label="id"]').textContent()).trim();
  const login = (await row.locator('td[data-label="login"]').textContent()).trim();
  const popupPromise = page.context().waitForEvent("page");
  await row.getByRole("link", { name: "read", exact: true }).click({ modifiers: ["ControlOrMeta"] });
  const popup = await popupPromise;
  try {
    await popup.bringToFront();
    await expect(popup).toHaveURL(`/admin/users/${id}/read`);
    await expect(popup.locator('section[data-page="user-read"]')).toContainText(login);
    await expect(page).toHaveURL(/\/admin\/users$/);
    await expect(row.locator('td[data-label="login"]')).toHaveText(login);
    await expect(page.locator('dialog[aria-label="read"][open]')).toHaveCount(0);
  } finally {
    await popup.close();
  }
});

test("test_user_read_link_supports_keyboard_navigation_and_browser_back", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const row = page.locator("tbody tr").first();
  const id = (await row.locator('td[data-label="id"]').textContent()).trim();
  const login = (await row.locator('td[data-label="login"]').textContent()).trim();
  const read = row.getByRole("link", { name: "read", exact: true });
  await read.focus();
  await expect(read).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(page).toHaveURL(`/admin/users/${id}/read`);
  await expect(page.locator('section[data-page="user-read"]')).toContainText(login);
  await page.goBack();
  await expect(page).toHaveURL(/\/admin\/users$/);
  await expect(read).toHaveAttribute("href", `/admin/users/${id}/read`);
  await expect(page.locator('dialog[aria-label="read"]')).toHaveCount(0);
});
