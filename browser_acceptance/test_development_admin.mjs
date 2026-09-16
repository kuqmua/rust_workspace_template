import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";
import { chromium } from "playwright";

test("test_development_admin_documented_password_signs_in", {
  skip: process.env.RUN_DEVELOPMENT_ADMIN_TEST !== "1"
    ? "Requires the provisioned local development database and server on port 8080"
    : false,
  timeout: 30_000
}, async () => {
  const documentation = await readFile(new URL("../ADMIN_LOGIN_AND_PASSWORD.md", import.meta.url), "utf8");
  const login = documentation.match(/^- Login: `([^`]+)`$/m)?.[1];
  const password = documentation.match(/^- Password: `([^`]+)`$/m)?.[1];
  assert.ok(login, "The development administrator login must be documented");
  assert.ok(password, "The development administrator password must be documented");
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    await page.goto("http://127.0.0.1:8080/admin/sign_in");
    await page.getByLabel("login", { exact: true }).fill(login);
    await page.getByLabel("password", { exact: true }).fill(password);
    await page.getByRole("button", { name: "sign_in", exact: true }).click();
    await page.waitForURL("http://127.0.0.1:8080/admin/users");
    await page.getByRole("link", { name: "create", exact: true }).waitFor();
    await page.getByRole("button", { name: login, exact: true }).first().waitFor();
    const identifier = await page.locator("tbody tr td:first-child").first().innerText();
    const displayName = await page.locator("tbody tr td:nth-child(3)").first().innerText();
    const banned = await page.locator("tbody tr td:nth-child(4)").first().innerText();
    await page.getByRole("button", { name: "filter_id", exact: true }).click();
    await page.locator("#table-filter-id input[value=eq]").check();
    await page.locator("#table-filter-id input[name=filter_value]:not([disabled])").fill(identifier);
    await page.locator("#table-filter-id form").getByRole("button", { name: "apply", exact: true }).click();
    await page.waitForURL(url => url.searchParams.get("filter_field") === "id"
      && url.searchParams.get("filter_operation") === "eq"
      && url.searchParams.has("filter_value"));
    const filteredIdentifiers = await page.locator("tbody tr td:first-child").allInnerTexts();
    assert.deepEqual(filteredIdentifiers, [identifier]);
    await page.goto("http://127.0.0.1:8080/admin/users");
    await page.getByRole("button", { name: "filter_login", exact: true }).click();
    await page.locator("#table-filter-login input[value=in]").check();
    await page.locator("#table-filter-login input[name=filter_value]:not([disabled])").fill(login);
    await page.locator("#table-filter-login form").getByRole("button", { name: "apply", exact: true }).click();
    await page.waitForURL(url => url.pathname === "/admin/users"
      && url.searchParams.get("filter_field") === "login"
      && url.searchParams.get("filter_operation") === "in"
      && url.searchParams.has("filter_value"));
    const filteredLogins = await page.locator("tbody tr td:nth-child(2)").allInnerTexts();
    assert.deepEqual(filteredLogins, [login]);
    await page.goto("http://127.0.0.1:8080/admin/users");
    await page.getByRole("button", { name: "filter_display_name", exact: true }).click();
    await page.locator("#table-filter-display_name input[value=eq]").check();
    await page.locator("#table-filter-display_name input[name=filter_value]:not([disabled])").fill(displayName);
    await page.locator("#table-filter-display_name form").getByRole("button", { name: "apply", exact: true }).click();
    await page.waitForURL(url => url.pathname === "/admin/users"
      && url.searchParams.get("filter_field") === "display_name"
      && url.searchParams.get("filter_operation") === "eq"
      && url.searchParams.has("filter_value"));
    const filteredDisplayNames = await page.locator("tbody tr td:nth-child(3)").allInnerTexts();
    assert.deepEqual(filteredDisplayNames, [displayName]);
    await page.goto("http://127.0.0.1:8080/admin/users");
    await page.getByRole("button", { name: "filter_is_banned", exact: true }).click();
    await page.locator("#table-filter-is_banned input[value=eq]").check();
    await page.locator("#table-filter-is_banned input[name=filter_value]:not([disabled])").fill(banned);
    await page.locator("#table-filter-is_banned form").getByRole("button", { name: "apply", exact: true }).click();
    await page.waitForURL(url => url.pathname === "/admin/users"
      && url.searchParams.get("filter_field") === "is_banned"
      && url.searchParams.get("filter_operation") === "eq"
      && url.searchParams.get("filter_value") === banned);
    const filteredBannedValues = await page.locator("tbody tr td:nth-child(4)").allInnerTexts();
    assert.ok(filteredBannedValues.length > 0);
    assert.ok(filteredBannedValues.every(value => value === banned));
    await page.goto("http://127.0.0.1:8080/admin/roles");
    const roleIdentifier = await page.locator("tbody tr td:first-child").first().innerText();
    const roleName = await page.locator("tbody tr td:nth-child(2)").first().innerText();
    const roleSystem = await page.locator("tbody tr td:nth-child(3)").first().innerText();
    await page.getByRole("button", { name: "filter_id", exact: true }).click();
    await page.locator("#table-filter-id input[value=eq]").check();
    await page.locator("#table-filter-id input[name=filter_value]:not([disabled])").fill(roleIdentifier);
    await page.locator("#table-filter-id form").getByRole("button", { name: "apply", exact: true }).click();
    await page.waitForURL(url => url.pathname === "/admin/roles"
      && url.searchParams.get("filter_field") === "id"
      && url.searchParams.get("filter_operation") === "eq"
      && url.searchParams.has("filter_value"));
    const filteredRoleIdentifiers = await page.locator("tbody tr td:first-child").allInnerTexts();
    assert.deepEqual(filteredRoleIdentifiers, [roleIdentifier]);
    await page.goto("http://127.0.0.1:8080/admin/roles");
    await page.getByRole("button", { name: "filter_name", exact: true }).click();
    await page.locator("#table-filter-name input[value=in]").check();
    await page.locator("#table-filter-name input[name=filter_value]:not([disabled])").fill(roleName);
    await page.locator("#table-filter-name form").getByRole("button", { name: "apply", exact: true }).click();
    await page.waitForURL(url => url.pathname === "/admin/roles"
      && url.searchParams.get("filter_field") === "name"
      && url.searchParams.get("filter_operation") === "in"
      && url.searchParams.has("filter_value"));
    const filteredRoleNames = await page.locator("tbody tr td:nth-child(2)").allInnerTexts();
    assert.deepEqual(filteredRoleNames, [roleName]);
    await page.goto("http://127.0.0.1:8080/admin/roles");
    await page.getByRole("button", { name: "filter_is_system", exact: true }).click();
    await page.locator("#table-filter-is_system input[value=eq]").check();
    await page.locator("#table-filter-is_system input[name=filter_value]:not([disabled])").fill(roleSystem);
    await page.locator("#table-filter-is_system form").getByRole("button", { name: "apply", exact: true }).click();
    await page.waitForURL(url => url.pathname === "/admin/roles"
      && url.searchParams.get("filter_field") === "is_system"
      && url.searchParams.get("filter_operation") === "eq"
      && url.searchParams.get("filter_value") === roleSystem);
    const filteredRoleSystems = await page.locator("tbody tr td:nth-child(3)").allInnerTexts();
    assert.ok(filteredRoleSystems.length > 0);
    assert.ok(filteredRoleSystems.every(value => value === roleSystem));
    await page.locator("header form button").click();
    await page.waitForURL("http://127.0.0.1:8080/admin/sign_in");
  } finally {
    await browser.close();
  }
});
