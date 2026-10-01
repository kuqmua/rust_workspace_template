import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

async function readFromTablePage(page, resource) {
  const path = `/${resource}/read`;
  const loaded = page.waitForResponse(value => new URL(value.url()).pathname === path && value.request().method() === "POST");
  await page.goto(`/admin/${resource}`);
  const response = await loaded;
  expect(response.status(), path).toBe(200);
  await expect(page.locator('section[data-renderer="csr"] table')).toBeVisible();
}

test("test_generated_reads_are_sent_by_table_pages", async ({ page }) => {
  await signInInitialAdministrator(page);
  try {
    await ["users", "roles", "rules", "system_settings"].reduce(async (previous, resource) => {
      await previous;
      await readFromTablePage(page, resource);

    }, Promise.resolve());
  } finally {
    await signOutIfAuthenticated(page);
  }
});

for (const resource of ["access_sessions", "role_rules"]) {
  test(`test_${resource}_page_sends_post_read`, async ({ page }) => {
    await signInInitialAdministrator(page);
    try {
      await readFromTablePage(page, resource);
    } finally {
      await signOutIfAuthenticated(page);
    }
  });
}
