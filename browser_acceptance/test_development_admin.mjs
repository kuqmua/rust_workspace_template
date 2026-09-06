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
  const documentation = await readFile(new URL("../DEVELOPMENT_ADMIN.md", import.meta.url), "utf8");
  const login = documentation.match(/^- Login: `([^`]+)`$/m)?.[1];
  const password = documentation.match(/^- Password: `([^`]+)`$/m)?.[1];
  assert.ok(login, "The development administrator login must be documented");
  assert.ok(password, "The development administrator password must be documented");
  const browser = await chromium.launch();
  try {
    const page = await browser.newPage();
    await page.goto("http://127.0.0.1:8080/admin/sign_in");
    await page.getByLabel("Login", { exact: true }).fill(login);
    await page.getByLabel("Password", { exact: true }).fill(password);
    await page.getByRole("button", { name: "sign_in", exact: true }).click();
    await page.waitForURL("http://127.0.0.1:8080/admin/users");
    await page.getByRole("link", { name: "create_user", exact: true }).waitFor();
    await page.getByRole("link", { name: "manage_users", exact: true }).waitFor();
    await page.locator("header form button").click();
    await page.waitForURL("http://127.0.0.1:8080/admin/sign_in");
  } finally {
    await browser.close();
  }
});
