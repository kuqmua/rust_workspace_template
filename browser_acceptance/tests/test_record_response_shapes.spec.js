import { expect, test } from "@playwright/test";
import { signInInitialAdministrator, signOutIfAuthenticated } from "./support/admin.js";

[
  ["empty_rows", ["first", "second"], [], [], []],
  ["first_row_only", ["first", "second"], [["<em>first value</em>", "second value"], ["ignored first", "ignored second"]], ["first", "second"], ["<em>first value</em>", "second value"]],
  ["extra_columns", ["first", "second"], [["only value"]], ["first"], ["only value"]],
  ["extra_values", ["first"], [["only value", "ignored value"]], ["first"], ["only value"]],
  ["empty_columns", [], [["ignored value"]], [], []],
  ["empty_values", ["first", "second"], [[]], [], []],
].forEach(([name, names, rows, labels, values]) => {
  test(`test_record_${name}_renders_exact_field_pairs`, async ({ page }) => {
    await signInInitialAdministrator(page);
    const intercept = route => route.fulfill({
      status: 200,
      contentType: "application/json",
      body: JSON.stringify({
        table: "refresh_tokens",
        columns: names.map(name => ({ name, label: `Unused label ${name}`, filters: [], input_kind: "text" })),
        items: rows.map(values => ({ values })),
        total: rows.length,
      }),
    });
    await page.route("**/refresh_tokens/read?*", intercept);
    try {
      await page.goto("/admin/refresh_tokens/11111111-1111-4111-8111-111111111111");
      const detail = page.locator('[data-page="refresh-token-read"]');
      await expect(detail).toBeAttached();
      if (rows.length === 0 || values.length > 0) await expect(detail).toBeVisible();
      else await expect(detail).not.toBeVisible();
      await expect(page.locator(".loading-state")).toHaveCount(0);
      await expect(detail.locator(".health-label")).toHaveText(labels);
      await expect(detail.locator(".health-result")).toHaveText(values);
      await expect(detail.getByText("resource not found", { exact: true })).toHaveCount(rows.length === 0 ? 1 : 0);
      await expect(detail.locator("em")).toHaveCount(0);
      await expect(detail).not.toContainText("ignored");
      await expect(detail).not.toContainText("Unused label");
    } finally {
      await page.unroute("**/refresh_tokens/read?*", intercept);
      await signOutIfAuthenticated(page);
    }
  });
});
