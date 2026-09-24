import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset, signOutIfAuthenticated } from "./support/admin.js";

test("test_rules_page_columns_match_read_api_without_extra_fields", async ({ page }) => {
  await signInAdministratorWithPasswordReset(page);
  try {
    const loaded = page.waitForResponse(response =>
      new URL(response.url()).pathname === "/rules/read"
        && response.request().method() === "POST"
    );
    await page.goto("/admin/rules");
    const response = await loaded;
    expect(response.status()).toBe(200);
    const view = await response.json();
    const columns = view.columns.map(column => column.name);
    const table = page.locator('section[data-renderer="csr"] table');
    await expect(table.locator("thead th")).toHaveCount(columns.length + 1);
    expect(await table.locator("thead th .table-column-heading > span").allTextContents())
      .toEqual(columns);
    expect(columns).not.toContain("name");
    expect(columns).not.toContain("updated_at");
    expect(columns).not.toContain("actions");
    expect(view.items.every(item => item.values.length === columns.length)).toBe(true);
    await expect(table.locator("tbody tr").first().locator("td")).toHaveCount(columns.length + 1);
    await expect(table.locator('td[data-label="actions"]')).toHaveCount(view.items.length);
    await expect(table.locator("tbody tr").first().getByRole("button", { name: "read", exact: true }))
      .toBeVisible();
  } finally {
    await signOutIfAuthenticated(page);
  }
});
