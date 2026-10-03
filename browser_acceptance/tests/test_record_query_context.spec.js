import { expect, test } from "@playwright/test";
import { signInAdministratorWithPasswordReset } from "./support/admin.js";

[
  ["users", "user-read"],
  ["roles", "role-read"],
  ["rules", "rule-read"],
  ["permission_actions", "permission-action-read"],
  ["permission_resource_actions", "permission-resource-action-read"],
  ["permission_resources", "permission-resource-read"],
  ["user_roles", "user-role-read"],
  ["role_rules", "role-rule-read"],
  ["refresh_tokens", "refresh-token-read"],
  ["access_sessions", "access-session-read"],
  ["login_attempts", "login-attempt-read"],
  ["audit_log", "audit_log-read"],
  ["system_settings", "system_settings-read"],
  ["rate_limits", "rate-limit-read"],
  ["cleanup_status", "cleanup-status-read"]
].forEach(([table, detailPage]) => {
  test(`test_${table}_details_ignore_list_query_parameters`, async ({ page }) => {
    await signInAdministratorWithPasswordReset(page);
    await page.goto(`/admin/${table}`);
    const row = page.locator("tbody tr").first();
    await expect(row).toBeVisible();
    const identifier = (await row.locator("td").first().textContent()).trim();
    const path = `/admin/${table}/${identifier}`;
    await page.goto(path);
    const values = page.locator(`[data-page="${detailPage}"] .health-result`);
    await expect(values.first()).toBeVisible();
    const expected = await values.allTextContents();
    const query = new URLSearchParams({ search: "x".repeat(129), sort: "x".repeat(33), filter_value: "x".repeat(4097) });
    query.append("limit", "0");
    query.append("limit", "1");
    query.append("limit", "2");
    query.append("offset", "999");
    query.append("offset", "-1");
    query.append("offset", "0");
    query.append("offset", "1");
    const response = await page.goto(`${path}?${query}`);
    expect(response.status()).toBe(200);
    await expect(values).toHaveText(expected);
  });
});

["profile", "health", "branding", "settings"].forEach(name => {
  test(`test_${name}_ignores_table_pagination_parameters`, async ({ page }) => {
    await signInAdministratorWithPasswordReset(page);
    await page.goto(`/admin/${name}?limit=0&offset=-1&limit=invalid`);
    await expect(page.locator('[data-renderer="csr"]')).toBeVisible();
    await expect(page.getByText("the_table_query_is_invalid", { exact: true })).toHaveCount(0);
  });
});
