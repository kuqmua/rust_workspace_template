import { expect, test } from "@playwright/test";
import { bootstrapAdministrator } from "./support/admin.js";
import { adminPages, dataTables } from "./support/pages.js";

test.skip(
  process.env.BROWSER_ACCEPTANCE_SCREENSHOTS !== "1",
  "documentation screenshots are generated explicitly"
);

test("capture administrator documentation screenshots", async ({ page }) => {
  await page.goto("/admin/sign_in");
  await page.screenshot({
    fullPage: true,
    path: "../docs/images/admin/sign-in.png"
  });
  await bootstrapAdministrator(page);

  for (const [name, path] of [
    ["users", adminPages.users.path],
    ["roles", adminPages.roles.path],
    ["settings", adminPages.settings.path],
    ["sessions", adminPages.sessions.path],
    ["data-table", dataTables.audit_log.path]
  ]) {
    await page.goto(path);
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    await page.screenshot({
      fullPage: true,
      path: `../docs/images/admin/${name}.png`
    });
  }
});
