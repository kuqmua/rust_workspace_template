import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

test("test_generated_table_reads_render_populated_rows", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  await ["role_rules", "access_sessions"].reduce(async (previous, resource) => {
    await previous;
    const response = page.waitForResponse(value => new URL(value.url()).pathname === `/${resource}/read` && value.request().method() === "POST");
    await page.goto(`/admin/${resource}`);
    const read = await response;
    expect(read.status(), await read.text()).toBe(200);
    const body = await read.json();
    expect(body.table).toBe(resource);
    expect(body.items.length).toBeGreaterThan(0);
    if (resource === "role_rules") {
      expect(body.columns.map(column => column.name)).toEqual(["id", "role_id", "rule_id", "created_at"]);
    }
    const firstRow = page.locator("tbody tr").first();
    await expect(firstRow.locator('td:not([data-label="actions"])')).toHaveText(body.items[0].values);
    await expect(firstRow.locator('td[data-label="actions"] button[command="show-modal"]')).toHaveAttribute("aria-label", "read");
    await expect(page.locator("tbody tr")).toHaveCount(body.items.length);
  }, Promise.resolve());
});
