import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_generated_table_reads_render_populated_rows", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await ["role_permissions", "access_sessions"].reduce(async (previous, resource) => {
    await previous;
    const response = page.waitForResponse(value => new URL(value.url()).pathname === `/${resource}/read` && value.request().method() === "POST");
    await page.goto(`/admin/${resource}`);
    const read = await response;
    expect(read.status(), await read.text()).toBe(200);
    const body = await read.json();
    expect(body.table).toBe(resource);
    expect(body.items.length).toBeGreaterThan(0);
    if (resource === "role_permissions") {
      expect(body.columns.map(column => column.name)).toEqual(["id", "role_id", "permission_id", "created_at"]);
    }
    await expect(page.locator("tbody tr").first().locator("td")).toHaveText([...body.items[0].values, ""]);
    await expect(page.locator("tbody tr")).toHaveCount(body.items.length);
  }, Promise.resolve());
});
