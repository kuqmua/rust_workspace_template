import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test.skip(process.env.BROWSER_ACCEPTANCE_SWAGGER_ENABLED !== "true", "requires the Swagger-enabled server launched by npm run test:swagger");

test("test_enabled_swagger_page_displays_openapi_document_after_reload", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  const response = await page.goto("/admin/swagger_ui");
  expect(response.status()).toBe(200);
  await expect(page.locator('nav[aria-label="admin_sections"] a[href="/admin/swagger_ui"]')).toHaveAttribute("aria-current", "page");
  const document = JSON.parse(await page.locator(".open-api-page pre").innerText());
  expect(document.openapi).toMatch(/^3\./);
  expect(Object.keys(document.paths).length).toBeGreaterThan(0);
  expect(document.components.schemas).toBeTruthy();
  await page.reload();
  expect(JSON.parse(await page.locator(".open-api-page pre").innerText())).toEqual(document);
});

test("test_enabled_swagger_page_requires_authentication", async ({ page }) => {
  await page.goto("/admin/swagger_ui");
  await expect(page).toHaveURL(/\/admin\/sign_in$/);
  await expect(page.getByRole("button", { name: "sign_in", exact: true })).toBeVisible();
});
