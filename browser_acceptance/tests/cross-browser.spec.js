import { expect, test } from "@playwright/test";
import {
  observeBrowserErrors,
  signInInitialAdministrator
} from "./support/admin.js";
import { primaryAdminPaths } from "./support/pages.js";

test.describe.configure({ mode: "serial" });
test.skip(
  process.env.BROWSER_ACCEPTANCE_CROSS_BROWSER !== "1",
  "cross-browser smoke runs on the scheduled matrix"
);

test("administrator shell works across production browser engines", async ({ page }) => {
  const { consoleErrors, pageErrors } = observeBrowserErrors(page);

  await signInInitialAdministrator(page);

  for (const path of primaryAdminPaths) {
    const response = await page.goto(path);
    expect(response).not.toBeNull();
    expect(response.status()).toBe(200);
    await expect(page.locator("main")).toBeVisible();
    await expect(page.locator("nav[aria-label='admin_sections']")).toBeVisible();
  }

  expect(consoleErrors).toEqual([]);
  expect(pageErrors).toEqual([]);
});
